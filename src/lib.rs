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
        let rl_path = worktree.which("rl").ok_or("rl not found in PATH")?;

        Ok(zed::Command {
            command: rl_path,
            args: vec!["lsp".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(RlExtension);
