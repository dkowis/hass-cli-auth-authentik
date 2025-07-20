mod authentik;
mod config;

use crate::authentik::Authentik;
use crate::config::Config;
use crate::prelude::*;
use anyhow::bail;
use clap::Parser;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod prelude {
    pub use anyhow::{Context, Result};
    pub use tracing::{debug, trace};
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

fn init_tracing(verbosity: clap_verbosity_flag::Verbosity) {
    tracing_subscriber::registry()
        .with(
            fmt::layer()
                .with_writer(std::io::stderr)
                .with_file(true)
                .with_thread_ids(true),
        )
        .with(verbosity.tracing_level_filter())
        .init();
}

//Single threaded is fine in this case
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let args = Cli::parse();
    init_tracing(args.verbosity);

    trace!("{:?}", args);
    //Locate and load in the config.toml.
    trace!("Loading config file from path: {}", args.config_path);

    let config = Config::load(&args.config_path)?;
    trace!("{:?}", config);

    let optional_user =
        Authentik::authenticate_user(&config, args.username.clone(), args.password.clone()).await?;

    // https://www.home-assistant.io/docs/authentication/providers/#command-line
    if let Some(user) = optional_user {
        enum Role {
            Admin,
            User,
            None,
        }

        let admin_group = config.admin_group_name.as_ref();
        let user_group = config.user_group_name.as_ref();

        let is_admin = admin_group.is_some_and(|g| user.groups.contains(g));
        let is_user = user_group.is_some_and(|g| user.groups.contains(g));

        let role = if is_admin {
            Role::Admin
        } else if is_user {
            Role::User
        } else {
            Role::None
        };

        // Logic for rejecting user if configured groups exist but user in none
        let any_group_defined = admin_group.is_some() || user_group.is_some();
        let user_in_any_group = is_admin || is_user;

        if any_group_defined && !user_in_any_group {
            // Reject user: user is not in any of the defined groups
            bail!("User is not a member of any required group");
        }
        //No groups are defined, it should still allow authentication
        println!("username = {}", user.display_name);

        match role {
            Role::Admin => println!("group = system-admin"),
            Role::User => println!("group = system-users"),
            _ => (),
        };

        Ok(())
    } else {
        //NO user returned! it's not legit!
        bail!("Did not successfully authenticate")
    }
}
