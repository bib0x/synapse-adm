mod cli;
mod config;
mod helper;
mod room;
mod user;

#[tokio::main]
async fn main() {
    let mut config = config::Config::new();

    let matches = cli::build_cli("neoctl").get_matches();

    match matches.subcommand() {

        Some(("room", sub_matches)) => {
            match sub_matches.subcommand() {
                Some(("ls", sub_matches)) => {
                    let order_by = sub_matches.get_one::<String>("order_by").unwrap();
                    let limit = sub_matches.get_one::<u64>("limit").unwrap();
                    let from = sub_matches.get_one::<u64>("from").unwrap();
                    let name = sub_matches.get_one::<String>("name"); 
                    room::Room::list_all_by(&config, *from, &order_by, *limit, name).await;
                },
                Some(("show", sub_matches)) => {
                    let room_id = sub_matches.get_one::<String>("room_id").unwrap();
                    room::Room::show_details(&config, &room_id).await;
                },
                Some(("members", sub_matches)) => {
                    let room_id = sub_matches.get_one::<String>("room_id").unwrap();
                    room::Room::list_members(&config, &room_id).await;
                },
                Some(("state", sub_matches)) => {
                    let room_id = sub_matches.get_one::<String>("room_id").unwrap();
                    room::Room::show_state(&config, &room_id).await;
                },
                Some(("isblocked", sub_matches)) => {
                    let room_id = sub_matches.get_one::<String>("room_id").unwrap();
                    room::Room::is_blocked(&config, &room_id).await;
                },
                Some(("block", sub_matches)) => {
                    let room_id = sub_matches.get_one::<String>("room_id").unwrap();
                    room::Room::block(&config, &room_id, room::BLOCKED).await;
                },
                Some(("unblock", sub_matches)) => {
                    let room_id = sub_matches.get_one::<String>("room_id").unwrap();
                    room::Room::block(&config, &room_id, room::UNBLOCKED).await;
                },
                Some(("setadm", sub_matches)) => {
                    let room_id = sub_matches.get_one::<String>("room_id").unwrap();
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    room::Room::promote_user_as_admin(&config, &room_id, &user_id).await;
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
                    user::User::list_all_by(&config, *from, &order_by, *limit, *guests, *deactivated, name, user_id).await;
                },
                Some(("show", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();

                    config.version = 2;
                    user::User::show_details(&config, &user_id).await;
                },
                Some(("deactivate", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::deactivate(&config, &user_id).await;
                },
                Some(("sessions", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::whois(&config, &user_id).await;
                },
                Some(("isadmin", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::isadmin(&config, &user_id).await;
                },
                Some(("promote", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::setadmin_server(&config, &user_id, true).await;
                },
                Some(("retrograde", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::setadmin_server(&config, &user_id, false).await;
                },
                Some(("rooms", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::joined_rooms(&config, &user_id).await;
                },
                Some(("accountdata", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::account_data(&config, &user_id).await;
                },
                Some(("medias", sub_matches)) => {
                    let order_by = sub_matches.get_one::<String>("order_by").unwrap();
                    let limit = sub_matches.get_one::<u64>("limit").unwrap();
                    let from = sub_matches.get_one::<u64>("from").unwrap();
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::list_medias(&config, *from, &order_by, *limit, &user_id).await;
                },
                Some(("devices", sub_matches)) => {
                    let device_id = sub_matches.get_one::<String>("device_id");
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    config.version = 2;
                    user::User::list_devices(&config, &user_id, device_id).await;
                },
                Some(("pushers", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::list_pushers(&config, &user_id).await;
                },
                Some(("ban", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::shadow_ban(&config, &user_id).await;
                },
                Some(("unban", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::shadow_unban(&config, &user_id).await;
                },
                Some(("ratelimit", sub_matches)) => {
                    let message_limit = sub_matches.get_one::<u64>("message");
                    let burst_count = sub_matches.get_one::<u64>("burst");
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::ratelimit(&config, message_limit, burst_count, &user_id).await;
                },
                Some(("unratelimit", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::unratelimit(&config, &user_id).await;
                },
                Some(("isavailable", sub_matches)) => {
                    let username = sub_matches.get_one::<String>("username").unwrap();
                    user::User::isavailable(&config, &username).await;
                },
                Some(("login", sub_matches)) => {
                    let user_id = sub_matches.get_one::<String>("user_id").unwrap();
                    user::User::loginas(&config, &user_id).await;
                },
                _ => unreachable!(),
            }
        },

        _ => unreachable!(),
    }

}
