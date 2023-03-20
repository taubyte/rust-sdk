use super::{imports, Event};
use crate::errno::Error;

impl Event {
    fn write_unsafe(&self, buffer: &[u8], n: &mut u32) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::eventHttpWrite(self.event, buffer.as_ptr(), buffer.len(), n)
        }
    }

    pub fn write(&self, buffer: &[u8]) -> Result<u32, Box<dyn std::error::Error>> {
        let mut n = 0;

        let err0 = self.write_unsafe(buffer, &mut n);
        if err0.is_err() {
            Err(format!("write failed with: {}", err0).into())
        } else {
            Ok(n)
        }
    }
}

#[cfg(test)]
pub mod test {
    pub static EXPECTED_ID: u32 = 0;
    pub static EXPECTED_WRITE: &str = "Hello, world!";

    #[test]
    fn test_write() {
        use crate::http::Event;

        let http: Event = Event {
            event: (EXPECTED_ID),
        };

        let wrong_string = http.write("pong".as_bytes());
        assert!(wrong_string.is_err());

        let wrong_http: Event = Event { event: (1) };
        let wrong_id = wrong_http.write(EXPECTED_WRITE.as_bytes());
        assert!(wrong_id.is_err());

        let success = http.write(EXPECTED_WRITE.as_bytes());
        assert!(success.is_ok());
    }
}
