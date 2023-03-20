use super::{request::Request as ClientRequest, response::Response, Client};
use http::{request::Builder, Request};

impl Client {
    pub fn send(&self, request: Request<&[u8]>) -> Result<Response, Box<dyn std::error::Error>> {
        let err = ClientRequest::new(self.id, request);
        if err.is_err() {
            Err(format!("Creating request failed with: {}", err.err().unwrap()).into())
        } else {
            err.unwrap().send()
        }
    }

    pub fn send_without_body(
        &self,
        request: Builder,
    ) -> Result<Response, Box<dyn std::error::Error>> {
        let err = ClientRequest::new_without_body(self.id, request.body(()).unwrap());
        if err.is_err() {
            Err(format!("Creating request failed with: {}", err.err().unwrap()).into())
        } else {
            err.unwrap().send()
        }
    }
}
