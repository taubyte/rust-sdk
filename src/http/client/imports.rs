use crate::errno::Error;

#[cfg(test)]
use crate::{errno::Errno, utils::test as utils};

#[cfg(not(test))]
#[link(wasm_import_module = "taubyte/sdk")]
extern "C" {
    pub fn newHttpClient(_client_id: *mut u32) -> Error;
    pub fn newHttpRequest(_client_id: u32, _request_id: *mut u32) -> Error;
    pub fn addHttpRequestHeader(
        _client_id: u32,
        _request_id: u32,
        _key_ptr: *const u8,
        _key_size: usize,
        _value_ptr: *const u8,
        _value_size: usize,
    ) -> Error;
    pub fn setHttpRequestMethod(_client_id: u32, _request_id: u32, _method_id: u32) -> Error;
    pub fn setHttpRequestURL(
        _client_id: u32,
        _request_id: u32,
        _url_ptr: *const u8,
        _url_size: usize,
    ) -> Error;
    pub fn doHttpRequest(_client_id: u32, _request_id: u32) -> Error;
    pub fn readHttpResponseBody(
        _client_id: u32,
        _request_id: u32,
        _buf_ptr: *mut u8,
        _buf_size: usize,
        _count_ptr: *mut u32,
    ) -> Error;
    pub fn setHttpRequestBody(
        client_id: u32,
        request_id: u32,
        buf_ptr: *const u8,
        buf_size: usize,
    ) -> Error;
    pub fn getHttpResponseHeaderSize(
        client_id: u32,
        request_id: u32,
        key_ptr: *const u8,
        key_size: usize,
        size: *mut usize,
    ) -> Error;
    pub fn getHttpResponseHeader(
        client_id: u32,
        request_id: u32,
        key_ptr: *const u8,
        key_size: usize,
        buf_ptr: *mut u8,
    ) -> Error;
    pub fn getHttpResponseHeaderKeysSize(
        client_id: u32,
        request_id: u32,
        size: *mut usize,
    ) -> Error;
    pub fn getHttpResponseHeaderKeys(
        client_id: u32,
        request_id: u32,
        buf_ptr: *mut u8,
        buf_size: usize,
    ) -> Error;
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn newHttpClient(_client_id: *mut u32) -> Error {
    use super::new::test;
    unsafe {
        *_client_id = test::CLIENT_ID;
    }
    Errno::ErrorNone.error()
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn newHttpRequest(_client_id: u32, _request_id: *mut u32) -> Error {
    use super::new::test;
    unsafe {
        *_request_id = test::REQUEST_ID;
    }
    Errno::ErrorNone.error()
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn addHttpRequestHeader(
    client_id: u32,
    request_id: u32,
    key_ptr: *const u8,
    key_size: usize,
    value_ptr: *const u8,
    value_size: usize,
) -> Error {
    use super::new::test;
    let key = utils::read_string(key_ptr, key_size);
    let value = utils::read_string(value_ptr, value_size);

    if client_id != test::CLIENT_ID {
        Errno::ErrorCap.error()
    } else if request_id != test::REQUEST_ID {
        Errno::ErrorCap.error()
    } else if key != test::HEADER_KEY {
        Errno::ErrorCap.error()
    } else if value != test::HEADER_VALUE {
        Errno::ErrorCap.error()
    } else {
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn setHttpRequestMethod(client_id: u32, request_id: u32, method_id: u32) -> Error {
    use super::new::test;

    if client_id != test::CLIENT_ID {
        Errno::ErrorCap.error()
    } else if request_id != test::REQUEST_ID {
        Errno::ErrorCap.error()
    } else if method_id != test::METHOD_ID {
        Errno::ErrorCap.error()
    } else {
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn setHttpRequestURL(
    client_id: u32,
    request_id: u32,
    url_ptr: *const u8,
    url_size: usize,
) -> Error {
    use super::new::test;
    let url = utils::read_string(url_ptr, url_size);

    if client_id != test::CLIENT_ID {
        Errno::ErrorCap.error()
    } else if request_id != test::REQUEST_ID {
        Errno::ErrorCap.error()
    } else if url != test::SEND_URL {
        Errno::ErrorCap.error()
    } else {
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn doHttpRequest(client_id: u32, request_id: u32) -> Error {
    use super::new::test;

    if client_id != test::CLIENT_ID {
        Errno::ErrorCap.error()
    } else if request_id != test::REQUEST_ID {
        Errno::ErrorCap.error()
    } else {
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn setHttpRequestBody(
    client_id: u32,
    request_id: u32,
    _buf_ptr: *const u8,
    _buf_size: usize,
) -> Error {
    use super::new::test;

    // TODO confirm body
    if client_id != test::CLIENT_ID {
        Errno::ErrorCap.error()
    } else if request_id != test::REQUEST_ID {
        Errno::ErrorCap.error()
    } else {
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub unsafe fn readHttpResponseBody(
    client_id: u32,
    request_id: u32,
    buf_ptr: *mut u8,
    buf_size: usize,
    count_ptr: *mut u32,
) -> Error {
    use super::new::test;

    if client_id != test::CLIENT_ID {
        Errno::ErrorCap.error()
    } else if request_id != test::REQUEST_ID {
        Errno::ErrorCap.error()
    } else {
        let body = test::RESPONSE_BODY.as_bytes();

        utils::write_bytes(buf_ptr, body);
        utils::write_u32(count_ptr, body.len() as u32);

        if buf_size >= body.len() {
            Errno::ErrorEOF.error()
        } else {
            Errno::ErrorNone.error()
        }
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn getHttpResponseHeaderSize(
    client_id: u32,
    request_id: u32,
    key_ptr: *const u8,
    key_size: usize,
    size: *mut usize,
) -> Error {
    use crate::{http::client::response::test, utils::codec};

    if client_id != test::CLIENT_ID {
        return Errno::ErrorCap.error();
    } else if request_id != test::REQUEST_ID {
        return Errno::ErrorCap.error();
    }

    let key = utils::read_string(key_ptr, key_size);
    assert_eq!(key, test::HEADER_KEY);

    let v = vec![test::HEADER_VALUE.to_string()];
    let header = codec::string_slice::from(v);
    utils::write_usize(size, header.len());

    Errno::ErrorNone.error()
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn getHttpResponseHeader(
    client_id: u32,
    request_id: u32,
    key_ptr: *const u8,
    key_size: usize,
    buf_ptr: *mut u8,
) -> Error {
    use crate::{http::client::response::test, utils::codec};

    if client_id != test::CLIENT_ID {
        return Errno::ErrorCap.error();
    } else if request_id != test::REQUEST_ID {
        return Errno::ErrorCap.error();
    }

    let key = utils::read_string(key_ptr, key_size);
    assert_eq!(key, test::HEADER_KEY);

    let v = vec![test::HEADER_VALUE.to_string()];
    let header = codec::string_slice::from(v);
    utils::write_bytes_vec(buf_ptr, header);

    Errno::ErrorNone.error()
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn getHttpResponseHeaderKeysSize(client_id: u32, request_id: u32, size: *mut usize) -> Error {
    use crate::http::client::response::test;

    if client_id != test::CLIENT_ID {
        Errno::ErrorCap.error()
    } else if request_id != test::REQUEST_ID {
        Errno::ErrorCap.error()
    } else {
        utils::write_string_slice_size(size, &test::EXPECTED_KEYS);
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn getHttpResponseHeaderKeys(
    client_id: u32,
    request_id: u32,
    buf_ptr: *mut u8,
    _buf_size: usize,
) -> Error {
    use crate::http::client::response::test;

    if client_id != test::CLIENT_ID {
        return Errno::ErrorCap.error();
    } else if request_id != test::REQUEST_ID {
        return Errno::ErrorCap.error();
    } else {
        utils::write_string_slice(buf_ptr, &test::EXPECTED_KEYS);

        Errno::ErrorNone.error()
    }
}
