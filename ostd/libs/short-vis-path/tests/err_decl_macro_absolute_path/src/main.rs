#![allow(dead_code)]
#![feature(decl_macro)]
#![feature(custom_inner_attributes)]

mod mm {
    pub mod frame {
        // This produces error in cpu module:
        // "cannot find macro `impl_frame_meta_for` in this scope"
        #![short_vis_path::add(mm = crate::mm)]

        pub mod meta {
            #[macro_export]
            macro_rules! check_frame_meta_layout {
                () => {
                    println!("check_frame_meta_layout")
                };
                ($i:ident) => {
                    struct $i;
                };
            }

            #[macro_export]
            macro_rules! impl_frame_meta_for {
                ($($t:tt)*) => {
                    check_frame_meta_layout!($($t)*);
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

mod cpu {
    impl_frame_meta_for!(S);
}

fn main() {
    impl_frame_meta_for!();
    impl_untyped_frame_meta_for!();
}
