pub fn read_string(ptr: *const u8, size: usize) -> String {
    unsafe {
        let buf = std::slice::from_raw_parts(ptr, size);
        std::str::from_utf8(buf).unwrap().to_string()
    }
}

pub fn read_bytes(ptr: *const u8, size: usize) -> Vec<u8> {
    unsafe {
        let buf = std::slice::from_raw_parts(ptr, size);
        buf.to_vec()
    }
}
