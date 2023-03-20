use super::{imports, Event};
use crate::errno::Error;

impl Event {
    fn return_code_unsafe(&self, code: u32) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::eventHttpRetCode(self.event, code)
        }
    }

    pub fn return_code(&self, code: u32) -> Result<(), Box<dyn std::error::Error>> {
        let err0 = self.return_code_unsafe(code);
        if err0.is_err() {
            Err(format!("return code failed with: {}", err0).into())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
pub mod test {
    pub static EXPECTED_ID: u32 = 0;
    pub static EXPECTED_CODE: u32 = 200;

    #[test]
    fn test_return() {
        use crate::http::Event;

        let http: Event = Event {
            event: (EXPECTED_ID),
        };

        let wrong_http: Event = Event { event: (1) };
        let wrong_id = wrong_http.return_code(EXPECTED_CODE);
        assert!(wrong_id.is_err());

        let wrong_code = http.return_code(404);
        assert!(wrong_code.is_err());

        let success = http.return_code(EXPECTED_CODE);
        assert!(success.is_ok());
    }
}
