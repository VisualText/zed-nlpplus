use std::env;
use zed_extension_api::{self as zed, Result};

const SERVER_NAME: &str = "nlpplus-language-server";
const PACKAGE_NAME: &str = "nlpplus-language-server";
const SERVER_SCRIPT: &str = "node_modules/nlpplus-language-server/bin/nlpplus-language-server.js";

struct NlpplusExtension {
    checked_for_update: bool,
}

impl NlpplusExtension {
    // Installs or updates the server from npm, once per session.
    fn ensure_package(&mut self, id: &zed::LanguageServerId) -> Result<()> {
        let installed = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if installed.is_some() && self.checked_for_update {
            return Ok(());
        }

        zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let latest = zed::npm_package_latest_version(PACKAGE_NAME)?;

        if installed.as_ref() != Some(&latest) {
            zed::set_language_server_installation_status(
                id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            if let Err(error) = zed::npm_install_package(PACKAGE_NAME, &latest) {
                // Keep using the installed version if the update fails.
                if installed.is_none() {
                    return Err(error);
                }
            }
        }

        self.checked_for_update = true;
        Ok(())
    }
}

impl zed::Extension for NlpplusExtension {
    fn new() -> Self {
        Self {
            checked_for_update: false,
        }
    }

    fn language_server_command(
        &mut self,
        id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // A server the user installed (npm i -g nlpplus-language-server) wins.
        if let Some(path) = worktree.which(SERVER_NAME) {
            return Ok(zed::Command {
                command: path,
                args: vec!["--stdio".to_string()],
                env: Default::default(),
            });
        }

        self.ensure_package(id)?;
        let script = env::current_dir()
            .map_err(|e| e.to_string())?
            .join(SERVER_SCRIPT)
            .to_string_lossy()
            .to_string();

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![script, "--stdio".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(NlpplusExtension);
