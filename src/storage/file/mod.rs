mod add;
mod close;
mod debug;
mod delete;
mod get;
mod new;
mod read;
mod version;
mod versions;

pub struct File {
    pub storage_id: u32,
    pub name: String,
    version: u32,
}

pub struct VersionedFile {
    pub file: File,
}

pub struct FileReader {
    storage_id: u32,
    fd: u32,
    pub consumed: bool,
}

#[warn(unused_imports)]
#[cfg(not(test))]
mod imports;

#[cfg(test)]
mod imports {
    pub use super::add::mock::*;
    pub use super::close::mock::*;
    pub use super::delete::mock::*;
    pub use super::get::mock::*;
    pub use super::read::mock::*;
    pub use super::version::mock::*;
    pub use super::versions::mock::*;
}
