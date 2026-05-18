#![feature(proc_macro_hygiene)]
#![feature(custom_inner_attributes)]
#![feature(decl_macro)]
#![allow(unused)]

mod fs;

mod mod_rs {
    mod procfs;
}

mod override_ {
    #[path = "procfs.rs"]
    pub mod procfs;
}

mod adhoc {
    // This attribute of outer style compiles and works. But rust-analyzer complains
    // "failed to write request: The length of a sequence must be known".
    #[short_vis_path::add(sys_procfs = crate::adhoc::outer)]
    pub mod outer {
        // the short name can be different from current module name
        pub(in sys_procfs) type Unit = ();
    }
}

fn main() {}
