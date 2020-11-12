#![feature(split_inclusive)]

mod utils;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use crate::convert::{Converter, ConverterBuild};

mod chars;
mod convert;
mod dict;

#[cfg(test)]
mod test;
