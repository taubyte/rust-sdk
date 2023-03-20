use crate::errno::Error;

#[link(wasm_import_module = "taubyte/sdk")]
extern "C" {
    pub fn storageNewContent(id: *mut u32) -> Error;
    pub fn storageOpenCid(id: *mut u32, cid: *const u8) -> Error;
    pub fn contentCloseFile(id: u32) -> Error;
    pub fn contentFileCid(id: u32, cid: *mut u8) -> Error;
    pub fn contentReadFile(id: u32, data: *mut u8, data_size: usize, count: *mut usize) -> Error;
    pub fn contentWriteFile(id: u32, data: *const u8, data_size: usize, write: *mut usize)
        -> Error;
    pub fn contentPushFile(id: u32, cid: *mut u8) -> Error;
    pub fn contentSeekFile(id: u32, offset: i64, whence: i32, offset_ptr: *mut i32) -> Error;
}
