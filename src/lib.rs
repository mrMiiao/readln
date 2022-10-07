//! __*readln*__ crate provides you access to `read` macro.
//!
//! Example:
//!
//! ```no_run
//! #[macro_use]
//! extern crate readln;
//!
//! fn main() {
//!     let _x: &str = read!().unwrap();
//!     let _y: u8 = read!(u8).unwrap();
//! }
//! ```

#[derive(Debug)]
pub enum ReadError {
    StdinError,
    ParseError
}

pub type ReadResult<T> = Result<T, ReadError>;

/// The magic macro.
#[macro_export]
macro_rules! read {
    () => {
        {
            let mut input = String::new();
            if let Err(_) = std::io::stdin().read_line(&mut input) {
                $crate::ReadResult::Err($crate::ReadError::StdinError)
            } else {
                unsafe {
                    let ptr: *const str = input.trim_end();
                    core::mem::forget(input);
                    $crate::ReadResult::Ok(&*ptr)
                }
            }
        }
    };

    ($t:ty) => {
        {
            let mut input = String::new();
            if let Err(_) = std::io::stdin().read_line(&mut input) {
                $crate::ReadResult::Err($crate::ReadError::StdinError)
            } else {
                unsafe {
                    let ptr: *const str = input.trim_end();
                    core::mem::forget(input);
                    match (&*ptr).parse::<$t>() {
                        Ok(x) => $crate::ReadResult::Ok(x),
                        Err(_) => $crate::ReadResult::Err($crate::ReadError::ParseError)
                    }
                }
            }
        }
    };
}