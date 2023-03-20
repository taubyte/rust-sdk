use crate::errno::Error;

use super::{imports, File, FileReader, VersionedFile};

impl File {
    fn get_unsafe(&self, fd: *mut u32) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::storageGetFile(
                self.storage_id,
                self.name.as_ptr(),
                self.name.len(),
                self.version,
                fd,
            )
        }
    }

    pub fn get(&self) -> Result<FileReader, Box<dyn std::error::Error>> {
        let mut fd: u32 = 0;
        let err0 = self.get_unsafe(&mut fd);
        if err0.is_err() {
            Err(format!("Getting storage file failed with: {}", err0).into())
        } else {
            Ok(FileReader {
                storage_id: self.storage_id,
                fd: fd,
                consumed: false,
            })
        }
    }
}

impl VersionedFile {
    pub fn get(&self) -> Result<FileReader, Box<dyn std::error::Error>> {
        self.file.get()
    }
}

#[cfg(test)]
pub mod test {
    use crate::storage::new::test as new_test;
    use crate::storage::Storage;

    pub static FILE_NAME: &str = "test_file";
    pub static MOCK_FD: u32 = 16;
    pub static VERSION: u32 = 4;

    #[test]
    fn test_get() {
        let storage = Storage::new(new_test::STORAGE_NAME).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        let file = storage.file(FILE_NAME).as_versioned(VERSION);
        let file_reader = file.get().unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(file_reader.fd, MOCK_FD);
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
pub mod mock {
    use super::test;
    use crate::{
        errno::{Errno, Error},
        storage::new::test as new_test,
        utils::test as utils,
    };

    pub fn storageGetFile(
        storage_id: u32,
        name_ptr: *const u8,
        name_size: usize,
        version: u32,
        fd: *mut u32,
    ) -> Error {
        let name = utils::read_string(name_ptr, name_size);

        if storage_id != new_test::STORAGE_ID {
            Errno::ErrorCap.error()
        } else if name != test::FILE_NAME {
            Errno::ErrorCap.error()
        } else if version != test::VERSION {
            Errno::ErrorCap.error()
        } else {
            utils::write_u32(fd, test::MOCK_FD);
            Errno::ErrorNone.error()
        }
    }
}
