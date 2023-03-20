use super::{File, VersionedFile};

impl File {
    pub fn new(storage_id: u32, name: &str) -> Self {
        File {
            storage_id,
            name: String::from(name),
            version: 0,
        }
    }

    fn new_versioned(storage_id: u32, name: &str, version: u32) -> Self {
        File {
            storage_id,
            name: String::from(name),
            version: version,
        }
    }

    fn get_version(&self) -> u32 {
        self.version as u32
    }
}

impl VersionedFile {
    pub fn new(file: File, version: u32) -> Self {
        VersionedFile {
            file: File::new_versioned(file.storage_id, &file.name, version),
        }
    }

    pub fn version(&self) -> u32 {
        self.file.get_version()
    }
}
