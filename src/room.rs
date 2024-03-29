use std::{error::Error, fmt};

use crate::config::Config;
use crate::helper;

use synapse_adm::http;
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

#[derive(Debug)]
pub struct ParseJsonError;
impl Error for ParseJsonError {}
impl fmt::Display for  ParseJsonError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid or empty JSON data")
    }
}

impl Room {

    pub async fn list_all_by(config: &Config, from: u64, order_by: &str, limit: u64, name: Option<&String>) {
        let target = if let Some(name) = name {
            format!("rooms?from={}&limit={}&order_by={}&search_term={}", from, limit, order_by, name)
        } else if order_by == self::DEF_ORDER_BY && limit == self::DEF_LIMIT && from == self::DEF_FROM {
            String::from("rooms")
        } else {
            format!("rooms?from={}&limit={}&order_by={}", from, limit, order_by)
        };
        http!(GET &target, &config);
    }

    pub async fn show_details(config: &Config, room_id: &str) {
        let target = format!("rooms/{}", room_id);
        http!(GET &target, &config);
    }

    pub async fn list_members(config: &Config, room_id: &str) {
        let target = format!("rooms/{}/members", room_id);
        http!(GET &target, &config);
    }

    pub async fn show_state(config: &Config, room_id: &str) {
        let target = format!("rooms/{}/state", room_id);
        http!(GET &target, &config);
    }

    pub async fn is_blocked(config: &Config, room_id: &str) {
        let target = format!("rooms/{}/block", room_id);
        http!(GET &target, &config);
    }

    pub async fn block(config: &Config, room_id: &str, block_status_wanted: bool) -> Result<(), Box<dyn Error>> {
        let target = format!("rooms/{}/block", room_id);
        let client = helper::HttpClient::new(&config, &target);
        let status = client.get()
                          .await?
                          .json::<RoomBlockStatus>()
                          .await
                          .map_err(|_| ParseJsonError)?;

        if block_status_wanted == self::BLOCKED {
            if status.block == self::BLOCKED {
                let message = format!("room {} already blocked", room_id);
                println!("{}", serde_json::json!({"message": message}))
            } else {
               let target = format!("rooms/{}/block", room_id);
               let body = RoomBlockRequest{ block: true };
               http!(PUT &target, &config, &body);
            }
        } else {
            if status.block == self::UNBLOCKED {
                let message = format!("room {} already unblocked", room_id);
                println!("{}", serde_json::json!({"message": message}))
            } else {
               let target = format!("rooms/{}/block", room_id);
               let body = RoomBlockRequest{ block: false };
               http!(PUT &target, &config, &body);
            }
        }
        Ok(())
    }

    pub async fn promote_user_as_admin(config: &Config, room_id: &str, user_id: &str) {
        let target = format!("rooms/{}/make_room_admin", room_id);
        let body = RoomSetAdminRequest{ user_id };
        http!(POST &target, &config, &body);
    }

}
