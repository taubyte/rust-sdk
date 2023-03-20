use crate::errno::Error;

use super::{imports, File, VersionedFile};

impl File {
    fn delete_unsafe(&self, all: u8) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::storageDeleteFile(
                self.storage_id,
                self.name.as_ptr(),
                self.name.len(),
                self.version,
                all,
            )
        }
    }

    pub fn delete(&self) -> Result<(), Box<dyn std::error::Error>> {
        let err0 = self.delete_unsafe(0);
        if err0.is_err() {
            Err(format!("Deleting file failed with: {}", err0).into())
        } else {
            Ok(())
        }
    }

    pub fn delete_all_versions(&self) -> Result<(), Box<dyn std::error::Error>> {
        let err0 = self.delete_unsafe(1);
        if err0.is_err() {
            Err(format!("Deleting all versions failed with: {}", err0).into())
        } else {
            Ok(())
        }
    }
}

impl VersionedFile {
    pub fn delete(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.file.delete()
    }

    pub fn delete_all_versions(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.file.delete_all_versions()
    }
}

#[cfg(test)]
pub mod test {
    pub static ALL: bool = true;
    use crate::{
        storage::file::get::test as get_test,
        storage::{new::test as new_test, Storage},
    };

    #[test]
    fn delete() {
        let storage = Storage::new(new_test::STORAGE_NAME).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        let file = storage
            .file(get_test::FILE_NAME)
            .as_versioned(get_test::VERSION);

        file.delete_all_versions().unwrap_or_else(|err| {
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
        storage::file::get::test as get_test,
        storage::new::test as new_test,
        utils::test as utils,
    };

    pub fn storageDeleteFile(
        storage_id: u32,
        name_ptr: *const u8,
        name_size: usize,
        version: u32,
        all: u8,
    ) -> Error {
        let name = utils::read_string(name_ptr, name_size);

        if storage_id != new_test::STORAGE_ID {
            Errno::ErrorCap.error()
        } else if name != get_test::FILE_NAME {
            Errno::ErrorCap.error()
        } else if version != get_test::VERSION {
            Errno::ErrorCap.error()
        } else if all != test::ALL as u8 {
            Errno::ErrorCap.error()
        } else {
            Errno::ErrorNone.error()
        }
    }
}
