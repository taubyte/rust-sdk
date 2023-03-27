use crate::{errno::Error, utils::codec};

use super::{imports, File, VersionedFile};

impl File {
    fn versions_size_unsafe(&self, versions: *mut usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::storageListVersionsSize(
                self.storage_id,
                self.name.as_ptr(),
                self.name.len(),
                versions,
            )
        }
    }

    fn versions_unsafe(&self, versions: &mut [u8]) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::storageListVersions(
                self.storage_id,
                self.name.as_ptr(),
                self.name.len(),
                versions.as_mut_ptr(),
            )
        }
    }

    pub fn versions(&self) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
        let mut versions_size: usize = 0;
        let err0 = self.versions_size_unsafe(&mut versions_size);
        if err0.is_err() {
            return Err(format!("Getting versions size failed with: {}", err0).into());
        }

        let mut versions: Vec<u8> = vec![0; versions_size];
        let err0 = self.versions_unsafe(&mut versions);
        if err0.is_err() {
            return Err(format!("Getting versions failed with: {}", err0).into());
        }

        let string_versions = codec::string_slice::to(versions);

        Ok(string_versions
            .iter()
            .map(|v| v.parse())
            .collect::<Result<Vec<u32>, std::num::ParseIntError>>()?)
    }
}

impl VersionedFile {
    pub fn versions(&self) -> Result<Vec<u32>, Box<dyn std::error::Error>> {
        self.file.versions()
    }
}

#[cfg(test)]
pub mod test {
    use crate::storage::file::get::test as get_test;
    use crate::storage::new::test as new_test;
    use crate::storage::Storage;

    pub static VERSIONS: &[u32] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    #[test]
    fn versions() {
        let storage = Storage::new(new_test::STORAGE_NAME).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        let file = storage.file(get_test::FILE_NAME);
        let versions = file.versions().unwrap();

        assert_eq!(versions, VERSIONS.to_vec());
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
        utils::{codec, test as utils},
    };

    pub fn storageListVersionsSize(
        storage_id: u32,
        name_ptr: *const u8,
        name_size: usize,
        versions_size: *mut usize,
    ) -> Error {
        let name = utils::read_string(name_ptr, name_size);

        let to_write_versions: Vec<String> = test::VERSIONS.iter().map(|v| v.to_string()).collect();
        let to_write = codec::string_slice::from(to_write_versions);

        if storage_id != new_test::STORAGE_ID {
            Errno::ErrorCap.error()
        } else if name != get_test::FILE_NAME {
            Errno::ErrorCap.error()
        } else {
            utils::write_usize(versions_size, to_write.len());
            Errno::ErrorNone.error()
        }
    }

    pub fn storageListVersions(
        storage_id: u32,
        name_ptr: *const u8,
        name_size: usize,
        versions: *mut u8,
    ) -> Error {
        let name = utils::read_string(name_ptr, name_size);

        let to_write_versions: Vec<String> = test::VERSIONS.iter().map(|v| v.to_string()).collect();
        let to_write = codec::string_slice::from(to_write_versions);

        if storage_id != new_test::STORAGE_ID {
            Errno::ErrorCap.error()
        } else if name != get_test::FILE_NAME {
            Errno::ErrorCap.error()
        } else {
            utils::write_bytes_vec(versions, to_write);
            Errno::ErrorNone.error()
        }
    }
}
