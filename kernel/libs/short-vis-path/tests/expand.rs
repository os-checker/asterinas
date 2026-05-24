// SPDX-License-Identifier: MPL-2.0

#![doc = include_str!("../README.md")]

struct Expanded {
    file_path: String,
    content: String,
}

impl Expanded {
    fn run(name: &str) -> Expanded {
        const PREFIX_DIR: &str = "tests/expand";
        let src = format!("{PREFIX_DIR}/{name}.rs");

        // This function run `cargo expand`, and generate a local expanded file.
        macrotest::expand(&src);

        let file_path = format!("{PREFIX_DIR}/{name}.expanded.rs");
        let content = std::fs::read_to_string(&file_path).unwrap();
        Expanded { file_path, content }
    }

    fn check_contains(&self, target_str: &str) {
        let content = &self.content;
        assert!(
            content.contains(target_str),
            "`{target_str}` must be included in the expanded content:\n`{content}`"
        );
    }
}

impl Drop for Expanded {
    fn drop(&mut self) {
        // Remove the expanded file, because we don't need the file.
        std::fs::remove_file(&self.file_path).unwrap();
    }
}

macro_rules! check {
    // # Arugments
    //
    // * `$name` refers to a file stem under `expand` directory.
    // * `$target_str` refers to a list of string that contains the expected
    //   expanded path (better with extra item names included for good search).
    ($name:ident: $($target_str:literal),+) => {
        #[test]
        fn $name() {
            let expanded = Expanded::run(stringify!($name));
            $( expanded.check_contains($target_str); )+
        }
    };
    ($( $name:ident: $($target_str:literal),+ );+ $(;)?) => {
        $( check! { $name: $($target_str),+ } )+
    };
}

// Share entry point `fs/procfs/` to test usages on items and the attribute syntax.
check! { integration_test:
    // override.rs
    "pub(in crate::fs) enum Enum",
    // field.rs
    "pub(in crate::fs::procfs) field",
    // nested.rs
    "pub(in crate::fs::procfs) fn nested",
    // fs.rs
    "pub(in crate::fs::procfs::fs) fn deepest_wins",
    // multi_args.rs
    "pub(in crate::fs) fn foo",
    "pub(in crate::fs::procfs) fn bar"
}

check! {
    file_based_module: "pub(in crate::ancestor) type Child";
    file_based_module_mod_style: "pub(in crate::ancestor) type ModStyle";
    outer_add_inline_module: "pub(in crate::ancestor) type Inline";
}
