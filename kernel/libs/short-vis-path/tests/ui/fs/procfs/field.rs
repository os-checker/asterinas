// SPDX-License-Identifier: MPL-2.0

#![short_vis_path::add(procfs)]

pub struct Data {
    // Restricted visibility on fields should be supported.
    pub(in procfs) field: (),
}
