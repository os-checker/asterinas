#![short_vis_path::add(procfs)]

mod multi;

mod inner {
    // This path will still be replaced, because it's in current file.
    pub(in procfs) struct S;
}
