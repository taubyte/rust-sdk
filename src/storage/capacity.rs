use super::{imports, Storage};
use crate::errno::Error;

// TODO confirm i64 is correct, using int in go-sdk
impl Storage {
    fn used_size_unsafe(&self, size: *mut usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::storageUsedSize(self.id, size)
        }
    }

    fn used_unsafe(&self, used: *mut u8) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::storageUsed(self.id, used)
        }
    }

    pub fn used(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let mut size: usize = 0;
        let err = self.used_size_unsafe(&mut size);
        if err.is_err() {
            Err(format!("Getting storage used size failed with: {}", err).into())
        } else {
            let mut used = vec![0; size];
            let err = self.used_unsafe(used.as_mut_ptr());
            if err.is_err() {
                Err(format!("Getting storage used failed with: {}", err).into())
            } else {
                let used = String::from_utf8(used)?;
                let used = used.parse::<i64>()?;
                Ok(used)
            }
        }
    }

    fn capacity_size_unsafe(&self, size: *mut usize) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::storageCapacitySize(self.id, size)
        }
    }

    fn capacity_unsafe(&self, capacity: *mut u8) -> Error {
        #[allow(unused_unsafe)]
        unsafe {
            imports::storageCapacity(self.id, capacity)
        }
    }

    pub fn capacity(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let mut size: usize = 0;
        let err = self.capacity_size_unsafe(&mut size);
        if err.is_err() {
            Err(format!("Getting storage capacity size failed with: {}", err).into())
        } else {
            let mut capacity = vec![0; size];
            let err = self.capacity_unsafe(capacity.as_mut_ptr());
            if err.is_err() {
                Err(format!("Getting storage capacity failed with: {}", err).into())
            } else {
                let capacity = String::from_utf8(capacity)?;
                let capacity = capacity.parse::<i64>()?;
                Ok(capacity)
            }
        }
    }

    pub fn remaining_capacity(&self) -> Result<i64, Box<dyn std::error::Error>> {
        let used = self.used()?;
        let capacity = self.capacity()?;
        Ok(capacity - used)
    }
}

#[cfg(test)]
pub mod test {
    use crate::storage::Storage;

    use crate::storage::new::test as new_test;
    pub static STORAGE_USED: &str = "500";
    pub static STORAGE_CAPACITY: &str = "1000";

    #[test]
    fn test_capacity() {
        let storage = Storage::new(new_test::STORAGE_NAME).unwrap_or_else(|err| {
            panic!("{}", err);
        });

        let used = storage.used().unwrap_or_else(|err| {
            panic!("{}", err);
        });
        assert_eq!(used, STORAGE_USED.parse::<i64>().unwrap());

        let capacity = storage.capacity().unwrap_or_else(|err| {
            panic!("{}", err);
        });
        assert_eq!(capacity, STORAGE_CAPACITY.parse::<i64>().unwrap());

        let remaining_capacity = storage.remaining_capacity().unwrap_or_else(|err| {
            panic!("{}", err);
        });
        assert_eq!(remaining_capacity, 500);
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
pub mod mock {
    use crate::{
        errno::{Errno, Error},
        storage::new::test as new_test,
        utils::test as utils,
    };

    pub fn storageUsedSize(id: u32, size: *mut usize) -> Error {
        use super::test;

        if id != new_test::STORAGE_ID {
            Errno::ErrorCap.error()
        } else {
            utils::write_usize(size, test::STORAGE_USED.len());
            Errno::ErrorNone.error()
        }
    }

    pub fn storageUsed(id: u32, used: *mut u8) -> Error {
        use super::test;

        if id != new_test::STORAGE_ID {
            Errno::ErrorCap.error()
        } else {
            utils::write_string(used, test::STORAGE_USED);
            Errno::ErrorNone.error()
        }
    }

    pub fn storageCapacitySize(id: u32, size: *mut usize) -> Error {
        use super::test;

        if id != new_test::STORAGE_ID {
            Errno::ErrorCap.error()
        } else {
            utils::write_usize(size, test::STORAGE_CAPACITY.len());
            Errno::ErrorNone.error()
        }
    }

    pub fn storageCapacity(id: u32, capacity: *mut u8) -> Error {
        use super::test;

        if id != new_test::STORAGE_ID {
            Errno::ErrorCap.error()
        } else {
            utils::write_string(capacity, test::STORAGE_CAPACITY);
            Errno::ErrorNone.error()
        }
    }
}
