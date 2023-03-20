pub fn from_bool(val: bool) -> u32 {
    val as u32
}

pub fn to_bool(val: u32) -> bool {
    val == 1
}
