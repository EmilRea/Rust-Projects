# AGENTS.md

## Cursor Cloud specific instructions

This repo is a small collection of standalone Rust CLI learning programs. There is no
server, database, web UI, or network dependency — each crate is a binary that prints to
stdout and exits.

### Layout / crates
- `hello_world` — prints `Hello, world!`
- `shapes_area_calculator` — sums areas of a list of shapes and prints the total
- `gradebook` — in-memory gradebook (`HashMap`) that computes letter grades and prints a report

The root `Cargo.toml` is a **virtual workspace** manifest tying these three crates together,
so standard cargo commands work from the repo root. (Before this, the root manifest was a
broken package and cargo's workspace walk-up caused every subcrate build to fail.)

### Toolchain (non-obvious)
- All crates declare `edition = "2024"`, which requires **Rust ≥ 1.85**. The base VM image
  may ship an older `rustc` (e.g. 1.83), which cannot parse these manifests. The startup
  update script installs/activates the `stable` toolchain via `rustup`, so a new-enough
  compiler is already the default when a session starts. If you ever see a
  `feature 'edition2024' is required` error, run `rustup default stable`.

### Common commands (run from repo root)
- Build: `cargo build`
- Lint: `cargo clippy --workspace` (currently emits only style warnings on existing code)
  and `cargo fmt --all --check`
- Test: `cargo test --workspace` (no tests are defined yet; the command still passes)
- Run a program: `cargo run -p <hello_world|shapes_area_calculator|gradebook>`

### Notes
- Only the root `/target` is git-ignored; the per-crate `target/` directories are committed
  in git history. Building from the workspace root writes artifacts to the root `/target`,
  so it does not touch those tracked files.
