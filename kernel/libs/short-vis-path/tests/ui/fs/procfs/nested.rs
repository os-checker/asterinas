// SPDX-License-Identifier: MPL-2.0

#![short_vis_path::add(procfs)]

struct S;

impl S {
    // This is a nested item: inherent function item in an impl item.
    pub(in procfs) fn nested(&self) {}
}
