// SPDX-License-Identifier: MPL-2.0

#![allow(dead_code)]
#![feature(proc_macro_hygiene)]
#![feature(custom_inner_attributes)]

#[cfg(not(good_diagnostics))]
pub mod ptrace;
// error[E0599]: no method named `detach_tracer_with` found for reference `&PosixThread` in the current scope
//  --> err-bad-diagnostic/src/main.rs:6:1
//   |
// 6 | pub mod ptrace;
//   | ^^^^^^^^^^^^^^^
//   |
// help: there is a method `detach_tracer` with a similar name
//   |
// 6 - pub mod ptrace;
// 6 + detach_tracer
//   |

#[cfg(good_diagnostics)]
pub mod ptrace {
    #![short_vis_path::add(process = crate::process)]
    pub struct PosixThread {}

    impl PosixThread {
        fn detach_tracer(&self) {}
    }

    fn f(thread: &PosixThread) {
        thread.detach_tracer_with()
    }
}
// error[E0599]: no method named `detach_tracer_with` found for reference `&PosixThread` in the current scope
//   --> err-bad-diagnostic/src/main.rs:16:16
//    |
// 16 |         thread.detach_tracer_with()
//    |                ^^^^^^^^^^^^^^^^^^
//    |
// help: there is a method `detach_tracer` with a similar name
//    |
// 16 -         thread.detach_tracer_with()
// 16 +         thread.detach_tracer()
//    |

fn main() {}
