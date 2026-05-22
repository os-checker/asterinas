// SPDX-License-Identifier: MPL-2.0

use std::{process::Command, sync::LazyLock};

use expect_test::expect_file;

macro_rules! snapshot {
    ($name:ident $success:literal: $stdout:expr; $stderr:expr) => {
        #[test]
        fn $name() {
            let dir = stringify!($name).replace('_', "-");
            let output = Command::new("cargo")
                .arg("expand")
                .current_dir(format!("tests/{dir}"))
                .output()
                .unwrap();

            let stdout = strip_pwd(std::str::from_utf8(&output.stdout).unwrap());
            const HEADER: &str = "// SPDX-License-Identifier: MPL-2.0";
            // Check if the expansion expected.
            expect_file![format!("{dir}/cargo_expand.rs")].assert_eq(&format!("{HEADER}\n\n{stdout}"));

            // Check if the output contains `pub(in crate::xx::subsystem)`.
            for substr in (&$stdout as &[&str]) {
                assert!(stdout.contains(substr), "{substr:?} should be included in the stdout of {dir}");
            }

            // Check if the error contains expected contents.
            let stderr = strip_pwd(std::str::from_utf8(&output.stderr).unwrap());
            for substr in (&$stderr as &[&str]) {
                assert!(stderr.contains(substr), "{substr:?} should be included in the stderr of {dir}");
            }

            // Stdout and stderr are redirected to user.
            let status = Command::new("cargo")
                .arg("check")
                .current_dir(format!("tests/{dir}"))
                .status()
                .unwrap();
            // Check if the code compiles.
            assert_eq!(
                status.success(), $success,
                "code compilation is expected to {expect}, while it {happen}",
                expect = if $success {"succeed"} else {"fail"},
                happen = if $success {"fail"} else {"succeeds"},
            );
        }
    };
    (@fail $($name:ident #stdout [$($stdout:literal),*] #stderr [$($stderr:literal),*]),+) => {
        $(snapshot! { $name false: [$($stdout),*]; [$($stderr),*] })+
    };
    (@success $($name:ident [$($stdout:literal),*]),+) => {
        $(snapshot! { $name true: [$($stdout),*]; [] })+
    };
}

snapshot! {
    @fail
    err_single_ident #stdout ["pub(in crate::procfs) fn foo()"] #stderr ["function `foo` is private"],
    err_decl_macro_absolute_path #stdout [] #stderr [
        "macro-expanded `macro_export` macros from the current crate cannot be referred to by absolute paths",
        "#[deny(macro_expanded_macro_exports_accessed_by_absolute_paths)]"
    ]
}
snapshot! {
    @success
    ok_single_ident [
        "pub(in crate::fs) f1:",
        "pub(in crate::fs) fn nested_impl_fn()",
        "pub(in crate::fs) fn baz()",
        "pub(in crate::fs) enum E {}",
        "pub(in crate::fs) macro mbe",
        "pub(in crate::fs) fn shallow_fs_wins()",
        "pub(in crate::fs::tmpfs::fs) fn deepest_fs_wins()",
        "pub(in crate::mod_rs::procfs) fn foo()",
        "pub(in crate::mod_rs::procfs) fn bar()",
        "pub(in crate::mod_rs::procfs) struct S;",
        "pub(in crate::override_::procfs) const UNIT:",
        "pub(in crate::adhoc::outer) type Unit"
    ],
    ok_decl_macro_2 []
}

// Don't include local path in output.
fn strip_pwd(s: &str) -> String {
    static PWD: LazyLock<String> = LazyLock::new(|| {
        std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    });

    s.replace(&**PWD, "$PWD")
}
