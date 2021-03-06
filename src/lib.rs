#![crate_name = "gmp"]

#![warn(deprecated)]
#![allow(non_camel_case_types)]

extern crate libc;
extern crate num_traits;

mod ffi;
pub mod mpz;
pub mod mpq;
pub mod mpf;
pub mod rand;

#[cfg(test)]
mod test;
