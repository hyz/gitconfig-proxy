//! `start` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;

use crate::config::GixConfig;
use abscissa_core::{config, Command, FrameworkError, Runnable};
use clap::Parser;

/// `start` subcommand
///
/// The `Parser` proc macro generates an option parser based on the struct
/// definition, and is defined in the `clap` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/clap/>
#[derive(Command, Debug, Parser)]
pub struct StartCmd {
    /// To whom are we saying hello?
    recipient: Vec<String>,
}

impl Runnable for StartCmd {
    /// Start the application.
    fn run(&self) {
        let config = APP.config();
        println!("Hello, {}!", &config.example.recipient);
    }
}

impl config::Override<GixConfig> for StartCmd {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(&self, mut config: GixConfig) -> Result<GixConfig, FrameworkError> {
        if !self.recipient.is_empty() {
            config.example.recipient = self.recipient.join(" ");
        }

        Ok(config)
    }
}
