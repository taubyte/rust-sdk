use crate::errno::Error;

use super::{File, VersionedFile};

impl File {
    fn latest_version_size_unsafe(&self, version: &mut usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            crate::storage::file::imports::storageCurrentVersionSize(
                self.storage_id,
                self.name.as_ptr(),
                self.name.len(),
                version,
            )
        }
    }

    fn latest_version_unsafe(&self, version: *mut u8) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            crate::storage::file::imports::storageCurrentVersion(
                self.name.as_ptr(),
                self.name.len(),
                version,
            )
        }
    }

    pub fn latest_version(&self) -> Result<u32, Box<dyn std::error::Error>> {
        let mut version_size: usize = 0;
        let err0 = self.latest_version_size_unsafe(&mut version_size);
        if err0.is_err() {
            return Err(format!("Getting latest version size failed with: {}", err0).into());
        }

        let mut version: Vec<u8> = vec![0; version_size];
        let err0 = self.latest_version_unsafe(version.as_mut_ptr());
        if err0.is_err() {
            Err(format!("Getting latest version failed with: {}", err0).into())
        } else {
            Ok(String::from_utf8(version)?.parse()?)
        }
    }

    pub fn as_versioned(&self, version: u32) -> VersionedFile {
        VersionedFile {
            file: File {
                storage_id: self.storage_id,
                name: self.name.to_string(),
                version: version,
            },
        }
    }
}

impl VersionedFile {
    pub fn latest_version(&self) -> Result<u32, Box<dyn std::error::Error>> {
        self.file.latest_version()
    }
}

#[cfg(test)]
pub mod test {
    use crate::storage::new::test as new_test;
    use crate::storage::Storage;

    pub static FILE_NAME: &str = "test_file";
    pub static CURRENT_VERSION: u32 = 3;

    #[test]
    fn test_latest() {
        let storage = Storage::new(new_test::STORAGE_NAME).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        let file = storage.file(FILE_NAME);
        let version = file.latest_version().unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(version, CURRENT_VERSION);
    }

    #[test]
    fn test_latest_versioned() {
        let storage = Storage::new(new_test::STORAGE_NAME).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        let file = storage.file(FILE_NAME).as_versioned(1);
        let version = file.latest_version().unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(version, CURRENT_VERSION);
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

    pub fn storageCurrentVersionSize(
        storage_id: u32,
        name_ptr: *const u8,
        name_size: usize,
        version_size: *mut usize,
    ) -> Error {
        let filename = utils::read_string(name_ptr, name_size);

        if storage_id != new_test::STORAGE_ID {
            Errno::ErrorCap.error()
        } else if filename != test::FILE_NAME {
            Errno::ErrorCap.error()
        } else {
            utils::write_usize(version_size, test::CURRENT_VERSION.to_string().len());
            Errno::ErrorNone.error()
        }
    }

    pub fn storageCurrentVersion(name_ptr: *const u8, name_size: usize, version: *mut u8) -> Error {
        let filename = utils::read_string(name_ptr, name_size);

        if filename != test::FILE_NAME {
            Errno::ErrorCap.error()
        } else {
            utils::write_bytes(version, test::CURRENT_VERSION.to_string().as_bytes());
            Errno::ErrorNone.error()
        }
    }
}
