#![short_vis_path::add(fs)]

pub struct S {
    pub(in fs) f1: (),
}

impl S {
    pub(in fs) fn nested_impl_fn() {}
}

pub(in fs) fn baz() {}
