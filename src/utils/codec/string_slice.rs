pub fn to(buf: Vec<u8>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut last_index: usize = 0;
    for idx in 0..buf.len() {
        if buf[idx] == 0 {
            let res = String::from_utf8(buf[last_index..idx].to_vec()).unwrap();

            // Ignoring ptr to slice
            if res.len() > 0 {
                result.push(res);
            }
            last_index = idx + 1;
        }
    }

    result
}
