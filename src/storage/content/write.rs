use std::io::Write;

use crate::storage::Content;

use super::{imports, ReadWriteContent};

impl Write for Content {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut count: usize = 0;
        #[allow(unused_unsafe)]
        let err0 =
            unsafe { imports::contentWriteFile(self.id, buf.as_ptr(), buf.len(), &mut count) };
        if err0.is_err() {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Writing content failed with: {}", err0),
            ))
        } else {
            Ok(count)
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        // TODO implement
        Ok(())
    }
}

impl Write for ReadWriteContent {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.content.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.content.flush()
    }
}

#[cfg(test)]
pub mod test {
    use crate::storage::Content;
    use std::io::Write;

    pub static EXPECTED_WRITE: &str = "Hello, world!";

    #[test]
    fn write() {
        let mut content = Content::new().unwrap_or_else(|err| {
            panic!("{}", err);
        });

        content
            .write_all(EXPECTED_WRITE.as_bytes())
            .unwrap_or_else(|err| {
                panic!("{}", err);
            });
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
pub mod mock {
    use super::test;

    use crate::{
        errno::{Errno, Error},
        storage::content::new::test as new_test,
        utils::test as utils,
    };

    pub fn contentWriteFile(
        id: u32,
        buf_ptr: *const u8,
        buf_size: usize,
        count: *mut usize,
    ) -> Error {
        if id != new_test::ID {
            return Errno::ErrorCap.error();
        }

        unsafe {
            let result = utils::read_string(buf_ptr, buf_size);
            if result != test::EXPECTED_WRITE {
                return Errno::ErrorCap.error();
            }

            *count = buf_size;
        }

        Errno::ErrorNone.error()
    }
}
