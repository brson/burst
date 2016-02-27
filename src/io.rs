use bpal;

pub type Result<T> = ::core::result::Result<T, Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
}

#[derive(Debug)]
pub struct Error(bpal::io::Error);

pub mod stdio {
    use core;
    use bpal;
    use super::{Write, Result, Error};

    pub fn stderr() -> Stderr {
        Stderr(bpal::io::stdio::stderr())
    }

    // FIXME: This should be arced and mutexed
    pub struct Stderr(bpal::io::stdio::Stderr);

    impl Write for Stderr {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            bpal::io::Write::write(&mut self.0, buf).map_err(Error)
        }
    }

    impl core::fmt::Write  for Stderr {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            if self.write(s.as_bytes()).is_ok() {
                Ok(())
            } else {
                Err(core::fmt::Error)
            }
        }
    }
}
