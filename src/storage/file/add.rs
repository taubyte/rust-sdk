use crate::errno::Error;

use super::{imports, File, VersionedFile};

impl File {
    fn add_unsafe(&self, data: &[u8], overwrite: bool) -> Result<u32, Error> {
        let mut new_version: u32 = 0;

        #[allow(unused_unsafe)]
        let err = unsafe {
            imports::storageAddFile(
                self.storage_id,
                self.name.as_ptr(),
                self.name.len(),
                &mut new_version,
                data.as_ptr() as *mut u8,
                data.len(),
                overwrite as u8,
            )
        };

        if err.is_err() {
            Err(err)
        } else {
            Ok(new_version)
        }
    }

    pub fn add(&self, data: &[u8], overwrite: bool) -> Result<u32, Box<dyn std::error::Error>> {
        let err0 = self.add_unsafe(data, overwrite);
        if err0.is_err() {
            Err(format!("Adding storage file failed with: {}", err0.unwrap()).into())
        } else {
            Ok(err0.unwrap())
        }
    }
}

impl VersionedFile {
    // Returns new version or an error
    pub fn add(&mut self, data: &[u8], overwrite: bool) -> Result<u32, Box<dyn std::error::Error>> {
        self.file.version = self.file.add(data, overwrite)?;
        Ok(self.file.version)
    }
}

#[cfg(test)]
pub mod test {
    use crate::storage::new::test as new_test;
    use crate::storage::Storage;

    pub static FILE_NAME: &str = "test_file";
    pub static DATA: &[u8] = "Hello, world!".as_bytes();
    pub static OVERWRITE: bool = true;
    pub static NEW_VERSION: u32 = 6;

    #[test]
    fn test_add() {
        let storage = Storage::new(new_test::STORAGE_NAME).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        let file = storage.file(FILE_NAME);
        file.add(DATA, OVERWRITE).unwrap_or_else(|err| {
            panic!("{}", err);
        });
    }

    #[test]
    fn test_versioned_add() {
        let storage = Storage::new(new_test::STORAGE_NAME).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        let mut file = storage.file(FILE_NAME).as_versioned(1);
        let version = file.add(DATA, OVERWRITE).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(version, NEW_VERSION);
        assert_eq!(file.version(), NEW_VERSION);
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
pub mod mock {
    use crate::{
        errno::{Errno, Error},
        storage::new::test as new_test,
        utils::test as utils,
    };

    pub fn storageAddFile(
        storage_id: u32,
        name_ptr: *const u8,
        name_size: usize,
        version: *mut u32,
        data_ptr: *mut u8,
        data_size: usize,
        overwrite: u8,
    ) -> Error {
        use super::test;

        let name = utils::read_string(name_ptr, name_size);
        let data = utils::read_bytes(data_ptr, data_size);

        if storage_id != new_test::STORAGE_ID {
            Errno::ErrorCap.error()
        } else if name != test::FILE_NAME {
            Errno::ErrorCap.error()
        } else if data != test::DATA {
            Errno::ErrorCap.error()
        } else if overwrite != (test::OVERWRITE as u8) {
            Errno::ErrorCap.error()
        } else {
            utils::write_u32(version, test::NEW_VERSION);
            Errno::ErrorNone.error()
        }
    }
}
