pub type Result<T> = ::core::result::Result<T, Error>;

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize>;
}

#[derive(Debug)]
pub struct Error;

impl Error {
    pub fn last_os_error() -> Error {
        Error
    }
}

pub mod stdio {
    use super::{Write, Result};
    use fd::FileDesc;
    use libc;

    pub struct Stderr(());

    pub fn stderr() -> Stderr {
        Stderr(())
    }

    impl Write for Stderr {
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            let fd = FileDesc::new(libc::STDERR_FILENO);
            let ret = fd.write(buf);
            fd.into_raw();
            ret
        }
    }
}
