#![feature(prelude_import)]
#![allow(dead_code)]
#![feature(decl_macro)]
#![feature(custom_inner_attributes)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
mod mm {
    pub mod frame {
        pub mod meta {
            pub macro check_frame_meta_layout {
                () => { println!("check_frame_meta_layout") }, ($i : ident) => { struct
                $i; }
            }
            pub macro impl_frame_meta_for {
                ($($t : tt)*) => { $crate::check_frame_meta_layout!($($t)*); }
            }
        }
        mod untyped {}
    }
}
pub use crate::mm::frame::meta::{check_frame_meta_layout, impl_frame_meta_for};
mod cpu {
    struct S;
}
fn main() {
    {
        ::std::io::_print(format_args!("check_frame_meta_layout\n"));
    };
    {
        ::std::io::_print(format_args!("check_frame_meta_layout\n"));
    };
}
