use zed_extension_api::{self as zed, LanguageServerId, Result};

struct RlExtension;

impl zed::Extension for RlExtension {
    fn new() -> Self {
        RlExtension
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // `rlsp` ships with the rl-lang toolchain (install script, package
        // managers, or `cargo install`). There is no auto-download: the
        // release archives bundle the whole toolchain, which the extension
        // sandbox cannot unpack.
        let rlsp = worktree
            .which("rlsp")
            .ok_or("rlsp not found in PATH - install rl-lang (https://github.com/rl-lang/rl-lang/releases), then reopen the workspace")?;

        Ok(zed::Command {
            command: rlsp,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(RlExtension);
