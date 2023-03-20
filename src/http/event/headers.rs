use super::{imports, Event, EventHeaders};
use crate::errno::Error;
use crate::utils::codec;

impl Event {
    pub fn headers(&self) -> EventHeaders {
        EventHeaders {
            event: (self.event),
        }
    }
}

impl EventHeaders {
    fn set_unsafe(&self, key: &str, value: &str) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::eventHttpHeaderAdd(
                self.event,
                key.as_ptr(),
                key.len(),
                value.as_ptr(),
                value.len(),
            )
        }
    }

    pub fn set(&self, key: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
        if key.len() == 0 {
            Err("Cannot set header with empty key".into())
        } else {
            let err0 = self.set_unsafe(key, value);
            if err0.is_err() {
                Err(format!("Setting header failed with: {}", err0).into())
            } else {
                Ok(())
            }
        }
    }

    fn get_size_unsafe(&self, size: *mut usize, key: &str) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getHttpEventHeadersSize(self.event, size, key.as_ptr(), key.len())
        }
    }

    fn get_unsafe(&self, key: &str, buf_ptr: &mut [u8], buf_size: usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getHttpEventHeaders(
                self.event,
                key.as_ptr(),
                key.len(),
                buf_ptr.as_mut_ptr(),
                buf_size,
            )
        }
    }

    // TODO this should return a Vec<String> need to change on vm and go-sdk
    pub fn get(&self, key: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut size: usize = 0;
        let err0 = self.get_size_unsafe(&mut size, key);
        if err0.is_err() {
            return Err(format!(
                "Getting Header Size By Name for key:`{}` Failed with: {}",
                key, err0
            )
            .into());
        }
        if size == 0 {
            return Ok("".to_string());
        }

        let mut buf: Vec<u8> = vec![0; size];
        let err0 = self.get_unsafe(key, &mut buf, size);
        if err0.is_err() {
            return Err(format!(
                "Getting Header By Name for key:`{}` Failed with: {}",
                key, err0
            )
            .into());
        }

        let s = String::from_utf8(buf);
        if s.is_err() {
            Err(format!(
                "Converting header slice to string slice failed with: {}",
                s.err().unwrap()
            )
            .into())
        } else {
            Ok(s.unwrap())
        }
    }

    fn list_size_unsafe(&self, size: *mut usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getHttpEventRequestHeaderKeysSize(self.event, size)
        }
    }

    fn list_unsafe(&self, buf_ptr: &mut [u8]) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getHttpEventRequestHeaderKeys(self.event, buf_ptr.as_mut_ptr(), buf_ptr.len())
        }
    }

    pub fn list(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut size: usize = 0;
        let err0 = self.list_size_unsafe(&mut size);
        if err0.is_err() {
            return Err(format!("Getting Header Keys Size Failed with: {}", err0).into());
        }
        if size == 0 {
            return Ok(Vec::<String>::new());
        }

        let mut buf: Vec<u8> = vec![0; size];
        let err0 = self.list_unsafe(&mut buf);
        if err0.is_err() {
            Err(format!("Getting Header Keys Failed with: {}", err0).into())
        } else {
            Ok(codec::string_slice::to(buf))
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::http::Event;
    pub static EXPECTED_ID: u32 = 0;
    pub static EXPECTED_KEY: &str = "Content-Type";
    pub static EXPECTED_VALUE: &str = "text/html";

    #[test]
    fn test_set() {
        let http: Event = Event {
            event: (EXPECTED_ID),
        };

        let headers = http.headers();
        let err0 = headers.set(EXPECTED_KEY, EXPECTED_VALUE);
        if err0.is_err() {
            panic!("Error setting header: {}", err0.err().unwrap());
        }
    }

    #[test]
    fn test_get() {
        let http: Event = Event {
            event: (EXPECTED_ID),
        };

        let headers = http.headers();
        let err = headers.set(EXPECTED_KEY, EXPECTED_VALUE);
        if err.is_err() {
            panic!("Error setting header: {}", err.err().unwrap());
        }

        let value = headers.get(EXPECTED_KEY).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(value, EXPECTED_VALUE);
    }

    pub static EXPECTED_KEYS: [&str; 2] = ["Content-Type", "User-Agent"];
    #[test]
    fn test_list() {
        let http: Event = Event {
            event: (EXPECTED_ID),
        };

        let headers = http.headers();
        let value = headers.list().unwrap_or_else(|err| {
            panic!("{}", err);
        });

        assert_eq!(value.len(), EXPECTED_KEYS.len());
        for i in 0..EXPECTED_KEYS.len() {
            assert_eq!(value[i], EXPECTED_KEYS[i]);
        }
    }
}
