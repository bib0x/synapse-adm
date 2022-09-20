// Ideas: config file to choose API versions ? 

use std::env;

mod cli;
mod config;
mod room;
mod user;
mod util;

fn main() {

    // XXX don't panic
    let token = env::var("MATRIX_TOKEN").unwrap();
    let mut config = config::Config::new("localhost", 8080, &token, 1);

    let matches = cli::build_cli("neoctl").get_matches();

    match matches.subcommand() {

        Some(("room", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("ls", sub_matches)) => {
                    let order_by = sub_matches.get_one::<String>("order_by").unwrap();
                    let limit = sub_matches.get_one::<u64>("limit").unwrap();
                    let from = sub_matches.get_one::<u64>("from").unwrap();
                    let name = sub_matches.get_one::<String>("name"); 
                    room::Room::list_all_by(&config, *from, &order_by, *limit, name);
                },
                Some(("show", sub_matches)) => {
                    let room_id = sub_matches.get_one::<String>("room_id").unwrap();
                    room::Room::show_details(&config, &room_id);
                },
                Some(("members", sub_matches)) => {
                    let room_id = sub_matches.get_one::<String>("room_id").unwrap();
                    room::Room::list_members(&config, &room_id);
                },
                Some(("state", sub_matches)) => {
                    let room_id = sub_matches.get_one::<String>("room_id").unwrap();
                    room::Room::show_state(&config, &room_id);
                },
                Some(("isblocked", sub_matches)) => {
                    let room_id = sub_matches.get_one::<String>("room_id").unwrap();
                    room::Room::is_blocked(&config, &room_id);
                },
                Some(("block", sub_matches)) => {
                    let room_id = sub_matches.get_one::<String>("room_id").unwrap();
                    room::Room::block(&config, &room_id, room::BLOCKED);
                },
                Some(("unblock", sub_matches)) => {
                    let room_id = sub_matches.get_one::<String>("room_id").unwrap();
                    room::Room::block(&config, &room_id, room::UNBLOCKED);
                },
                Some(("setadm", sub_matches)) => {
                    let room_id = sub_matches.get_one::<String>("room_id").unwrap();
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    room::Room::promote_user_as_admin(&config, &room_id, &user_id);
                },
                _ => unreachable!(),
            }
        },

        Some(("user", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("show", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();

                    config.version = 2;
                    user::User::show_details(&config, &user_id);
                },
                Some(("deactivate", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();

                    user::User::deactivate(&config, &user_id);
                },
                _ => unreachable!(),
            }
        },

        _ => unreachable!(),
    }

}
