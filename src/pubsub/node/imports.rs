use crate::errno::Error;

#[link(wasm_import_module = "taubyte/sdk")]
extern "C" {
    pub fn setSubscriptionChannel(channel_ptr: *const u8, channel_size: usize) -> Error;
    pub fn publishToChannel(
        channel_ptr: *const u8,
        channel_size: usize,
        data: *const u8,
        data_size: usize,
    ) -> Error;
    pub fn getWebSocketURLSize(
        channel_ptr: *const u8,
        channel_size: usize,
        size: *mut usize,
    ) -> Error;
    pub fn getWebSocketURL(
        channel_ptr: *const u8,
        channel_size: usize,
        socket_url: *mut u8,
    ) -> Error;
}
