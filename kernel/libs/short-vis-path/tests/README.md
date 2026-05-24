# How to test the crate

## Set `SHORT_VIS_PATH_DIR` to `test/expand`

`#![short_vis_path::add]` requires a correct base directory, while the tested
Rust files reside in a non-standard Cargo project structure, so
`SHORT_VIS_PATH_DIR` must be set to specify the correct file prefix.

```bash
# Do this:
kernel/libs/short-vis-path $ SHORT_VIS_PATH_DIR=$PWD/tests/expand cargo test
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.34s

# Not do this:
kernel/libs/short-vis-path $ cargo test
tests/expand/integration_test.expanded.rs - refreshed
test integration_test ... FAILED

failures:

---- integration_test stdout ----
Running 1 macro expansion tests

thread 'integration_test' (129687) panicked at kernel/libs/short-vis-path/tests/expand.rs:22:9:
`pub(in crate::fs) enum Enum` must be included in the expanded content:
`#![feature(proc_macro_hygiene)]
#![feature(custom_inner_attributes)]
pub mod fs {
    pub mod procfs {}
}
`
```

`macrotest::expand` manually constructs Cargo workspace with
`CARGO_MANIFEST_DIR` and module files not sharing the same path prefix, the `add`
attribute just panics, and code won't expand to anything, thus tests must fail.

## Add a testcase

If the test case to be added can share the same module layout with
`integration_test.rs`, add a submodule in the `expand/fs/procfs` directory and
append a new matching string in `check! { integration_test }`, with the module
file listed above the string.

If the test case has a separate module layout (e.g., inline vs. file-based
modules), add the submodule in the `expand/ancestor` directory.
