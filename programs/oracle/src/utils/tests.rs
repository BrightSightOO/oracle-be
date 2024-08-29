use std::num::NonZeroUsize;

use borsh::schema::{BorshSchemaContainer, Declaration, Definition, Fields};
use borsh::BorshSchema;
use borsh_size::BorshSize;

#[track_caller]
pub fn validate_schema<T: BorshSchema + ?Sized>() {
    if let Err(err) = BorshSchemaContainer::for_type::<T>().validate() {
        panic!("invalid schema: {err:?}");
    }
}

#[track_caller]
pub fn validate_max_size<T: BorshSize + BorshSchema + ?Sized>() {
    use borsh::schema::SchemaMaxSerializedSizeError as Error;

    const ONE: NonZeroUsize = match NonZeroUsize::new(1) {
        Some(one) => one,
        None => unreachable!(),
    };

    fn max_size<'a>(
        count: NonZeroUsize,
        declaration: &'a str,
        schema: &'a BorshSchemaContainer,
        stack: &mut Vec<&'a str>,
    ) -> Result<usize, Error> {
        fn add(x: usize, y: usize) -> Result<usize, Error> {
            x.checked_add(y).ok_or(Error::Overflow)
        }

        fn mul(x: NonZeroUsize, y: usize) -> Result<usize, Error> {
            x.get().checked_mul(y).ok_or(Error::Overflow)
        }

        /// Calculates max serialised size of a tuple with given members.
        fn tuple<'a>(
            count: NonZeroUsize,
            elements: impl core::iter::IntoIterator<Item = &'a Declaration>,
            schema: &'a BorshSchemaContainer,
            stack: &mut Vec<&'a str>,
        ) -> Result<usize, Error> {
            let mut sum: usize = 0;
            for el in elements {
                sum = add(sum, max_size(ONE, el, schema, stack)?)?;
            }
            mul(count, sum)
        }

        if stack.iter().any(|dec| *dec == declaration) {
            return Err(Error::Recursive);
        }
        stack.push(declaration);

        let res = match schema.get_definition(declaration).ok_or(declaration) {
            Ok(Definition::Primitive(size)) => match size {
                0 => Ok(0),
                size => usize::from(*size).checked_mul(count.get()).ok_or(Error::Overflow),
            },
            Ok(Definition::Sequence { length_width, length_range, elements }) => {
                struct Unsized;

                let max_len = if *length_width == Definition::DEFAULT_LENGTH_WIDTH
                    && *length_range == Definition::DEFAULT_LENGTH_RANGE
                {
                    Err(Unsized)
                } else {
                    // Assume sequence has the maximum number of elements.
                    usize::try_from(*length_range.end()).map_err(|_| Unsized)
                };

                let sz = match max_len.map(NonZeroUsize::new) {
                    Ok(Some(max_len)) => max_size(max_len, elements, schema, stack)?,
                    Ok(None) => 0,
                    Err(_) if is_zero_size(elements, schema, stack)? => 0,
                    Err(_) => return Err(Error::Overflow),
                };

                mul(count, add(sz, usize::from(*length_width))?)
            }

            Ok(Definition::Enum { tag_width, variants }) => {
                let mut max = 0;
                for (_, _, variant) in variants {
                    let sz = max_size(ONE, variant, schema, stack)?;
                    max = max.max(sz);
                }
                add(max, usize::from(*tag_width))
            }

            // Tuples and structs sum sizes of all the members.
            Ok(Definition::Tuple { elements }) => tuple(count, elements, schema, stack),
            Ok(Definition::Struct { fields }) => match fields {
                Fields::NamedFields(fields) => {
                    tuple(count, fields.iter().map(|(_, field)| field), schema, stack)
                }
                Fields::UnnamedFields(fields) => tuple(count, fields, schema, stack),
                Fields::Empty => Ok(0),
            },

            Err(declaration) => Err(Error::MissingDefinition(declaration.to_string())),
        }?;

        stack.pop();

        Ok(res)
    }

    fn is_zero_size<'a>(
        declaration: &'a str,
        schema: &'a BorshSchemaContainer,
        stack: &mut Vec<&'a str>,
    ) -> Result<bool, Error> {
        fn all<'a, T: 'a>(
            iter: impl Iterator<Item = T>,
            f_key: impl Fn(&T) -> &'a Declaration,
            schema: &'a BorshSchemaContainer,
            stack: &mut Vec<&'a str>,
        ) -> Result<bool, Error> {
            for element in iter {
                let declaration = f_key(&element);
                if !is_zero_size(declaration.as_str(), schema, stack)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }

        if stack.iter().any(|dec| *dec == declaration) {
            return Err(Error::Recursive);
        }
        stack.push(declaration);

        let res = match schema.get_definition(declaration).ok_or(declaration) {
            Ok(Definition::Primitive(size)) => *size == 0,
            Ok(Definition::Sequence { length_width, length_range, elements }) => {
                if *length_width == 0 {
                    // zero-sized array
                    if length_range.clone().count() == 1 && *length_range.start() == 0 {
                        return Ok(true);
                    }
                    if is_zero_size(elements.as_str(), schema, stack)? {
                        return Ok(true);
                    }
                }
                false
            }
            Ok(Definition::Tuple { elements }) => all(elements.iter(), |key| *key, schema, stack)?,
            Ok(Definition::Enum { tag_width: 0, variants }) => all(
                variants.iter(),
                |(_variant_discrim, _variant_name, declaration)| declaration,
                schema,
                stack,
            )?,
            Ok(Definition::Enum { .. }) => false,
            Ok(Definition::Struct { fields }) => match fields {
                Fields::NamedFields(fields) => {
                    all(fields.iter(), |(_field_name, declaration)| declaration, schema, stack)?
                }
                Fields::UnnamedFields(fields) => {
                    all(fields.iter(), |declaration| declaration, schema, stack)?
                }
                Fields::Empty => true,
            },

            Err(declaration) => {
                return Err(Error::MissingDefinition(declaration.into()));
            }
        };

        stack.pop();

        Ok(res)
    }

    let schema = BorshSchemaContainer::for_type::<T>();

    let mut stack = Vec::new();

    let schema_max_size = match max_size(ONE, schema.declaration(), &schema, &mut stack) {
        Ok(max) => Some(max),
        Err(Error::Overflow | Error::Recursive) => None,
        Err(err) => panic!("failed to determine schema max size: {err:?}"),
    };

    assert_eq!(schema_max_size, T::MAX_SIZE);
}
