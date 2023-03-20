use super::imports;
use crate::{
    errno::{Errno, Error},
    utils::codec,
};

pub struct Response {
    client: u32,
    request: u32,
    consumed: bool,
}

pub struct ResponseHeaders {
    client: u32,
    request: u32,
}

pub fn new(client: u32, request: u32) -> Response {
    Response {
        client: client,
        request: request,
        consumed: false,
    }
}

impl std::io::Read for Response {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self.consumed {
            return Ok(0);
        }

        let mut n = 0;
        let err0 = unsafe {
            imports::readHttpResponseBody(
                self.client,
                self.request,
                buf.as_mut_ptr(),
                buf.len(),
                &mut n,
            )
        };
        if err0.is_errno(Errno::ErrorEOF) {
            // TODO, should we close here?

            self.consumed = true;
            return Ok(n as usize);
        }
        if err0.ok() {
            Ok(n as usize)
        } else {
            Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("reading body failed with: {}", err0),
            ))
        }
    }
}

impl Response {
    pub fn headers(&self) -> ResponseHeaders {
        ResponseHeaders {
            client: self.client,
            request: self.request,
        }
    }
}

impl ResponseHeaders {
    fn get_size_unsafe(&self, size: *mut usize, key: &str) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getHttpResponseHeaderSize(
                self.client,
                self.request,
                key.as_ptr(),
                key.len(),
                size,
            )
        }
    }

    fn get_unsafe(&self, key: &str, buf_ptr: &mut [u8]) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getHttpResponseHeader(
                self.client,
                self.request,
                key.as_ptr(),
                key.len(),
                buf_ptr.as_mut_ptr(),
            )
        }
    }

    pub fn get(&self, key: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
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
            return Ok(Vec::new());
        }

        let mut buf: Vec<u8> = vec![0; size];
        let err0 = self.get_unsafe(key, &mut buf);
        if err0.is_err() {
            return Err(format!(
                "Getting Header By Name for key:`{}` Failed with: {}",
                key, err0
            )
            .into());
        }

        Ok(codec::string_slice::to(buf))
    }

    fn list_size_unsafe(&self, size: *mut usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getHttpResponseHeaderKeysSize(self.client, self.request, size)
        }
    }

    fn list_unsafe(&self, buf_ptr: &mut [u8]) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::getHttpResponseHeaderKeys(
                self.client,
                self.request,
                buf_ptr.as_mut_ptr(),
                buf_ptr.len(),
            )
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
    use crate::http::Client;
    pub static CLIENT_ID: u32 = 1;
    pub static REQUEST_ID: u32 = 2;

    pub static HEADER_KEY: &str = "content-type";
    pub static HEADER_VALUE: &str = "text/html";
    pub static EXPECTED_KEYS: [&str; 2] = ["Content-Type", "User-Agent"];

    #[test]
    fn test_response_headers() {
        use crate::http::client::new::test::SEND_URL;
        let response = Client::get(SEND_URL).unwrap_or_else(|err| {
            panic!("Sending request failed with: {}", err);
        });

        // handle response
        let headers = response.headers();

        let header_value = headers.get(HEADER_KEY).unwrap_or_else(|err| {
            panic!(
                "Getting Header By Name for key:`{}` Failed with: {}",
                HEADER_KEY, err
            );
        });

        assert_eq!(header_value[0], HEADER_VALUE);

        let value = headers.list().unwrap_or_else(|err| {
            panic!("Getting Header Keys Failed with: {}", err);
        });

        assert_eq!(value, EXPECTED_KEYS);
    }
}
