#![feature(prelude_import)]
#![allow(dead_code)]
#![feature(decl_macro)]
#![feature(custom_inner_attributes)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
mod mm {
    pub mod frame {
        pub mod meta {}
        mod untyped {}
    }
}
mod cpu {}
fn main() {
    {
        ::std::io::_print(format_args!("check_frame_meta_layout\n"));
    };
    {
        ::std::io::_print(format_args!("check_frame_meta_layout\n"));
    };
}
