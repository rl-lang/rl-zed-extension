# rl-zed-extension

RL language support for [Zed](https://zed.dev): syntax highlighting via
[tree-sitter-rl](https://github.com/rl-lang/tree-sitter-rl) and
diagnostics/hover/completions via `rlsp`.

## Requirements

The `rlsp` binary must be on your `PATH` (install
[rl-lang](https://github.com/rl-lang/rl-lang/releases) first, then reopen
the workspace). The extension reports a clear error otherwise.

## For maintainers

The grammar pin in `extension.toml` tracks
[rl-lang/tree-sitter-rl](https://github.com/rl-lang/tree-sitter-rl).
To update it:

1. Copy fresh queries:
   ```bash
   cp ../tree-sitter-rl/queries/rl/highlights.scm languages/rl/
   cp ../tree-sitter-rl/queries/rl/injections.scm languages/rl/
   ```
2. Point `[grammars.rl] commit` at the new upstream HEAD.
3. Bump `version` in `extension.toml`.

## License

MIT or Apache 2.0 at your option.
