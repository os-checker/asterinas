// SPDX-License-Identifier: MPL-2.0

#![feature(decl_macro)]
#![feature(custom_inner_attributes)]

mod mm {
    // This produces macro-expanded error in cpu module.
    #![short_vis_path::add(mm = crate::mm)]

    #[macro_export]
    macro_rules! impl_frame_meta_for {
        () => {};
    }
}

mod cpu {
    crate::impl_frame_meta_for!();
    // error: macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths
    //   --> err-decl-macro-absolute-path/src/main.rs:15:5
    //    |
    // 15 |     crate::impl_frame_meta_for!();
    //    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
    //    |
    // note: the macro is defined here
    //   --> err-decl-macro-absolute-path/src/main.rs:9:5
    //    |
    //  9 | /     macro_rules! impl_frame_meta_for {
    // 10 | |         () => {};
    // 11 | |     }
    //    | |_____^
    //    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
    //    = note: for more information, see issue #52234 <https://github.com/rust-lang/rust/issues/52234>
    //    = note: `#[deny(macro_expanded_macro_exports_accessed_by_absolute_paths)]` (part of `#[deny(future_incompatible)]`) on by default
}

fn main() {}
