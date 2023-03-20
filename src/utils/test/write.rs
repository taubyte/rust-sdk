use cid::Cid;

use crate::utils::codec;

pub fn write_u32(id: *mut u32, to_write: u32) {
    unsafe { *id = to_write }
}

pub fn write_u8(id: *mut u8, to_write: u8) {
    unsafe { *id = to_write }
}

pub fn write_usize(id: *mut usize, to_write: usize) {
    unsafe { *id = to_write }
}

pub fn write_string(ptr: *mut u8, to_write: &str) {
    unsafe {
        let buf = std::slice::from_raw_parts_mut(ptr, to_write.len());
        buf.copy_from_slice(to_write.as_bytes());
    }
}

pub fn write_bytes(ptr: *mut u8, to_write: &[u8]) {
    unsafe {
        let buf = std::slice::from_raw_parts_mut(ptr, to_write.len());
        buf.copy_from_slice(to_write);
    }
}

pub fn write_bytes_vec(ptr: *mut u8, to_write: Vec<u8>) {
    unsafe {
        let buf = std::slice::from_raw_parts_mut(ptr, to_write.len());
        buf.copy_from_slice(&to_write);
    }
}

pub fn write_string_slice(ptr: *mut u8, to_write: &[&str]) {
    let v: Vec<String> = to_write.iter().map(|s| s.to_string()).collect();
    let bytes_slice = codec::byte_slice::from_string_slice(v);

    unsafe {
        let buf = std::slice::from_raw_parts_mut(ptr, bytes_slice.len());
        buf.copy_from_slice(&bytes_slice);
    }
}

pub fn write_string_slice_size(ptr: *mut usize, to_write: &[&str]) {
    let v: Vec<String> = to_write.iter().map(|s| s.to_string()).collect();
    let bytes_slice = codec::byte_slice::from_string_slice(v);
    write_usize(ptr, bytes_slice.len())
}

pub fn write_cid_string(cid: &str, cid_ptr: *mut u8) {
    let cid = Cid::try_from(cid).unwrap();
    write_bytes(cid_ptr, cid.to_bytes().as_slice())
}
