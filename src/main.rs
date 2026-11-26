mod cargo;
mod changed;
mod cli;
mod git;
mod impact;
mod process;

use std::env;

use anyhow::{Context, Result};
use guppy::MetadataCommand;

fn main() -> Result<()> {
    let since = match cli::parse(env::args())? {
        cli::Command::Help => {
            println!("{}", cli::USAGE);
            return Ok(());
        },
        cli::Command::Run { since } => since,
    };

    git::ensure_commit_exists(&since)?;
    cargo::ensure_workspaces_installed()?;

    let changed = changed::changed_since(&since)?;
    if changed.is_empty() {
        return Ok(());
    }

    let graph = MetadataCommand::new()
        .build_graph()
        .context("failed to load the workspace with `cargo metadata`")?;

    let impact = impact::impacted(&graph, &changed)?;
    for name in &impact.unknown {
        eprintln!("warning: `{name}` is not a workspace member, skipping it");
    }
    if !impact.crates.is_empty() {
        println!("{}", Vec::from_iter(impact.crates).join(" "));
    }

    Ok(())
}
