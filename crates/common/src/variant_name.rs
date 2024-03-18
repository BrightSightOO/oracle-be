pub trait VariantName {
    /// Returns the name of the enum variant.
    fn variant_name(&self) -> &'static str;
}
