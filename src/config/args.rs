//! This module provides arguments related to the user config.

use clap::{Arg, ArgMatches};

const ARG_CONFIG: &str = "config";

/// Represents the config file path argument. This argument allows the
/// user to customize the config file path.
pub fn arg() -> Arg {
    Arg::new(ARG_CONFIG)
        .help("Set a custom configuration file path")
        .long("config")
        .short('c')
        .global(true)
        .value_name("PATH")
}

/// Represents the config file path argument parser.
pub fn parse_arg(matches: &ArgMatches) -> Option<&str> {
    matches.get_one::<String>(ARG_CONFIG).map(String::as_str)
}
