use std::{
    ffi::OsStr,
    io,
    process::{Command, Output},
};

use anyhow::{Context, Result, bail};

/// Runs `program args`, turning a missing executable into a readable error.
pub fn run(program: &OsStr, args: &[&str]) -> Result<Output> {
    let display = program.to_string_lossy();
    match Command::new(program).args(args).output() {
        Ok(output) => Ok(output),
        Err(err) if err.kind() == io::ErrorKind::NotFound => {
            bail!("`{display}` was not found: is it installed and on PATH?")
        },
        Err(err) => Err(err).with_context(|| format!("failed to run `{display} {}`", args.join(" "))),
    }
}

/// Like [`run`], and returns stdout, failing on a non-zero exit.
pub fn run_checked(program: &OsStr, args: &[&str]) -> Result<String> {
    let output = run(program, args)?;
    let display = || format!("{} {}", program.to_string_lossy(), args.join(" "));
    if !output.status.success() {
        bail!(
            "`{}` failed ({}):\n{}",
            display(),
            output.status,
            String::from_utf8_lossy(&output.stderr).trim_end()
        );
    }
    String::from_utf8(output.stdout).with_context(|| format!("`{}` printed non UTF-8 output", display()))
}
