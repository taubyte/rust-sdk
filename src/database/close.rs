use super::{imports, Database};
use crate::errno::Error;

impl Database {
    fn close_unsafe(&self) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::databaseClose(self.id)
        }
    }

    pub fn close(&self) -> Result<(), Box<dyn std::error::Error>> {
        let err = self.close_unsafe();
        if err.is_err() {
            Err(format!("Closing database failed with: {}", err).into())
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
pub mod test {
    use crate::database::{new::test, Database};

    #[test]
    fn test_close() {
        let database = Database::new(test::DATABASE_NAME).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        database.close().unwrap_or_else(|err| {
            panic!("{}", err);
        })
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
pub mod mock {
    use crate::{
        database::new::test as new_test,
        errno::{Errno, Error},
    };

    pub fn databaseClose(id: u32) -> Error {
        if id != new_test::DATABASE_ID {
            Errno::ErrorCap.error()
        } else {
            Errno::ErrorNone.error()
        }
    }
}
