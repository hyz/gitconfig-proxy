//! Myex2 Subcommands
//!
//! This is where you specify the subcommands of your application.
//!
//! The default application comes with two subcommands:
//!
//! - `start`: launches the application
//! - `--version`: print application version
//!
//! See the `impl Configurable` below for how to specify the path to the
//! application's configuration file.

mod clone;
mod prefix;
mod pull;
mod start;

use self::start::StartCmd;

use crate::config::GixConfig;
use abscissa_core::{config::Override, Command, Configurable, FrameworkError, Runnable};
use clap::Parser;
use directories::BaseDirs;
use std::path::{Path, PathBuf};

/// Myex2 Configuration Filename
pub const CONFIG_FILE: &str = "github-helper.toml";

/// Myex2 Subcommands
/// Subcommands need to be listed in an enum.
#[derive(Command, Debug, Parser, Runnable)]
pub enum Myex2Cmd {
    /// The `start` subcommand
    Start(StartCmd),
    /// The `clone` subcommand
    Clone(self::clone::Subcommand),
    /// The `pull` subcommand
    Pull(self::pull::Subcommand),
    /// The `prefix` subcommand
    Prefix(self::prefix::Subcommand),
}

/// Entry point for the application. It needs to be a struct to allow using subcommands!
#[derive(Command, Debug, Parser)]
#[clap(author, about, version)]
pub struct EntryPoint {
    #[clap(subcommand)]
    cmd: Myex2Cmd,

    /// Enable verbose logging
    #[clap(short, long)]
    pub verbose: bool,

    /// Use the specified config file
    #[clap(short, long)]
    pub config: Option<String>,
}

impl Runnable for EntryPoint {
    fn run(&self) {
        // println!("config-path {:?}", self.config_path());
        self.cmd.run()
    }
}

/// This trait allows you to define how application configuration is loaded.
impl Configurable<GixConfig> for EntryPoint {
    /// Location of the configuration file
    fn config_path(&self) -> Option<PathBuf> {
        self.config.as_ref().map(PathBuf::from).or_else(|| {
            BaseDirs::new()
                .and_then(|dirs| Some(dirs.config_dir().join(CONFIG_FILE)).filter(|f| f.exists()))
            //.or_else(|| Some(PathBuf::from(CONFIG_FILE)).filter(|f| f.exists()))
        })
    }

    /// Apply changes to the config after it's been loaded, e.g. overriding
    /// values in a config file using command-line options.
    ///
    /// This can be safely deleted if you don't want to override config
    /// settings from command-line options.
    fn process_config(&self, config: GixConfig) -> Result<GixConfig, FrameworkError> {
        match &self.cmd {
            Myex2Cmd::Start(cmd) => cmd.override_config(config),
            Myex2Cmd::Clone(cmd) => cmd.override_config(config),
            Myex2Cmd::Pull(cmd) => cmd.override_config(config),
            Myex2Cmd::Prefix(cmd) => cmd.override_config(config),
            //
            // If you don't need special overrides for some
            // subcommands, you can just use a catch all
            // _ => Ok(config),
        }
    }
}
