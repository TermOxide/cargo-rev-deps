use std::collections::BTreeSet;

use anyhow::{Context, Result};

use crate::cargo;

/// Names of the workspace crates `cargo workspaces` reports as changed since `since`.
pub fn changed_since(since: &str) -> Result<BTreeSet<String>> {
    let stdout = cargo::run_checked(&["workspaces", "changed", "--since", since])
        .with_context(|| format!("could not list the crates changed since `{since}`"))?;
    Ok(stdout.split_whitespace().map(str::to_owned).collect())
}
