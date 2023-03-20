use super::{imports, Event};
use crate::errno::Error;

impl Event {
    fn path_size_unsafe(&self, size: *mut usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getHttpEventPathSize(self.event, size)
        }
    }

    fn path_unsafe(&self, buf_ptr: *mut u8, buf_size: usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getHttpEventPath(self.event, buf_ptr, buf_size)
        }
    }

    pub fn path(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut size: usize = 0;
        let err0 = self.path_size_unsafe(&mut size);
        if err0.is_err() {
            return Err(format!("Getting path size failed with: {}", err0).into());
        }

        let mut buf = vec![0u8; size];
        let err0 = self.path_unsafe(buf.as_mut_ptr(), size);
        if err0.is_err() {
            Err(format!("Getting path failed with: {}", err0).into())
        } else {
            Ok(String::from_utf8(buf)?)
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::http::Event;
    pub static EXPECTED_ID: u32 = 0;
    pub static EXPECTED_PATH: &str = "/test/v1";

    #[test]
    fn test_path() {
        let event = Event { event: EXPECTED_ID };
        let path = event.path().unwrap();
        assert_eq!(path, EXPECTED_PATH);
    }
}
