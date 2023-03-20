use cid::Cid;

pub struct Reader {
    data: Vec<u8>,
}

impl Reader {
    pub fn new() -> Reader {
        Reader { data: vec![0; 64] }
    }

    pub fn ptr(&mut self) -> *mut u8 {
        self.data.as_mut_ptr()
    }

    pub fn parse(&self) -> Result<Cid, Box<dyn std::error::Error>> {
        let err = Cid::try_from(self.data.as_slice());
        if err.is_err() {
            Err(format!("Parsing cid failed with: {}", err.unwrap_err()).into())
        } else {
            Ok(err.unwrap())
        }
    }
}
