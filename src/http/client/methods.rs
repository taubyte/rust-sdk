use super::{response::Response, Client};
use http::{method, request::Builder, Uri};

fn send_helper(builder: Builder) -> Result<Response, Box<dyn std::error::Error>> {
    let client = Client::new()?;
    client.send_without_body(builder)
}

impl Client {
    pub fn get<T>(uri: T) -> Result<Response, Box<dyn std::error::Error>>
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<http::Error>,
    {
        send_helper(
            http::Request::builder()
                .uri(uri)
                .method(method::Method::GET),
        )
    }

    pub fn post<T>(uri: T) -> Result<Response, Box<dyn std::error::Error>>
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<http::Error>,
    {
        send_helper(
            http::Request::builder()
                .uri(uri)
                .method(method::Method::POST),
        )
    }

    pub fn put<T>(uri: T) -> Result<Response, Box<dyn std::error::Error>>
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<http::Error>,
    {
        send_helper(
            http::Request::builder()
                .uri(uri)
                .method(method::Method::PUT),
        )
    }

    pub fn delete<T>(uri: T) -> Result<Response, Box<dyn std::error::Error>>
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<http::Error>,
    {
        send_helper(
            http::Request::builder()
                .uri(uri)
                .method(method::Method::DELETE),
        )
    }

    pub fn head<T>(uri: T) -> Result<Response, Box<dyn std::error::Error>>
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<http::Error>,
    {
        send_helper(
            http::Request::builder()
                .uri(uri)
                .method(method::Method::HEAD),
        )
    }
}
