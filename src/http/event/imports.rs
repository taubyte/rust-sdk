use crate::errno::Error;

#[cfg(test)]
use crate::{errno::Errno, utils::test as utils};

#[cfg(not(test))]
#[link(wasm_import_module = "taubyte/sdk")]
extern "C" {
    pub fn eventHttpWrite(
        event_id: u32,
        _buf_ptr: *const u8,
        _buf_size: usize,
        _n_ptr: *mut u32,
    ) -> Error;
    pub fn eventHttpRetCode(event_id: u32, _code: u32) -> Error;
    pub fn readHttpEventBody(
        event_id: u32,
        _buf_ptr: *const u8,
        _buf_size: usize,
        _n_ptr: *mut u32,
    ) -> Error;
    pub fn eventHttpHeaderAdd(
        event_id: u32,
        _key_ptr: *const u8,
        _key_size: usize,
        _value_ptr: *const u8,
        _value_size: usize,
    ) -> Error;
    pub fn getHttpEventHeadersSize(
        event_id: u32,
        _size: *mut usize,
        _key_ptr: *const u8,
        _key_size: usize,
    ) -> Error;
    pub fn getHttpEventHeaders(
        event_id: u32,
        _key_ptr: *const u8,
        _key_size: usize,
        _buf_ptr: *mut u8,
        _buf_size: usize,
    ) -> Error;
    pub fn getHttpEventRequestHeaderKeysSize(event_id: u32, _size: *mut usize) -> Error;
    pub fn getHttpEventRequestHeaderKeys(
        event_id: u32,
        _buf_ptr: *mut u8,
        _buf_size: usize,
    ) -> Error;
    pub fn getHttpEventMethodSize(event_id: u32, _size: *mut usize) -> Error;
    pub fn getHttpEventMethod(event_id: u32, _buf_ptr: *mut u8, _buf_size: usize) -> Error;
    pub fn getHttpEventHostSize(event_id: u32, _size: *mut usize) -> Error;
    pub fn getHttpEventHost(event_id: u32, _buf_ptr: *mut u8, _buf_size: usize) -> Error;
    pub fn getHttpEventPathSize(event_id: u32, _size: *mut usize) -> Error;
    pub fn getHttpEventPath(event_id: u32, _buf_ptr: *mut u8, _buf_size: usize) -> Error;
    pub fn getHttpEventUserAgentSize(event_id: u32, _size: *mut usize) -> Error;
    pub fn getHttpEventUserAgent(event_id: u32, _buf_ptr: *mut u8, _buf_size: usize) -> Error;

    pub fn getHttpEventQueryValueByNameSize(
        event_id: u32,
        _size: *mut usize,
        _key_ptr: *const u8,
        _key_size: usize,
    ) -> Error;
    pub fn getHttpEventQueryValueByName(
        event_id: u32,
        _key_ptr: *const u8,
        _key_size: usize,
        _buf_ptr: *mut u8,
        _buf_size: usize,
    ) -> Error;
    pub fn getHttpEventRequestQueryKeysSize(event_id: u32, _size: *mut usize) -> Error;
    pub fn getHttpEventRequestQueryKeys(event_id: u32, _buf_ptr: *mut u8) -> Error;
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn eventHttpWrite(
    event_id: u32,
    buf_ptr: *const u8,
    buf_size: usize,
    _n_ptr: *mut u32,
) -> Error {
    use super::write::test;

    let wrote = utils::read_bytes(buf_ptr, buf_size);

    // Compare to expected
    if wrote != test::EXPECTED_WRITE.as_bytes() {
        Errno::ErrorCap.error()
    } else if event_id != test::EXPECTED_ID {
        Errno::ErrorCap.error()
    } else {
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn eventHttpRetCode(event_id: u32, code: u32) -> Error {
    use super::r#return::test;

    // Compare to expected
    if code != test::EXPECTED_CODE {
        Errno::ErrorCap.error()
    } else if event_id != test::EXPECTED_ID {
        Errno::ErrorCap.error()
    } else {
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub unsafe fn readHttpEventBody(
    event_id: u32,
    buf_ptr: *mut u8,
    buf_size: usize,
    n_ptr: *mut u32,
) -> Error {
    use super::body::test;

    let body = test::EXPECTED_BODY.as_bytes();

    utils::write_bytes(buf_ptr, body);
    utils::write_u32(n_ptr, body.len() as u32);

    // return EOF when it reaches the end
    if event_id != test::EXPECTED_ID {
        Errno::ErrorCap.error()
    } else if buf_size >= body.len() {
        Errno::ErrorEOF.error()
    } else {
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn eventHttpHeaderAdd(
    event_id: u32,
    key_ptr: *const u8,
    key_size: usize,
    value_ptr: *const u8,
    value_size: usize,
) -> Error {
    use super::headers::test;

    let key = utils::read_string(key_ptr, key_size);
    let value = utils::read_string(value_ptr, value_size);

    if event_id != test::EXPECTED_ID {
        Errno::ErrorCap.error()
    } else if key != test::EXPECTED_KEY {
        Errno::ErrorCap.error()
    } else if value != test::EXPECTED_VALUE {
        Errno::ErrorCap.error()
    } else {
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn getHttpEventHeadersSize(
    event_id: u32,
    size: *mut usize,
    key_ptr: *const u8,
    key_size: usize,
) -> Error {
    use super::headers::test;

    let key = utils::read_string(key_ptr, key_size);

    // Compare id to 0:
    if event_id != test::EXPECTED_ID {
        Errno::ErrorCap.error()
    } else if key != test::EXPECTED_KEY {
        Errno::ErrorCap.error()
    } else {
        utils::write_usize(size, test::EXPECTED_VALUE.len());
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn getHttpEventHeaders(
    event_id: u32,
    key_ptr: *const u8,
    key_size: usize,
    buf_ptr: *mut u8,
    _buf_size: usize,
) -> Error {
    use super::headers::test;
    let key = utils::read_string(key_ptr, key_size);

    // Compare id to 0:
    if event_id != test::EXPECTED_ID {
        Errno::ErrorCap.error()
    } else if key != test::EXPECTED_KEY {
        Errno::ErrorCap.error()
    } else {
        utils::write_string(buf_ptr, test::EXPECTED_VALUE);
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn getHttpEventRequestHeaderKeysSize(event_id: u32, size: *mut usize) -> Error {
    use super::headers::test;

    if event_id != test::EXPECTED_ID {
        Errno::ErrorCap.error()
    } else {
        utils::write_string_slice_size(size, &test::EXPECTED_KEYS);
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn getHttpEventRequestHeaderKeys(event_id: u32, buf_ptr: *mut u8, _buf_size: usize) -> Error {
    use super::headers::test;

    if event_id != test::EXPECTED_ID {
        Errno::ErrorCap.error()
    } else {
        utils::write_string_slice(buf_ptr, &test::EXPECTED_KEYS);
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn getHttpEventMethodSize(event_id: u32, size: *mut usize) -> Error {
    use super::method::test;
    if event_id != test::EXPECTED_ID {
        Errno::ErrorCap.error()
    } else {
        utils::write_usize(size, test::EXPECTED_METHOD.len());
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn getHttpEventMethod(event_id: u32, buf_ptr: *mut u8, _buf_size: usize) -> Error {
    use super::method::test;
    if event_id != test::EXPECTED_ID {
        Errno::ErrorCap.error()
    } else {
        utils::write_string(buf_ptr, test::EXPECTED_METHOD);
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn getHttpEventHostSize(event_id: u32, size: *mut usize) -> Error {
    use super::host::test;
    if event_id != test::EXPECTED_ID {
        Errno::ErrorCap.error()
    } else {
        utils::write_usize(size, test::EXPECTED_HOST.len());
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn getHttpEventHost(event_id: u32, buf_ptr: *mut u8, _buf_size: usize) -> Error {
    use super::host::test;
    if event_id != test::EXPECTED_ID {
        Errno::ErrorCap.error()
    } else {
        utils::write_string(buf_ptr, test::EXPECTED_HOST);
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn getHttpEventPathSize(event_id: u32, size: *mut usize) -> Error {
    use super::path::test;
    if event_id != test::EXPECTED_ID {
        Errno::ErrorCap.error()
    } else {
        utils::write_usize(size, test::EXPECTED_PATH.len());
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn getHttpEventPath(event_id: u32, buf_ptr: *mut u8, _buf_size: usize) -> Error {
    use super::path::test;
    if event_id != test::EXPECTED_ID {
        Errno::ErrorCap.error()
    } else {
        utils::write_string(buf_ptr, test::EXPECTED_PATH);
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn getHttpEventUserAgentSize(event_id: u32, size: *mut usize) -> Error {
    use super::user_agent::test;
    if event_id != test::EXPECTED_ID {
        Errno::ErrorCap.error()
    } else {
        utils::write_usize(size, test::EXPECTED_USER_AGENT.len());
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn getHttpEventUserAgent(event_id: u32, buf_ptr: *mut u8, _buf_size: usize) -> Error {
    use super::user_agent::test;
    if event_id != test::EXPECTED_ID {
        Errno::ErrorCap.error()
    } else {
        utils::write_string(buf_ptr, test::EXPECTED_USER_AGENT);
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn getHttpEventQueryValueByNameSize(
    event_id: u32,
    size: *mut usize,
    key_ptr: *const u8,
    key_size: usize,
) -> Error {
    use super::query::test;

    let key = utils::read_string(key_ptr, key_size);

    if event_id != test::EXPECTED_ID {
        Errno::ErrorCap.error()
    } else if key != test::EXPECTED_QUERY_KEY {
        Errno::ErrorCap.error()
    } else {
        utils::write_usize(size, test::EXPECTED_QUERY_VALUE.len());
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn getHttpEventQueryValueByName(
    event_id: u32,
    key_ptr: *const u8,
    key_size: usize,
    buf_ptr: *mut u8,
    _buf_size: usize,
) -> Error {
    use super::query::test;

    let key = utils::read_string(key_ptr, key_size);

    if event_id != test::EXPECTED_ID {
        Errno::ErrorCap.error()
    } else if key != test::EXPECTED_QUERY_KEY {
        Errno::ErrorCap.error()
    } else {
        utils::write_string(buf_ptr, test::EXPECTED_QUERY_VALUE);
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn getHttpEventRequestQueryKeysSize(event_id: u32, size: *mut usize) -> Error {
    use super::query::test;

    if event_id != test::EXPECTED_ID {
        Errno::ErrorCap.error()
    } else {
        utils::write_string_slice_size(size, &test::EXPECTED_QUERY_KEYS);
        Errno::ErrorNone.error()
    }
}

#[allow(non_snake_case)]
#[cfg(test)]
pub fn getHttpEventRequestQueryKeys(event_id: u32, buf_ptr: *mut u8) -> Error {
    use super::query::test;

    if event_id != test::EXPECTED_ID {
        Errno::ErrorCap.error()
    } else {
        utils::write_string_slice(buf_ptr, &test::EXPECTED_QUERY_KEYS);
        Errno::ErrorNone.error()
    }
}
