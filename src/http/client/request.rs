use super::{imports, response};
use crate::{errno::Error, utils::convert};
use http::{Method, Request as HttpRequest, Uri};

pub struct Request {
    id: u32,
    client: u32,
}

fn new_request(client_id: u32, request_id: *mut u32) -> Error {
    #[allow(unused_unsafe)]
    unsafe {
        imports::newHttpRequest(client_id, request_id)
    }
}

impl Request {
    #[allow(unused_unsafe)]
    fn send_request(&self) -> Error {
        unsafe { imports::doHttpRequest(self.client, self.id) }
    }

    pub fn send(&self) -> Result<response::Response, Box<dyn std::error::Error>> {
        let err0 = self.send_request();
        if err0.is_err() {
            Err(format!("Sending request failed with: {}", err0).into())
        } else {
            Ok(response::new(self.client, self.id))
        }
    }

    pub fn new(
        client_id: u32,
        request: HttpRequest<&[u8]>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut id: u32 = 0;
        let err0 = new_request(client_id, &mut id);
        if err0.is_err() {
            return Err(format!("Creating client failed with: {}", err0).into());
        }

        let client_request = Request {
            id: id,
            client: client_id,
        };

        for (k, v) in request.headers() {
            let err0 = client_request.add_headers(k.as_str(), v.as_bytes());
            if err0.is_err() {
                return Err(format!("Adding header ({}: {:?}) failed with: {}", k, v, err0).into());
            }
        }

        client_request.set_method(request.method())?;
        client_request.set_url(request.uri())?;
        client_request.set_body(request.body())?;

        Ok(client_request)
    }

    pub fn new_without_body(
        client_id: u32,
        request: HttpRequest<()>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut id: u32 = 0;
        let err0 = new_request(client_id, &mut id);
        if err0.is_err() {
            return Err(format!("Creating client failed with: {}", err0).into());
        }

        let client_request = Request {
            id: id,
            client: client_id,
        };

        for (k, v) in request.headers() {
            let err0 = client_request.add_headers(k.as_str(), v.as_bytes());
            if err0.is_err() {
                return Err(format!("Adding header ({}: {:?}) failed with: {}", k, v, err0).into());
            }
        }

        client_request.set_method(request.method())?;
        client_request.set_url(request.uri())?;

        Ok(client_request)
    }

    fn add_headers(&self, key: &str, value: &[u8]) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::addHttpRequestHeader(
                self.client,
                self.id,
                key.as_ptr(),
                key.len(),
                value.as_ptr(),
                value.len(),
            )
        }
    }

    fn set_method(&self, method: &Method) -> Result<(), Box<dyn std::error::Error>> {
        let method = convert::method::to_u32(method).or_else(|err| Err(err))?;

        #[allow(unused_unsafe)]
        unsafe {
            let err0 = imports::setHttpRequestMethod(self.client, self.id, method);
            if err0.is_err() {
                Err(format!("Setting method failed with: {}", err0).into())
            } else {
                Ok(())
            }
        }
    }

    fn set_url(&self, url: &Uri) -> Result<(), Box<dyn std::error::Error>> {
        let url = url.to_string();

        #[allow(unused_unsafe)]
        unsafe {
            let err0 = imports::setHttpRequestURL(self.client, self.id, url.as_ptr(), url.len());
            if err0.is_err() {
                Err(format!("Setting url: `{}` failed with: {}", url, err0).into())
            } else {
                Ok(())
            }
        }
    }

    fn set_body(&self, body: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        #[allow(unused_unsafe)]
        unsafe {
            let err0 = imports::setHttpRequestBody(
                self.client,
                self.id,
                body.as_ptr(),
                body.len() as usize,
            );
            if err0.is_err() {
                Err(format!("Setting body failed with: {}", err0).into())
            } else {
                Ok(())
            }
        }
    }
}
