use crate::config::Config;
use crate::util;

use reqwest;
use serde::{Deserialize, Serialize};

pub struct Room {
    pub name: String,
    pub order_by: String,
    pub from: u64,
}

pub const DEF_ORDER_BY: &'static str = "name";
pub const DEF_LIMIT: u64 = 100;
pub const DEF_FROM: u64 = 0;

pub const BLOCKED : bool = true;
pub const UNBLOCKED : bool = false;


#[derive(Deserialize)]
struct RoomBlockStatus {
    block: bool,
}

#[derive(Serialize)]
struct RoomBlockRequest {
    block: bool
}

#[derive(Serialize)]
struct RoomSetAdminRequest<'a> {
    user_id: &'a str,
}

impl Room {

    pub fn list_all_by(config: &Config, from: u64, order_by: &str, limit: u64, name: Option<&String>) {
        let target = if let Some(name) = name {
            format!("rooms?from={}&limit={}&order_by={}&search_term={}", from, limit, order_by, name)
        } else if order_by == self::DEF_ORDER_BY && limit == self::DEF_LIMIT && from == self::DEF_FROM {
            String::from("rooms")
        } else {
            format!("rooms?from={}&limit={}&order_by={}", from, limit, order_by)
        };
        util::http!(GET &target, &config);
    }

    pub fn show_details(config: &Config, room_id: &str) {
        let target = format!("rooms/{}", room_id);
        util::http!(GET &target, &config);
    }

    pub fn list_members(config: &Config, room_id: &str) {
        let target = format!("rooms/{}/members", room_id);
        util::http!(GET &target, &config);
    }

    pub fn show_state(config: &Config, room_id: &str) {
        let target = format!("rooms/{}/state", room_id);
        util::http!(GET &target, &config);
    }

    pub fn is_blocked(config: &Config, room_id: &str) {
        let target = format!("rooms/{}/block", room_id);
        util::http!(GET &target, &config);
    }

    pub fn block(config: &Config, room_id: &str, block_status_wanted: bool) {
        let target = format!("rooms/{}/block", room_id);
        // We will panic if we can't get a HTTP response
        // XXX don't panic !
        let status = match util::http_get_request(&target, &config) {
            Ok(response) => {
                if response.status() == reqwest::StatusCode::OK {
                    match response.json::<RoomBlockStatus>() {
                        Ok(data) => data,
                        _ => panic!("[-] No HTTP response body found."),
                    }
                } else {
                    panic!("[-]  No HTTP response body found.");
                }
            },
            Err(_) => panic!("[-] No HTTP response body found."),
        };
        
        if block_status_wanted == self::BLOCKED {
            if status.block == self::BLOCKED {
                let message = format!("room {} already blocked", room_id);
                util::json_stdout!(message);
            } else {
               let target = format!("rooms/{}/block", room_id);
               let body = RoomBlockRequest{ block: true };
               util::http!(PUT &target, &config, &body);
            }
        } else {
            if status.block == self::UNBLOCKED {
                let msg = format!("room {} already unblocked", room_id);
                util::json_stdout!(msg);
            } else {
               let target = format!("rooms/{}/block", room_id);
               let body = RoomBlockRequest{ block: false };
               util::http!(PUT &target, &config, &body);
            }
        }
    }

    pub fn promote_user_as_admin(config: &Config, room_id: &str, user_id: &str) {
        let target = format!("rooms/{}/make_room_admin", room_id);
        let body = RoomSetAdminRequest{ user_id };
        util::http!(POST &target, &config, &body);
    }

}
