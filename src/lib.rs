#![feature(split_inclusive)]
#![feature(async_closure)]

#[cfg(any(feature="wee_alloc",target_arch = "wasm32"))]
extern crate wee_alloc;

#[cfg(any(feature="wee_alloc",target_arch = "wasm32"))]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use crate::convert::{Converter, ConverterBuild};
#[cfg(target_arch = "wasm32")]
use crate::convert::{Convertor, ConvertorBuild};

mod chars;
mod convert;
mod dict;
