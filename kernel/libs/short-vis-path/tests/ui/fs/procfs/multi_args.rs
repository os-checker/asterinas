// SPDX-License-Identifier: MPL-2.0

// Two arguments should recognized.
#![short_vis_path::add(procfs, fs = crate::fs)]

pub(in fs) fn foo() {}

pub(in procfs) fn bar() {}
