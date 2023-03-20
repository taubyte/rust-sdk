use crate::errno::Error;

use super::{imports, FileReader};

impl FileReader {
    fn close_unsafe(&self) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::storageCloseFile(self.storage_id, self.fd)
        }
    }

    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let err0 = self.close_unsafe();
        if err0.ok() {
            Ok(())
        } else {
            Err(format!("Closing storage file failed with: {}", err0).into())
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::storage::file::get::test as get_test;
    use crate::storage::new::test as new_test;
    use crate::storage::Storage;

    #[test]
    fn test_close() {
        let storage = Storage::new(new_test::STORAGE_NAME).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        let reader = storage
            .file(get_test::FILE_NAME)
            .as_versioned(get_test::VERSION)
            .get()
            .unwrap_or_else(|err| {
                panic!("{}", err);
            });

        reader.close().unwrap_or_else(|err| {
            panic!("{}", err);
        });
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
pub mod mock {
    use crate::{
        errno::{Errno, Error},
        storage::file::get::test as get_test,
        storage::new::test as new_test,
    };

    pub fn storageCloseFile(storage_id: u32, fd: u32) -> Error {
        if storage_id != new_test::STORAGE_ID {
            Errno::ErrorCap.error()
        } else if fd != get_test::MOCK_FD {
            Errno::ErrorCap.error()
        } else {
            Errno::ErrorNone.error()
        }
    }
}
