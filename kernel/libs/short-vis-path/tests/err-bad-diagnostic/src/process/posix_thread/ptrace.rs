// SPDX-License-Identifier: MPL-2.0

// #![short_vis_path::add(process = crate::process)]

pub struct PosixThread {}

impl PosixThread {
    fn detach_tracer(&self) {}
}

fn f(thread: &PosixThread) {
    thread.detach_tracer_with()
}
