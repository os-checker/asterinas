use expect_test::expect_file;
use std::{process::Command, sync::LazyLock};

macro_rules! snapshot {
    ($name:ident $success:literal $substr:expr) => {
        #[test]
        fn $name() {
            let dir = stringify!($name);
            let output = Command::new("cargo")
                .arg("expand")
                .current_dir(format!("tests/{dir}"))
                .output()
                .unwrap();

            let stdout = strip_pwd(std::str::from_utf8(&output.stdout).unwrap());
            // Check if the expansion expected.
            expect_file![format!("{dir}/cargo_expand.rs")].assert_eq(&stdout);

            // Check if the output contains `pub(in crate::xx::subsystem)`.
            for substr in (&$substr as &[&str]) {
                assert!(stdout.contains(substr), "{substr:?} should be included in the output of {dir}");
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
    (@fail $($name:ident [$($substr:literal),*]),+) => {
        $(snapshot! { $name false [$($substr),*] })+
    };
    (@success $($name:ident [$($substr:literal),*]),+) => {
        $(snapshot! { $name true [$($substr),*] })+
    };
}

snapshot! {
    @fail err_single_ident ["pub(in crate::procfs) fn foo()"]
}
snapshot! {
    @success ok_single_ident [
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
    ]
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
