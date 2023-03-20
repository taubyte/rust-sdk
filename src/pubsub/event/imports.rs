use crate::errno::Error;

#[link(wasm_import_module = "taubyte/sdk")]
extern "C" {
    pub fn getMessageData(eventId: u32, buf: *mut u8) -> Error;
    pub fn getMessageDataSize(eventId: u32, sizePtr: *mut usize) -> Error;
    pub fn getMessageChannel(eventId: u32, channelPtr: *mut u8) -> Error;
    pub fn getMessageChannelSize(eventId: u32, channelSizePtr: *mut usize) -> Error;
}
