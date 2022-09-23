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
                Some(("ls", sub_matches)) => {
                    let order_by = sub_matches.get_one::<String>("order_by").unwrap();
                    let limit = sub_matches.get_one::<u64>("limit").unwrap();
                    let from = sub_matches.get_one::<u64>("from").unwrap();
                    let guests = sub_matches.get_one::<bool>("guests").unwrap();
                    let deactivated= sub_matches.get_one::<bool>("deactivated").unwrap();
                    let name = sub_matches.get_one::<String>("name");
                    let user_id = sub_matches.get_one::<String>("user_id");

                    config.version = 2;
                    user::User::list_all_by(&config, *from, &order_by, *limit, *guests, *deactivated, name, user_id);
                },
                Some(("show", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();

                    config.version = 2;
                    user::User::show_details(&config, &user_id);
                },
                Some(("deactivate", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::deactivate(&config, &user_id);
                },
                Some(("sessions", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::whois(&config, &user_id);
                },
                Some(("isadmin", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::isadmin(&config, &user_id);
                },
                Some(("promote", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::setadmin_server(&config, &user_id, true);
                },
                Some(("retrograde", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::setadmin_server(&config, &user_id, false);
                },
                Some(("rooms", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::joined_rooms(&config, &user_id);
                },
                Some(("accountdata", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::account_data(&config, &user_id);
                },
                Some(("medias", sub_matches)) => {
                    let order_by = sub_matches.get_one::<String>("order_by").unwrap();
                    let limit = sub_matches.get_one::<u64>("limit").unwrap();
                    let from = sub_matches.get_one::<u64>("from").unwrap();
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::list_medias(&config, *from, &order_by, *limit, &user_id);
                },
                Some(("devices", sub_matches)) => {
                    let device_id = sub_matches.get_one::<String>("device_id");
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    config.version = 2;
                    user::User::list_devices(&config, &user_id, device_id);
                },
                Some(("pushers", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::list_pushers(&config, &user_id);
                },
                Some(("ban", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::shadow_ban(&config, &user_id);
                },
                Some(("unban", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::shadow_unban(&config, &user_id);
                },
                Some(("ratelimit", sub_matches)) => {
                    let message_limit = sub_matches.get_one::<u64>("message");
                    let burst_count = sub_matches.get_one::<u64>("burst");
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::ratelimit(&config, message_limit, burst_count, &user_id);
                },
                Some(("unratelimit", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::unratelimit(&config, &user_id);
                },
                Some(("isavailable", sub_matches)) => {
                    let username = sub_matches.get_one::<String>("username").unwrap();
                    user::User::isavailable(&config, &username);
                },
                Some(("login", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::loginas(&config, &user_id);
                },
                _ => unreachable!(),
            }
        },

        _ => unreachable!(),
    }

}
