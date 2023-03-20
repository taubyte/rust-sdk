#[cfg(not(test))]
#[link(wasm_import_module = "taubyte/sdk")]
extern "C" {
    pub fn getEventType(baseEvent: u32, id: *mut u32);
}
