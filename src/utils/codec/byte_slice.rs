pub fn from_string_slice(vec: Vec<String>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for s in vec {
        result.append(&mut s.into_bytes());
        result.push(0);
    }

    result
}
