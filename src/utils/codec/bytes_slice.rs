pub fn to(buf:Vec<u8>) -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = Vec::new();
    let mut idx = 0;
    while idx < buf.len(){
        if idx+2 >= buf.len(){
            break
        }
        let size = u16::from_le_bytes([buf[idx], buf[idx+1]]) as usize;
        idx += 2;
        if idx+size > buf.len(){
            break
        }
        result.push(buf[idx..idx+size].to_vec());
        idx += size;
    }
    result
}
pub fn from(vec: Vec<Vec<u8>>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for mut s in vec {
        result.append(&mut s.len().to_le_bytes().to_vec());
        result.append(&mut s);
    }
    result
}