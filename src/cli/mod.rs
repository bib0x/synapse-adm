use clap::Command;

pub mod room;
pub mod user;


pub fn build_cli(program_name: &str) -> Command<'static> {
    Command::new(program_name)
        .about("CLI to consume Synapse API for Matrix server")
        .subcommand(room::build_room_commands())
        .subcommand(user::build_user_commands())
}
