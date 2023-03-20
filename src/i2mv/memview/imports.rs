use super::Error;

#[link(wasm_import_module = "taubyte/sdk")]
extern "C" {
    pub fn memoryViewNew(
        bufPtr: *const u8,
        bufSize: usize,
        isCloser: u32,
        idPtr: *mut u32,
    ) -> Error;
    pub fn memoryViewOpen(id: u32, isClosablePtr: *mut u32, size: *mut usize) -> Error;
    pub fn memoryViewRead(
        id: u32,
        off_set: u32,
        count: usize,
        bufPtr: *mut u8,
        nPtr: *mut usize,
    ) -> Error;
    pub fn memoryViewClose(id: u32);
}
