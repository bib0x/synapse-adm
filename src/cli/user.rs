use clap::{Arg, Command};

pub fn build_user_commands() -> Command <'static> {
    Command::new("user")
        .subcommand(
            Command::new("ls")
                .about("List Matrix users")
                .arg(
                    Arg::new("deactivated")
                    .long("deactivated")
                    .short('d')
                    .action(clap::ArgAction::SetTrue)
                    .help("Include deactivated users from the search.")
                )
                .arg(
                    Arg::new("from")
                        .long("from")
                        .short('f')
                        .value_parser(clap::value_parser!(u64))
                        .default_value("0")
                        .takes_value(true)
                        .help("Offset in the returned list.")
                )
                .arg(
                    Arg::new("guests")
                    .long("no-guests")
                    .action(clap::ArgAction::SetFalse)
                    .help("Exclude guest users from the search.")
                )
                .arg(
                    Arg::new("limit")
                        .long("limit")
                        .short('l')
                        .value_parser(clap::value_parser!(u64))
                        .default_value("100")
                        .takes_value(true)
                        .help("Maximum amount of users to return.")
                )
                .arg(
                    Arg::new("name")
                        .long("name")
                        .short('n')
                        .takes_value(true)
                        .help("Filter users with user id or displaynames.")
                )
                .arg(
                    Arg::new("order_by")
                        .long("order_by")
                        .short('o')
                        .default_value("name")
                        .takes_value(true)
                        .value_parser([
                          "name", "is_guest", "admin", "user_type", "deactivated",
                          "shadow_banned", "displayname", "avatar_url", "creation_ts"
                        ])
                        .help("Sort the returned list of users.")
                )
                .arg(
                    Arg::new("user_id")
                    .long("user")
                    .short('u')
                    .takes_value(true)
                    .help("Filter users with only user id.")
                )
        )
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
        .subcommand(
            Command::new("deactivate")
                .about("Deactivate Matrix user")
                .arg(
                    Arg::new("user_id")
                    .long("user")
                    .short('u')
                    .takes_value(true)
                    .required(true)
                    .help("user identifier")
                )
        )
}
