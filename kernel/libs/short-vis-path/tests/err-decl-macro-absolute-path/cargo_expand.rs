// SPDX-License-Identifier: MPL-2.0

#![feature(prelude_import)]
#![feature(decl_macro)]
#![feature(custom_inner_attributes)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
mod mm {}
mod cpu {}
fn main() {}
