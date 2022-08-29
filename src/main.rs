// Ideas
// - environment variable for API TOKEN, API version

use std::env;

mod cli;
mod config;
mod room;
mod util;

fn main() {

    let token = env::var("MATRIX_TOKEN").unwrap();
    let config = config::Config::new("localhost", 8080, &token, 1);

    let matches = cli::build_cli("neoctl").get_matches();

    match matches.subcommand() {
        Some(("room", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("ls", _sub_matches)) => {
                    room::Room::find_all(&config);
                },
                _ => unreachable!(),

            }
        },
        _ => unreachable!(),
    }

}
