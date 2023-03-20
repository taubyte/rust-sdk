use super::{imports, Event};
use crate::errno::Error;

impl Event {
    fn method_size_unsafe(&self, size: *mut usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getHttpEventMethodSize(self.event, size)
        }
    }

    fn method_unsafe(&self, buf_ptr: *mut u8, buf_size: usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getHttpEventMethod(self.event, buf_ptr, buf_size)
        }
    }

    pub fn method(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut size: usize = 0;
        let err0 = self.method_size_unsafe(&mut size);
        if err0.is_err() {
            return Err(format!("Getting method size failed with: {}", err0).into());
        }

        let mut buf = vec![0u8; size];
        let err0 = self.method_unsafe(buf.as_mut_ptr(), size);
        if err0.is_err() {
            Err(format!("Getting method failed with: {}", err0).into())
        } else {
            Ok(String::from_utf8(buf)?)
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::http::Event;
    pub static EXPECTED_ID: u32 = 0;
    pub static EXPECTED_METHOD: &str = "POST";

    #[test]
    fn test_method() {
        let event = Event { event: EXPECTED_ID };
        let method = event.method().unwrap();
        assert_eq!(method, EXPECTED_METHOD);
    }
}
