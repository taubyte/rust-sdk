use super::{imports, Client};
use crate::errno::Error;

fn new_client(id: *mut u32) -> Error {
    #[allow(unused_unsafe)]
    unsafe {
        imports::newHttpClient(id)
    }
}

impl Client {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut id: u32 = 0;
        let err0 = new_client(&mut id);
        if err0.is_err() {
            Err(format!("Creating client failed with: {}", err0).into())
        } else {
            Ok(Client { id: id })
        }
    }
}

// TODO split out as unit tests and mocks
#[cfg(test)]
pub mod test {
    use std::io::Read;

    use crate::http::Client;
    pub static CLIENT_ID: u32 = 1;
    pub static REQUEST_ID: u32 = 2;
    pub static METHOD_ID: u32 = 1;

    pub static HEADER_KEY: &str = "content-type";
    pub static HEADER_VALUE: &str = "application/json";
    pub static SEND_URL: &str = "https://google.com/";
    pub static RESPONSE_BODY: &str = "Hello, world!";

    #[test]
    fn test_client() {
        let client = Client::new().unwrap_or_else(|err| {
            panic!("Creating client failed with: {}", err);
        });

        assert_eq!(client.id, CLIENT_ID);

        let request = http::Request::builder()
            .header(HEADER_KEY, HEADER_VALUE)
            .uri(SEND_URL);

        let mut response = client.send_without_body(request).unwrap_or_else(|err| {
            panic!("Sending request failed with: {}", err);
        });

        // handle response
        let mut buffer = String::new();
        let err = response.read_to_string(&mut buffer);
        if err.is_err() {
            panic!("Reading response failed with: {}", err.err().unwrap());
        }

        assert_eq!(buffer, RESPONSE_BODY)
    }

    #[test]
    fn test_get() {
        let mut response = Client::get(SEND_URL).unwrap_or_else(|err| {
            panic!("Sending request failed with: {}", err);
        });

        // handle response
        let mut buffer = String::new();
        let err = response.read_to_string(&mut buffer);
        if err.is_err() {
            panic!("Reading response failed with: {}", err.err().unwrap());
        }

        assert_eq!(buffer, RESPONSE_BODY)
    }
}
