use std::{env, path::Path};
use zed_extension_api::{self as zed, Command, ContextServerId, Result};

const PACKAGE_NAME: &str = "postgres-context-server";
const PACKAGE_VERSION: &str = "0.1.0";
const SERVER_PATH: &str = "node_modules/postgres-context-server/index.mjs";
const PACKAGE_TARBALL_NAME: &str = "postgres-context-server.tgz";

struct PostgresModelContextExtension;

impl zed::Extension for PostgresModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(&mut self, _context_server_id: &ContextServerId) -> Result<Command> {
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(PACKAGE_VERSION) {
            // For now, install the context server from a vendored tarball.
            let src_path = env::var("ZED_EXTENSION_SRC_DIR")
                .map_err(|_| "failed to read extension src env var".to_string())?;
            let package_path = Path::new(&src_path).join(PACKAGE_TARBALL_NAME);
            let package_path = package_path.to_string_lossy();
            zed::npm_install_package(package_path.as_ref(), PACKAGE_VERSION)?;
        }

        Ok(Command {
            command: "node".to_string(),
            args: vec![env::current_dir()
                .unwrap()
                .join(SERVER_PATH)
                .to_string_lossy()
                .to_string()],
            env: Vec::new(),
        })
    }
}

zed::register_extension!(PostgresModelContextExtension);
