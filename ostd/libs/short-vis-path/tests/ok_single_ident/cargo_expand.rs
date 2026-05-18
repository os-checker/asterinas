#![feature(prelude_import)]
#![feature(proc_macro_hygiene)]
#![feature(custom_inner_attributes)]
#![feature(decl_macro)]
#![allow(unused)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2024::*;
mod fs {
    mod nested {
        pub struct S {
            pub(in crate::fs) f1: (),
        }
        impl S {
            pub(in crate::fs) fn nested_impl_fn() {}
        }
        pub(in crate::fs) fn baz() {}
    }
    mod procfs {
        pub(in crate::fs) enum E {}
    }
    mod mbe {
        pub(in crate::fs) macro mbe {
            () => {}
        }
    }
    mod ramfs {
        mod fs {
            pub(in crate::fs) fn shallow_fs_wins() {}
        }
    }
    mod tmpfs {
        mod fs {
            pub(in crate::fs::tmpfs::fs) fn deepest_fs_wins() {}
        }
    }
}
mod mod_rs {
    mod procfs {
        mod multi {
            pub(in crate::mod_rs::procfs) fn foo() {}
            pub(in crate::mod_rs::procfs) fn bar() {}
        }
        mod inner {
            pub(in crate::mod_rs::procfs) struct S;
        }
    }
}
mod override_ {
    #[path = "procfs.rs"]
    pub mod procfs {
        pub(in crate::override_::procfs) const UNIT: () = ();
    }
}
mod adhoc {
    pub mod outer {
        pub(in crate::adhoc::outer) type Unit = ();
    }
}
fn main() {}
