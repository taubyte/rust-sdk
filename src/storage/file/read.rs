use crate::errno::Errno;

use super::{imports, FileReader};

impl std::io::Read for FileReader {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.consumed {
            return Ok(0);
        }

        let mut n: u32 = 0;

        #[allow(unused_unsafe)]
        let err0 = unsafe {
            imports::storageReadFile(
                self.storage_id,
                self.fd,
                buf.as_mut_ptr(),
                buf.len() as usize,
                &mut n,
            )
        };
        if err0.is_errno(Errno::ErrorEOF) {
            self.consumed = true;
            // TODO, should we close here?
            return Ok(n as usize);
        }
        if err0.ok() {
            Ok(n as usize)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("reading body failed with: {}", err0),
            ))
        }
    }
}

#[cfg(test)]
pub mod test {
    use std::io::Read;

    use crate::storage::file::get::test as get_test;
    use crate::storage::new::test as new_test;
    use crate::storage::Storage;

    pub static MOCK_DATA: &str = "Hello, world!";

    #[test]
    fn test_read() {
        let storage = Storage::new(new_test::STORAGE_NAME).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        let mut reader = storage
            .file(get_test::FILE_NAME)
            .as_versioned(get_test::VERSION)
            .get()
            .unwrap_or_else(|err| {
                panic!("{}", err);
            });

        let mut buffer = String::new();
        reader.read_to_string(&mut buffer).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(buffer, MOCK_DATA);
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
pub mod mock {
    use super::test;
    use crate::{
        errno::{Errno, Error},
        storage::file::get::test as get_test,
        storage::new::test as new_test,
        utils::test as utils,
    };

    pub fn storageReadFile(
        storage_id: u32,
        fd: u32,
        data_ptr: *mut u8,
        data_size: usize,
        count: *mut u32,
    ) -> Error {
        let body = test::MOCK_DATA.as_bytes();

        if storage_id != new_test::STORAGE_ID {
            Errno::ErrorCap.error()
        } else if fd != get_test::MOCK_FD {
            Errno::ErrorCap.error()
        } else {
            utils::write_bytes(data_ptr, body);
            utils::write_u32(count, test::MOCK_DATA.len() as u32);
            if data_size >= body.len() {
                Errno::ErrorEOF.error()
            } else {
                Errno::ErrorNone.error()
            }
        }
    }
}
