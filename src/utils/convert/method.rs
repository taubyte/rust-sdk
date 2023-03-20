use http::Method;

enum EnumMethod {
    GET = 1,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    TRACE,
    CONNECT,
}

pub fn to_u32(method: &Method) -> Result<u32, Box<dyn std::error::Error>> {
    match method.as_str() {
        "GET" => Ok(EnumMethod::GET as u32),
        "POST" => Ok(EnumMethod::POST as u32),
        "PUT" => Ok(EnumMethod::PUT as u32),
        "DELETE" => Ok(EnumMethod::DELETE as u32),
        "HEAD" => Ok(EnumMethod::HEAD as u32),
        "OPTIONS" => Ok(EnumMethod::OPTIONS as u32),
        "PATCH" => Ok(EnumMethod::PATCH as u32),
        "TRACE" => Ok(EnumMethod::TRACE as u32),
        "CONNECT" => Ok(EnumMethod::CONNECT as u32),
        _ => Err(format!("Method {} not supported", method.as_str()).into()),
    }
}

pub fn to_method(method: u32) -> Result<Method, Box<dyn std::error::Error>> {
    let http_method = match method {
        0 => return Err(format!("Method {} not supported", method).into()),
        1 => Method::GET,
        2 => Method::POST,
        3 => Method::PUT,
        4 => Method::DELETE,
        5 => Method::HEAD,
        6 => Method::OPTIONS,
        7 => Method::PATCH,
        8 => Method::TRACE,
        9 => Method::CONNECT,
        _ => return Err(format!("Method {} not supported", method).into()),
    };

    Ok(http_method)
}
