// SPDX-License-Identifier: MPL-2.0

#![short_vis_path::add(fs)]

// `fs` refers to the deepest module.
pub(in fs) fn deepest_wins() {}
