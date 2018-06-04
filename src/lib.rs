extern crate libc;
#[allow(unused_variables)]
#[allow(dead_code)]

mod ffi;
mod matrix;
pub mod swt;

pub use matrix::Matrix;

pub use matrix::OpenAs;
pub use matrix::FileFormat;