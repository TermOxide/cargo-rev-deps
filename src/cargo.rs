use std::{env, ffi::OsString};

use anyhow::{Result, bail};

use crate::process;

/// The cargo that launched us (cargo sets `$CARGO` for its subcommands), or
/// `cargo` from `$PATH`.
fn cargo_bin() -> OsString { env::var_os("CARGO").unwrap_or_else(|| "cargo".into()) }

/// Runs `cargo <args>` and returns its stdout, failing on a non-zero exit.
pub fn run_checked(args: &[&str]) -> Result<String> { process::run_checked(&cargo_bin(), args) }

/// Fails with an install hint when the `cargo workspaces` subcommand is missing.
///
/// Cargo looks for subcommands in `$CARGO_HOME/bin` as well as `$PATH`, so
/// asking cargo itself is the only reliable check.
pub fn ensure_workspaces_installed() -> Result<()> {
    if !process::run(&cargo_bin(), &["workspaces", "--version"])?.status.success() {
        bail!(
            "the `cargo workspaces` subcommand is required but was not found\ninstall it with: cargo install \
             cargo-workspaces"
        );
    }
    Ok(())
}
