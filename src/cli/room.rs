use clap::{Arg, Command};

pub fn build_room_commands() -> Command {
    Command::new("room")
        .subcommand(
            Command::new("ls")
                .about("List Matrix server rooms")
                .arg(
                    Arg::new("from")
                        .long("from")
                        .short('f')
                        .value_parser(clap::value_parser!(u64))
                        .default_value("0")
                        .help("Offset in the returned list.")
                )
                .arg(
                    Arg::new("limit")
                        .long("limit")
                        .short('l')
                        .value_parser(clap::value_parser!(u64))
                        .default_value("100")
                        .help("Maximum amount of rooms to return")
                )
                .arg(
                    Arg::new("name")
                        .long("name")
                        .short('n')
                        .help("Filter rooms by their room name, canonical alias and room id")
                )
                .arg(
                    Arg::new("order_by")
                        .long("order_by")
                        .short('o')
                        .default_value("name")
                        .value_parser([
                          "alphabetical", "size", "name", "canonical_alias",
                          "joined_members", "joined_local_members", "version",
                          "creator", "encryption", "federatable", "public",
                          "join_rules", "guest_access", "history_visibility", "state_events"
                        ])
                        .help("Sort the returned list of rooms")
                )
        )
        .subcommand(
            Command::new("show")
            .about("Show Matrix room details")
            .arg(
                Arg::new("room_id")
                .long("id")
                .short('i')
                .required(true)
                .help("Room identifier")
            )
        )
        .subcommand(
            Command::new("members")
            .about("Show Matrix room members")
            .arg(
                Arg::new("room_id")
                .long("id")
                .short('i')
                .required(true)
                .help("Room identifier")
            )
        )
        .subcommand(
            Command::new("state")
            .about("Show Matrix room state")
            .arg(
                Arg::new("room_id")
                .long("id")
                .short('i')
                .required(true)
                .help("Room identifier")
            )
        )
        .subcommand(
            Command::new("isblocked")
            .about("Show Matrix room block status")
            .arg(
                Arg::new("room_id")
                .long("id")
                .short('i')
                .required(true)
                .help("Room identifier")
            )
        )
        .subcommand(
            Command::new("block")
            .about("Block Matrix room block")
            .arg(
                Arg::new("room_id")
                .long("id")
                .short('i')
                .required(true)
                .help("Room identifier")
            )
        )
        .subcommand(
            Command::new("unblock")
            .about("Unblock Matrix room block")
            .arg(
                Arg::new("room_id")
                .long("id")
                .short('i')
                .required(true)
                .help("Room identifier")
            )
        )
        .subcommand(
            Command::new("setadm")
            .about("Promote a user as room administrator")
            .arg(
                Arg::new("room_id")
                .long("id")
                .short('i')
                .required(true)
                .help("Room identifier")
            )
            .arg(
                Arg::new("user_id")
                .long("user")
                .short('u')
                .required(true)
                .help("User identifier")
            )
        )
}
