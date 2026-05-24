// SPDX-License-Identifier: MPL-2.0

#![short_vis_path::add(fs = crate::fs)]

// `fs` refers to the specified ancestor module.
pub(in fs) enum Enum {}
