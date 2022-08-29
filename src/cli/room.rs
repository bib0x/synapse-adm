use clap::{Arg, Command};

pub fn build_room_commands() -> Command <'static> {
    Command::new("room")
        .subcommand(
            Command::new("ls")
                .about("List Matrix server rooms")
                .arg(
                    Arg::new("Room's name, id or alias").long("name").short('n')
                )
        )
}
