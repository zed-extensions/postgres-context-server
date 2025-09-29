use schemars::JsonSchema;
use serde::Deserialize;
use std::env;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "@zeddotdev/postgres-context-server";
const SERVER_PATH: &str = "node_modules/@zeddotdev/postgres-context-server/index.mjs";

struct PostgresModelContextExtension;

#[derive(Debug, Deserialize, JsonSchema)]
struct PostgresContextServerSettings {
    database_url: String,
}

impl zed::Extension for PostgresModelContextExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        let latest_version = zed::npm_package_latest_version(PACKAGE_NAME)?;
        let version = zed::npm_package_installed_version(PACKAGE_NAME)?;
        if version.as_deref() != Some(latest_version.as_ref()) {
            zed::npm_install_package(PACKAGE_NAME, &latest_version)?;
        }

        let settings = ContextServerSettings::for_project("postgres-context-server", project)?;
        let Some(settings) = settings.settings else {
            return Err("missing `database_url` setting".into());
        };
        let settings: PostgresContextServerSettings =
            serde_json::from_value(settings).map_err(|e| e.to_string())?;

        let node_path = zed::node_binary_path()?;
        let server_path = env::current_dir()
            .unwrap()
            .join(SERVER_PATH)
            .to_string_lossy()
            .to_string();

        Ok(Command {
            command: node_path,
            args: vec![server_path],
            env: vec![("DATABASE_URL".into(), settings.database_url)],
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        _project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();
        let default_settings = include_str!("../configuration/default_settings.jsonc").to_string();
        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(PostgresContextServerSettings))
                .map_err(|e| e.to_string())?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(PostgresModelContextExtension);
