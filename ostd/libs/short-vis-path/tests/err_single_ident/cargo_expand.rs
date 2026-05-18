#![feature(prelude_import)]
#![feature(proc_macro_hygiene)]
#![feature(custom_inner_attributes)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
mod procfs {
    pub(in crate::procfs) fn foo() {}
}
fn main() {
    procfs::foo()
}
