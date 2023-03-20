use super::file::{File, VersionedFile};
use super::{imports, Storage};
use crate::errno::Error;

impl Storage {
    pub fn file(&self, file_name: &str) -> File {
        File::new(self.id, file_name)
    }

    fn file_versioned(&self, file_name: &str, version: u32) -> VersionedFile {
        VersionedFile::new(File::new(self.id, file_name), version)
    }

    fn list_files_size_unsafe(&self, size: *mut usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::storageListFilesSize(self.id, size)
        }
    }

    fn list_files_unsafe(&self, files: *mut u8) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::storageListFiles(self.id, files)
        }
    }

    pub fn list_files(&self) -> Result<Vec<VersionedFile>, Box<dyn std::error::Error>> {
        let mut size: usize = 0;
        let err = self.list_files_size_unsafe(&mut size);
        if err.is_err() {
            return Err(format!("Failed storage list file size with {}", err).into());
        }

        let mut buf = vec![0_u8; size];
        let err = self.list_files_unsafe(&mut buf[0]);
        if err.is_err() {
            return Err(format!("Failed storage list files with {}", err).into());
        }

        let mut files: Vec<VersionedFile> = Vec::new();

        let values = String::from_utf8(buf)?;
        let values_split = values.split('/');
        let values: Vec<&str> = values_split.collect();
        for (idx, value) in values.iter().enumerate() {
            if *value == "file" && values[idx + 1] != "file" {
                let version = values[idx + 2].trim_matches('\x00');
                let version = version.parse::<u32>()?;
                files.push(self.file_versioned(values[idx + 1], version));
            }
            continue;
        }

        Ok(files)
    }
}

#[cfg(test)]
pub mod test {
    use crate::storage::file::{File, VersionedFile};
    use crate::storage::new::test as new_test;
    use crate::storage::Storage;

    pub static FILE_STR: &str = "file/file1/1/file/file2/1/file/file3/2";

    #[test]
    fn list_files() {
        let storage = Storage::new(new_test::STORAGE_NAME).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        let files = storage.list_files().unwrap_or_else(|err| {
            panic!("{}", err);
        });

        let expected_files = vec![
            VersionedFile::new(File::new(new_test::STORAGE_ID, "file1"), 1),
            VersionedFile::new(File::new(new_test::STORAGE_ID, "file2"), 1),
            VersionedFile::new(File::new(new_test::STORAGE_ID, "file3"), 2),
        ];

        assert_eq!(files, expected_files);
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

    pub fn storageListFilesSize(storage_id: u32, size: *mut usize) -> Error {
        use super::test;

        if storage_id != new_test::STORAGE_ID {
            Errno::ErrorCap.error()
        } else {
            utils::write_usize(size, test::FILE_STR.len());
            Errno::ErrorNone.error()
        }
    }

    pub fn storageListFiles(storage_id: u32, files: *mut u8) -> Error {
        use super::test;

        if storage_id != new_test::STORAGE_ID {
            Errno::ErrorCap.error()
        } else {
            utils::write_string(files, test::FILE_STR);
            Errno::ErrorNone.error()
        }
    }
}
