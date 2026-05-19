#![allow(dead_code)]
#![feature(decl_macro)]
#![feature(custom_inner_attributes)]

mod mm {
    pub mod frame {
        #![short_vis_path::add(mm = crate::mm)]

        pub mod meta {
            pub macro check_frame_meta_layout {
                () => {
                    println!("check_frame_meta_layout")
                },
                ($i:ident) => {
                    struct $i;
                }
            }

            pub macro impl_frame_meta_for {
                ($($t:tt)*) => {
                    $crate::check_frame_meta_layout!($($t)*);
                }
            }
        }

        mod untyped {
            #[macro_export]
            macro_rules! impl_untyped_frame_meta_for {
                () => {
                    check_frame_meta_layout!()
                };
            }
        }
    }
}

pub use crate::mm::frame::meta::{check_frame_meta_layout, impl_frame_meta_for};

mod cpu {
    crate::mm::frame::meta::impl_frame_meta_for!(S);
}

fn main() {
    impl_frame_meta_for!();
    impl_untyped_frame_meta_for!();
}
