use anyhow::{Result, bail};

use crate::process;

/// Fails unless `rev` names a commit.
///
/// `cargo workspaces changed --since <rev>` exits 0 with no output for a ref
/// that doesn't exist, which would read as "nothing changed".
pub fn ensure_commit_exists(rev: &str) -> Result<()> {
    let spec = format!("{rev}^{{commit}}");
    let output = process::run("git".as_ref(), &["rev-parse", "--verify", "--quiet", &spec])?;
    if !output.status.success() {
        bail!("`{rev}` is not a commit, tag or branch of this git repository");
    }
    Ok(())
}
