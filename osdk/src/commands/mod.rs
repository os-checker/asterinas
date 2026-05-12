// SPDX-License-Identifier: MPL-2.0

//! This module contains subcommands of cargo-osdk.

mod build;
mod debug;
mod new;
mod profile;
mod run;
mod test;
mod util;

pub use self::{
    build::execute_build_command, debug::execute_debug_command, new::execute_new_command,
    profile::execute_profile_command, run::execute_run_command, test::execute_test_command,
};

use crate::{arch::get_default_arch, cli::KtestWithForwardedArguments, error_msg};
use std::env;

/// Execute the forwarded cargo command with arguments.
pub fn execute_forwarded_command(subcommand: &str, cli: &KtestWithForwardedArguments) {
    let mut cargo = util::cargo();
    cargo.arg(subcommand).args(util::COMMON_CARGO_ARGS);
    if !cli.args.contains(&"--target".to_owned()) {
        cargo.arg("--target").arg(get_default_arch().triple());
    }
    cargo.args(&cli.args);

    let env_rustflags = env::var("RUSTFLAGS").unwrap_or_default();
    let rustflags = env_rustflags + " --check-cfg cfg(ktest)";
    let rustflags = if cli.ktests {
        rustflags + " --cfg ktest"
    } else {
        rustflags
    };
    cargo.env("RUSTFLAGS", rustflags);

    if cli.klint {
        if subcommand != "check" {
            error_msg!(@exit "`--klint` must be used only with `cargo osdk check`");
        }
        if let Ok(klint) = env::var("RUSTC") {
            if klint.contains("klint") {
                cargo.env("RUSTC", klint);
            } else {
                error_msg!(@exit "`--klint` can be used only with `cargo osdk check`");
            }
        } else {
            match which::which("klint") {
                Ok(path) => cargo.env("RUSTC", path),
                Err(_) => {
                    error_msg!(@exit "Command `klint` is not found. Please install it or put it in $PATH.");
                }
            };
        }
    }

    // When generating documentation via `cargo doc`, the `--check-cfg cfg(ktest)` flag
    // must be specified in both `RUSTFLAGS` and `RUSTDOCFLAGS`.
    if subcommand == "doc" {
        let env_rustdocflags = env::var("RUSTDOCFLAGS").unwrap_or_default();
        let rustdocflags = env_rustdocflags + " --check-cfg cfg(ktest)";
        cargo.env("RUSTDOCFLAGS", rustdocflags);
    }

    let status = cargo.status().expect("Failed to execute cargo");
    if !status.success() {
        error_msg!("Command {:?} failed with status: {:?}", cargo, status);
        std::process::exit(status.code().unwrap_or(1));
    }
}
