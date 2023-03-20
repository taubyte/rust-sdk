use super::{File, VersionedFile};
impl std::fmt::Debug for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("File")
            .field("storage_id", &self.storage_id)
            .field("name", &self.name)
            .field("version", &self.version)
            .finish()
    }
}

impl std::fmt::Debug for VersionedFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.file.fmt(f)
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.storage_id == other.storage_id
            && self.name == other.name
            && self.version == other.version
    }
}

impl PartialEq for VersionedFile {
    fn eq(&self, other: &Self) -> bool {
        self.file.eq(&other.file)
    }
}
