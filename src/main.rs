mod app_config;
mod interact;
mod provider;

use anyhow::{Context, Result};
use clap::{crate_authors, crate_description, crate_version, App, AppSettings, Arg};

use crate::app_config::AppConfig;
use crate::provider::{get_provider, DEFAULT_PROVIDER, PROVIDERS};

fn main() -> Result<()> {
    const DOMAIN_ABOUT: &str = "the domain name (.rs is added if omitted)";
    let matches = App::new("rust-domain")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        // .subcommand(
        //     App::new("register")
        //         .about("registers a new .rs domain")
        //         .arg(Arg::new("domain").about(DOMAIN_ABOUT).required(true)),
        // )
        // .subcommand(
        //     App::new("ditch")
        //         .about("ditches a .rs domain of yours")
        //         .arg(Arg::new("domain").about(DOMAIN_ABOUT).required(true)),
        // )
        // .subcommand(
        //     App::new("renew")
        //         .about("renews a .rs domain of yours")
        //         .arg(Arg::new("domain").about(DOMAIN_ABOUT).required(true)),
        // )
        // .subcommand(
        //     App::new("transfer")
        //         .about("transfers a .rs domain of yours")
        //         .arg(Arg::new("domain").about(DOMAIN_ABOUT).required(true)),
        // )
        // .subcommand(
        //     App::new("ls")
        //         .about("list all your .rs domains")
        // )
        .subcommand(
            App::new("check")
                .about("check if a .rs domain is available")
                .arg(Arg::new("domain").about(DOMAIN_ABOUT).required(true)),
        )
        .subcommand(
            App::new("signup")
                .about("signup with the ISP an get started")
                .arg(
                    Arg::new("provider")
                        .about("the ISP that carries out the services")
                        .possible_values(&PROVIDERS),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("signup", args)) => {
            let provider = args.value_of("provider").unwrap_or(DEFAULT_PROVIDER);
            let api = get_provider(provider).context("The given provider is not supported.")?;
            api.signup()?;
        }
        Some(("check", args)) => {
            let config = AppConfig::load()?;
            let domain = args
                .value_of("domain")
                .context("Domain needs to be provided.")?;
            let api = get_provider(config.provider.as_str())
                .context("The given provider is not supported.")?;
            let available = api.check_domain(domain)?;
            println!(
                "The domain '{}' is {}",
                domain,
                match available {
                    true => "available",
                    false => "already taken!",
                }
            )
        }
        Some(_) => {
            println!("... this feature will come soon");
        }
        _ => {}
    }

    Ok(())
}
