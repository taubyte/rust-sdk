use std::io::Read;

use crate::{errno::Errno, storage::Content};

use super::{imports, ReadOnlyContent, ReadWriteContent};

impl Read for Content {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.consumed {
            self.consumed = false;
            return Ok(0);
        }

        let mut count: usize = 0;
        let err0 =
            unsafe { imports::contentReadFile(self.id, buf.as_mut_ptr(), buf.len(), &mut count) };
        if err0.is_errno(Errno::ErrorEOF) {
            self.consumed = true;
            // TODO, should we close here?
            Ok(count)
        } else if err0.is_err() {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Reading content failed with: {}", err0),
            ))
        } else {
            Ok(count)
        }
    }
}

impl Read for ReadWriteContent {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.content.read(buf)
    }
}

impl Read for ReadOnlyContent {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.content.read(buf)
    }
}

#[cfg(test)]
pub mod test {
    use std::io::Read;

    use crate::storage::Content;

    pub static EXPECTED_READ: &str = "Hello, world!";

    #[test]
    fn read() {
        let mut content = Content::new().unwrap_or_else(|err| {
            panic!("{}", err);
        });

        let mut buffer = String::new();
        content.read_to_string(&mut buffer).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(buffer, EXPECTED_READ);
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
pub mod mock {
    use super::test;
    use crate::{
        errno::{Errno, Error},
        storage::content::new::test as new_test,
    };

    #[cfg(test)]
    pub unsafe fn contentReadFile(
        id: u32,
        buf_ptr: *const u8,
        buf_len: usize,
        count_ptr: *mut usize,
    ) -> Error {
        if id != new_test::ID {
            Errno::ErrorCap.error()
        } else {
            let body = test::EXPECTED_READ.as_bytes();

            let buf = std::slice::from_raw_parts_mut(buf_ptr as *mut u8, body.len() as usize);
            buf.copy_from_slice(body);
            *count_ptr = body.len();

            if buf_len >= body.len() {
                Errno::ErrorEOF.error()
            } else {
                Errno::ErrorNone.error()
            }
        }
    }
}
