use crate::errno::Error;

#[link(wasm_import_module = "taubyte/sdk")]
extern "C" {
    pub fn storageNew(name_ptr: *const u8, name_size: usize, id: *mut u32) -> Error;
    pub fn storageGet(name_ptr: *const u8, name_size: usize, id: *mut u32) -> Error;
    pub fn storageUsedSize(id: u32, size: *mut usize) -> Error;
    pub fn storageUsed(id: u32, used: *mut u8) -> Error;
    pub fn storageCapacitySize(id: u32, size: *mut usize) -> Error;
    pub fn storageCapacity(id: u32, capacity: *mut u8) -> Error;
    pub fn storageCid(id: u32, name_ptr: *const u8, name_size: usize, cid: *mut u8) -> Error;
    pub fn storageListFilesSize(id: u32, size: *mut usize) -> Error;
    pub fn storageListFiles(id: u32, files: *mut u8) -> Error;
}
