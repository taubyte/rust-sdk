use super::Error;
#[cfg(test)]
use super::{test, Errno};

#[cfg(not(test))]
#[link(wasm_import_module = "taubyte/sdk")]
extern "C" {
    pub fn fifoNew(closable: u32) -> u32;
    pub fn fifoIsCloser(id: u32, isCloser: *mut u32) -> Error;
    pub fn fifoPush(id: u32, buf: u8) -> Error;
    pub fn fifoPop(id: u32, bufPtr: *mut u8) -> Error;
    pub fn fifoClose(id: u32);
}
