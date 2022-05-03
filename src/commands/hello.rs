//! `hello` subcommand - example of how to write a subcommand

/// App-local prelude includes `app_reader()`/`app_writer()`/`app_config()`
/// accessors along with logging macros. Customize as you see fit.
use crate::prelude::*;

use crate::config::Myex2Config;
use abscissa_core::{config, Command, FrameworkError, Runnable};
use clap::Parser;

/// `hello` subcommand
///
/// The `Parser` proc macro generates an option parser based on the struct
/// definition, and is defined in the `clap` crate. See their documentation
/// for a more comprehensive example:
///
/// <https://docs.rs/clap/>
#[derive(Command, Debug, Parser)]
pub struct Subcommand {
    /// remove prefix url
    #[clap(long, parse(from_flag))]
    deprefix: bool,
    /// To whom are we saying hello?
    recipient: Vec<String>,
}

impl Runnable for Subcommand {
    /// Start the application.
    fn run(&self) {
        let config = APP.config();
        println!("Hello, {:?} {:?}", &config, self);
    }
}

impl config::Override<Myex2Config> for Subcommand {
    // Process the given command line options, overriding settings from
    // a configuration file using explicit flags taken from command-line
    // arguments.
    fn override_config(&self, mut config: Myex2Config) -> Result<Myex2Config, FrameworkError> {
        if !self.recipient.is_empty() {
            config.hello.recipient = self.recipient.join(" ");
        }

        Ok(config)
    }
}
