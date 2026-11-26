# cargo-rev-deps

Lists the crates of a Cargo workspace changed since a git commit or tag, along with every crate that
depends on them, directly or not. CI uses the list to build and test only what a change can break.

## Install

```sh
cargo install cargo-workspaces cargo-rev-deps --locked
```

It runs `cargo workspaces changed`, so `cargo-workspaces` must be installed too.

## Use

From the workspace root:

```sh
cargo-rev-deps <since_commit_or_tag>
```

It prints the impacted crates on one line, sorted and separated by spaces, and nothing when no crate
changed.

## License

BSD-2-Clause
