use super::{imports, Event};
use crate::errno::Error;

impl Event {
    fn host_size_unsafe(&self, size: *mut usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getHttpEventHostSize(self.event, size)
        }
    }

    fn host_unsafe(&self, buf_ptr: *mut u8, buf_size: usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getHttpEventHost(self.event, buf_ptr, buf_size)
        }
    }

    pub fn host(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut size: usize = 0;
        let err0 = self.host_size_unsafe(&mut size);
        if err0.is_err() {
            return Err(format!("Getting host size failed with: {}", err0).into());
        }

        let mut buf = vec![0u8; size];
        let err0 = self.host_unsafe(buf.as_mut_ptr(), size);
        if err0.is_err() {
            return Err(format!("Getting host failed with: {}", err0).into());
        }

        Ok(String::from_utf8(buf)?)
    }
}

#[cfg(test)]
pub mod test {
    use crate::http::Event;
    pub static EXPECTED_ID: u32 = 0;
    pub static EXPECTED_HOST: &str = "hal.computers.com";

    #[test]
    fn test_host() {
        let event = Event { event: EXPECTED_ID };
        let host = event.host().unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(host, EXPECTED_HOST);
    }
}
