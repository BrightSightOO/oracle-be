use std::io;

use borsh::BorshSerialize;

pub fn serialized_len<T: BorshSerialize>(value: &T) -> io::Result<usize> {
    struct LenWriter {
        len: usize,
    }

    impl LenWriter {
        #[inline]
        fn add_written_bytes(&mut self, written: usize) {
            #[cold]
            #[inline(never)]
            fn len_writer_overflow() -> ! {
                panic!("length of written bytes overflowed usize");
            }

            self.len = match self.len.checked_add(written) {
                Some(len) => len,
                None => len_writer_overflow(),
            };
        }
    }

    impl io::Write for LenWriter {
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }

        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            let written = buf.len();
            self.add_written_bytes(written);
            Ok(written)
        }

        fn write_all(&mut self, buf: &[u8]) -> io::Result<()> {
            self.add_written_bytes(buf.len());
            Ok(())
        }
    }

    let mut w = LenWriter { len: 0 };
    value.serialize(&mut w)?;
    Ok(w.len)
}
