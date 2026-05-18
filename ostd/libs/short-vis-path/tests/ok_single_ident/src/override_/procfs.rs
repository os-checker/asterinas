#![short_vis_path::add(fs = crate::override_::procfs)]

// fs is an alias to the absolute path
pub(in fs) const UNIT: () = ();
