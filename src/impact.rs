use std::collections::BTreeSet;

use anyhow::Result;
use guppy::graph::{DependencyDirection, PackageGraph};

pub struct Impact {
    /// The changed crates and every workspace crate that depends on them, transitively.
    pub crates: BTreeSet<String>,
    /// Changed names that are not members of the workspace.
    pub unknown: Vec<String>,
}

pub fn impacted(graph: &PackageGraph, changed: &BTreeSet<String>) -> Result<Impact> {
    let workspace = graph.workspace();
    let mut ids = Vec::new();
    let mut unknown = Vec::new();
    for name in changed {
        match workspace.member_by_name(name) {
            Ok(package) => ids.push(package.id()),
            Err(_) => unknown.push(name.clone()),
        }
    }

    let crates = if ids.is_empty() {
        BTreeSet::new()
    } else {
        // The reverse query is already transitive and includes its roots.
        graph
            .query_reverse(ids)?
            .resolve()
            .packages(DependencyDirection::Reverse)
            .filter(|package| package.in_workspace())
            .map(|package| package.name().to_owned())
            .collect()
    };

    Ok(Impact { crates, unknown })
}
