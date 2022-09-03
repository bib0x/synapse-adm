use clap::{Arg, Command};

pub fn build_user_commands() -> Command <'static> {
    Command::new("user")
        .subcommand(
            Command::new("show")
                .about("Show Matrix user details")
                .arg(
                    Arg::new("user_id")
                    .long("user")
                    .short('u')
                    .takes_value(true)
                    .required(true)
                    .help("User identifier")
                )
        )
}
