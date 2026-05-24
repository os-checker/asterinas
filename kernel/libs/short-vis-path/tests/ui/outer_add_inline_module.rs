// SPDX-License-Identifier: MPL-2.0

// NOTE: the attribute doesn't require any nightly feature to work.

pub mod ancestor {
    // This attribute of outer style compiles and works. But rust-analyzer complains
    // "failed to write request: The length of a sequence must be known",
    // or "proc-macro panicked: Unknown local file path to call site span".
    #[short_vis_path::add(ancestor = crate::ancestor)]
    pub mod outer_inline_module {
        // the short name can be different from current module name
        pub(in ancestor) type Inline = ();
    }
}
