# Sort Spaces

<p align="center">
  <a href="https://github.com/dorzey/herdr-sort-spaces-plugin/actions/workflows/ci.yml"><img alt="CI" src="https://github.com/dorzey/herdr-sort-spaces-plugin/actions/workflows/ci.yml/badge.svg" /></a>
  <img alt="Herdr 0.8.0+" src="https://img.shields.io/badge/Herdr-0.8.0%2B-66b3ff" />
  <img alt="Linux and macOS" src="https://img.shields.io/badge/platform-Linux%20%7C%20macOS-c084fc" />
</p>

Keeps Herdr workspaces ordered lexicographically by label. Sort on demand, or let it re-sort automatically whenever a workspace is created or renamed.

## Install

```bash
herdr plugin install dorzey/herdr-sort-spaces-plugin --yes
```

Two actions are added to the workspace context menu:

- **Sort spaces A-Z** (`sort-spaces.sort-asc`)
- **Sort spaces Z-A** (`sort-spaces.sort-desc`)

Auto-sort on `workspace.created` and `workspace.renamed` is on by default (ascending).

## Requirements

- Herdr `0.8.0` or newer
- Linux or macOS (uses the raw Unix socket for `workspace.move_block`; no Windows named pipe support)
- Rust stable + Cargo only when building from source

Build and link locally:

```bash
git clone https://github.com/dorzey/herdr-sort-spaces-plugin.git
cd herdr-sort-spaces-plugin
cargo build --release
herdr plugin link "$PWD"
```

## Testing

```bash
cargo test
```
