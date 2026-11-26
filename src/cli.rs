use anyhow::{Result, bail};

pub const USAGE: &str = "\
Usage: cargo rev-deps <since>

Arguments:
  <since>  Git commit, tag or branch to compare the workspace against

Prints the workspace crates changed since <since>, plus every workspace
crate that depends on them, sorted and space-separated.";

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Run { since: String },
    Help,
}

/// Parses the full argv, program name included.
///
/// Run as `cargo rev-deps`, cargo passes `rev-deps` as the first argument, so
/// it is skipped when present.
pub fn parse(args: impl IntoIterator<Item = String>) -> Result<Command> {
    let mut args = args.into_iter().skip(1).peekable();
    if args.peek().is_some_and(|arg| arg == "rev-deps") {
        args.next();
    }

    let rest: Vec<String> = args.collect();
    if rest.iter().any(|arg| arg == "-h" || arg == "--help") {
        return Ok(Command::Help);
    }
    match rest.as_slice() {
        [since] if !since.starts_with('-') => Ok(Command::Run { since: since.clone() }),
        _ => bail!("{USAGE}"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Each argv is what the binary receives; the comment above it is what was typed.

    fn parse_strs(args: &[&str]) -> Result<Command> { parse(args.iter().map(|s| s.to_string())) }

    fn run(since: &str) -> Command { Command::Run { since: since.to_owned() } }

    #[test]
    fn as_cargo_subcommand() {
        // cargo rev-deps v1.0
        assert_eq!(parse_strs(&["cargo-rev-deps", "rev-deps", "v1.0"]).unwrap(), run("v1.0"));
    }

    #[test]
    fn as_standalone_binary() {
        // cargo-rev-deps v1.0
        assert_eq!(parse_strs(&["cargo-rev-deps", "v1.0"]).unwrap(), run("v1.0"));
    }

    #[test]
    fn help() {
        // cargo rev-deps --help
        assert_eq!(parse_strs(&["cargo-rev-deps", "rev-deps", "--help"]).unwrap(), Command::Help);
        // cargo-rev-deps -h
        assert_eq!(parse_strs(&["cargo-rev-deps", "-h"]).unwrap(), Command::Help);
    }

    #[test]
    fn help_wins_over_other_args() {
        // cargo rev-deps v1.0 --help
        assert_eq!(
            parse_strs(&["cargo-rev-deps", "rev-deps", "v1.0", "--help"]).unwrap(),
            Command::Help
        );
        // cargo-rev-deps -h a b
        assert_eq!(parse_strs(&["cargo-rev-deps", "-h", "a", "b"]).unwrap(), Command::Help);
    }

    #[test]
    fn rejects_missing_extra_or_unknown_args() {
        // cargo rev-deps
        assert!(parse_strs(&["cargo-rev-deps", "rev-deps"]).is_err());
        // cargo-rev-deps a b
        assert!(parse_strs(&["cargo-rev-deps", "a", "b"]).is_err());
        // cargo-rev-deps --since
        assert!(parse_strs(&["cargo-rev-deps", "--since"]).is_err());
    }
}
