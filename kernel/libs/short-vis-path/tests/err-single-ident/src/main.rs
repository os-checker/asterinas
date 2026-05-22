// SPDX-License-Identifier: MPL-2.0

#![feature(proc_macro_hygiene)]
#![feature(custom_inner_attributes)]

mod procfs;

fn main() {
    procfs::foo()
}
