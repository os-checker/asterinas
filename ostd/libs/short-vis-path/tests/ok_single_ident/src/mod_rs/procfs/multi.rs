#![short_vis_path::add(procfs, fs = crate::mod_rs::procfs)]

pub(in procfs) fn foo() {}
pub(in fs) fn bar() {}
