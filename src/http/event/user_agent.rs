use super::{imports, Event};
use crate::errno::Error;

impl Event {
    fn user_agent_size_unsafe(&self, size: *mut usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getHttpEventUserAgentSize(self.event, size)
        }
    }

    fn user_agent_unsafe(&self, buf_ptr: *mut u8, buf_size: usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getHttpEventUserAgent(self.event, buf_ptr, buf_size)
        }
    }

    pub fn user_agent(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut size: usize = 0;
        let err0 = self.user_agent_size_unsafe(&mut size);
        if err0.is_err() {
            return Err(format!("Getting user_agent size failed with: {}", err0).into());
        }

        let mut buf = vec![0u8; size];
        let err0 = self.user_agent_unsafe(buf.as_mut_ptr(), size);
        if err0.is_err() {
            Err(format!("Getting user_agent failed with: {}", err0).into())
        } else {
            Ok(String::from_utf8(buf)?)
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::http::Event;
    pub static EXPECTED_ID: u32 = 0;
    pub static EXPECTED_USER_AGENT: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/517.36 (KHTML, like Gecko) Chrome/101.0.0.0 Safari/5447.36";

    #[test]
    fn test_user_agent() {
        let event = Event { event: EXPECTED_ID };
        let user_agent = event.user_agent().unwrap();

        assert_eq!(user_agent, EXPECTED_USER_AGENT);
    }
}
