mod config;
mod ldap;
mod rfc4511;

use anyhow::{anyhow, bail};
use clap::Parser;
use crate::config::Config;
use crate::ldap::UserChecking;
use crate::prelude::*;


mod prelude {
    pub use anyhow::{Context, Result};
    pub use tracing::{debug, error, info, trace, warn};
}

#[derive(Debug, Parser)]
struct Cli {
    #[command(flatten)]
    verbosity: clap_verbosity_flag::Verbosity,

    #[arg(env = "username", help = "The username to login with")]
    username: String,

    #[arg(env = "password", help = "The password to login with")]
    password: String,

    #[arg(
        short,
        long,
        help = "path to the config file",
        default_value = "config.toml"
    )]
    config_path: String,
}

//Single threaded is fine in this case
fn main() -> Result<()> {
    let args = Cli::parse();

    tracing_subscriber::fmt()
        .with_max_level(args.verbosity)
        .init();

    trace!("{:?}", args);
    //Locate and load in the config.toml.
    trace!("Loading config file from path: {}", args.config_path);

    let config = Config::load(&args.config_path)?;
    trace!("{:?}", config);

    //check for user groups
    let user_checking = UserChecking::new(&config).context("Failed to create user checking")?;

    let user = user_checking.get_user(&args.username).context("Unable to get users' groups")?;

    trace!("Found one and only one user: {:?}", user);
    if let Some(user) = user {
        //Got a user
        let authenticated = user_checking.verify_credentials(&args.username, &args.password).context("Unable to verify credentials")?;
        trace!("Authenticated: {:?}", authenticated);

        if authenticated {
            println!("name = {}", user.display_name);
            return Ok(())
        }
    }
    //If we've fallen through, it's not authenticated!
    bail!("Did not successfully authenticate")
}
