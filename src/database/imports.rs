use crate::errno::Error;

#[link(wasm_import_module = "taubyte/sdk")]
extern "C" {
    pub fn newDatabase(name_ptr: *const u8, name_size: usize, id: *mut u32) -> Error;
    pub fn databaseGet(id: u32, key_ptr: *const u8, key_size: usize, data: *mut u8) -> Error;
    pub fn databaseGetSize(id: u32, key_ptr: *const u8, key_size: usize, size: *mut usize)
        -> Error;
    pub fn databasePut(
        id: u32,
        key_ptr: *const u8,
        key_size: usize,
        data: *mut u8,
        data_size: usize,
    ) -> Error;
    pub fn databaseClose(id: u32) -> Error;
    pub fn databaseDelete(id: u32, key_ptr: *const u8, key_size: usize) -> Error;
    pub fn databaseList(id: u32, key_ptr: *const u8, key_size: usize, data: *mut u8) -> Error;
    pub fn databaseListSize(
        id: u32,
        key_ptr: *const u8,
        key_size: usize,
        size: *mut usize,
    ) -> Error;
}
