use super::{imports, Event, EventQueries};
use crate::errno::Error;
use crate::utils::codec;

impl Event {
    pub fn queries(&self) -> EventQueries {
        EventQueries {
            event: (self.event),
        }
    }
}

impl EventQueries {
    fn query_value_by_name_size_unsafe(&self, size: *mut usize, key: &str) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getHttpEventQueryValueByNameSize(self.event, size, key.as_ptr(), key.len())
        }
    }

    fn query_value_by_name_unsafe(&self, key: &str, buf_ptr: *mut u8, buf_size: usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getHttpEventQueryValueByName(
                self.event,
                key.as_ptr(),
                key.len(),
                buf_ptr,
                buf_size,
            )
        }
    }

    pub fn get(&self, key: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut size: usize = 0;
        let err0 = self.query_value_by_name_size_unsafe(&mut size, key);
        if err0.is_err() {
            return Err(format!(
                "Getting HTTP query size for key: `{}` failed with: {}",
                key, err0
            )
            .into());
        }
        if size == 0 {
            return Ok(String::new());
        }

        let mut buf = vec![0u8; size];
        let err0 = self.query_value_by_name_unsafe(key, buf.as_mut_ptr(), size);
        if err0.is_err() {
            Err(format!(
                "Getting HTTP query for key: `{}` failed with: {}",
                key, err0
            )
            .into())
        } else {
            Ok(String::from_utf8(buf)?)
        }
    }

    fn query_keys_size_unsafe(&self, size: *mut usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getHttpEventRequestQueryKeysSize(self.event, size)
        }
    }

    fn query_keys_unsafe(&self, buf_ptr: *mut u8) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getHttpEventRequestQueryKeys(self.event, buf_ptr)
        }
    }

    pub fn list(&self) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let mut size: usize = 0;
        let err0 = self.query_keys_size_unsafe(&mut size);
        if err0.is_err() {
            return Err(format!("Getting all query keys size failed with: {}", err0).into());
        }
        if size == 0 {
            return Ok(Vec::new());
        }

        let mut buf = vec![0u8; size];
        let err0 = self.query_keys_unsafe(buf.as_mut_ptr());
        if err0.is_err() {
            Err(format!("Getting all query keys failed with: {}", err0).into())
        } else {
            Ok(codec::string_slice::to(buf))
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::http::Event;
    pub static EXPECTED_ID: u32 = 0;
    pub static EXPECTED_QUERY_KEY: &str = "query1";
    pub static EXPECTED_QUERY_VALUE: &str = "value1";
    pub static EXPECTED_QUERY_KEYS: [&str; 2] = ["query1", "query2"];

    #[test]
    fn test_get() {
        let http: Event = Event {
            event: (EXPECTED_ID),
        };

        let queries = http.queries();
        let query = queries.get(EXPECTED_QUERY_KEY).unwrap();
        assert_eq!(query, EXPECTED_QUERY_VALUE);
    }

    #[test]
    fn test_list() {
        let http: Event = Event {
            event: (EXPECTED_ID),
        };

        let queries = http.queries();
        let query = queries.list().unwrap();

        assert_eq!(query.len(), EXPECTED_QUERY_KEYS.len());
        for i in 0..EXPECTED_QUERY_KEYS.len() {
            assert_eq!(query[i], EXPECTED_QUERY_KEYS[i]);
        }
    }
}
