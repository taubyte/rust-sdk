use crate::errno::Error;

#[link(wasm_import_module = "taubyte/sdk")]
extern "C" {
    pub fn storageAddFile(
        storage_id: u32,
        name_ptr: *const u8,
        name_size: usize,
        version: *mut u32,
        data: *mut u8,
        data_size: usize,
        overwrite: u8,
    ) -> Error;
    pub fn storageGetFile(
        storage_id: u32,
        name_ptr: *const u8,
        name_size: usize,
        version: u32,
        fd: *mut u32,
    ) -> Error;
    pub fn storageReadFile(
        storage_id: u32,
        fd: u32,
        data_ptr: *mut u8,
        data_size: usize,
        count: *mut u32,
    ) -> Error;
    pub fn storageCloseFile(storage_id: u32, fd: u32) -> Error;
    pub fn storageDeleteFile(
        storage_id: u32,
        name_ptr: *const u8,
        name_size: usize,
        version: u32,
        all: u8,
    ) -> Error;
    pub fn storageListVersionsSize(
        storage_id: u32,
        name_ptr: *const u8,
        name_size: usize,
        size: *mut usize,
    ) -> Error;
    pub fn storageListVersions(
        storage_id: u32,
        name_ptr: *const u8,
        name_size: usize,
        versions: *mut u8,
    ) -> Error;
    pub fn storageCurrentVersionSize(
        storage_id: u32,
        name_ptr: *const u8,
        name_size: usize,
        version_size: *mut usize,
    ) -> Error;

    // TODO, not passing storage_id here, need to review host function as it seems to be being cached between size and this call.
    pub fn storageCurrentVersion(name_ptr: *const u8, name_size: usize, version: *mut u8) -> Error;
}
