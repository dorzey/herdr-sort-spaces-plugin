# Sort Spaces

<p align="center">
  <a href="https://github.com/dorzey/herdr-sort-spaces-plugin/actions/workflows/ci.yml"><img alt="CI" src="https://github.com/dorzey/herdr-sort-spaces-plugin/actions/workflows/ci.yml/badge.svg" /></a>
  <a href="LICENSE"><img alt="MIT License" src="https://img.shields.io/badge/license-MIT-2ea44f" /></a>
  <img alt="Herdr 0.8.0+" src="https://img.shields.io/badge/Herdr-0.8.0%2B-66b3ff" />
  <img alt="Linux and macOS" src="https://img.shields.io/badge/platform-Linux%20%7C%20macOS-c084fc" />
</p>

Keeps Herdr workspaces ordered lexicographically by label. Sort on demand, or let it re-sort automatically whenever a workspace is created or renamed.

## Install

```bash
herdr plugin install dorzey/herdr-sort-spaces-plugin --yes
```

Four actions are added to the workspace context menu:

- **Sort spaces A-Z** (`sort-spaces.sort-asc`)
- **Sort spaces Z-A** (`sort-spaces.sort-desc`)
- **sort-spaces: install the default keybinding** (`sort-spaces.setup-keys`)
- **sort-spaces: remove the default keybinding** (`sort-spaces.remove-keys`)

Auto-sort on `workspace.created` and `workspace.renamed` is on by default (ascending).

### Keybinding

Herdr only binds keys from your own config, so run **sort-spaces: install the
default keybinding** once (from the workspace action menu, or
`herdr plugin action invoke sort-spaces.setup-keys`) to write `prefix+shift+s`
→ `sort-spaces.sort-asc` into `~/.config/herdr/config.toml`. It backs the file
up first and leaves a commented-out binding for `sort-desc` beside it. Run
**sort-spaces: remove the default keybinding** to undo it.

To bind a key yourself instead:

```toml
[[keys.command]]
key = "prefix+shift+s"
type = "plugin_action"
command = "sort-spaces.sort-asc"
description = "sort spaces A-Z"
```

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
