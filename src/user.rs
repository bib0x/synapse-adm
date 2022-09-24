use crate::config::Config;
use crate::util;

use serde::{Deserialize,Serialize};

pub struct User;

#[derive(Serialize)]
pub struct UserDeactivateBody {
    erase: bool
}

#[derive(Serialize)]
pub struct UserAdminPromotionBody {
    admin: bool
}

#[derive(Serialize)]
pub struct UserEmptyBody;

#[derive(Deserialize,Serialize)]
pub struct UserRatelimitBody {
    messages_per_second: u64,
    burst_count: u64,
}

#[derive(Serialize)]
pub struct UserLogoutBody {
    access_token: String
}

impl User {

    pub fn list_all_by(
        config: &Config, 
        from: u64, 
        order_by: &str, 
        limit: u64, 
        guests: bool,
        deactivated: bool,
        name: Option<&String>,
        user_id: Option<&String>) {
        
        let target = if let Some(user_id) = user_id {
            format!("users?from={}&limit={}&order_by={}&user_id={}&guests={}&deactivated={}",
                    from, limit, order_by, user_id, guests, deactivated)
        } else {
           if let Some(name) = name {
                format!("users?from={}&limit={}&order_by={}&name={}&guests={}&deactivated={}", 
                        from, limit, order_by, name, guests, deactivated)
            } else {
               format!("rooms?from={}&limit={}&order_by={}&guests={}&deactivated={}", 
                       from, limit, order_by, guests, deactivated) 
            }
        };

        util::http!(GET &target, &config);
    }

    pub fn show_details(config: &Config, user_id: &str) {
        let target = format!("users/{}", user_id);
        util::http!(GET &target, &config);
    }

    pub fn deactivate(config: &Config, user_id: &str) {
        let target = format!("deactivate/{}", user_id);
        let body = UserDeactivateBody { erase: false };
        util::http!(POST &target, &config, &body);
    }

    pub fn whois(config: &Config, user_id: &str) {
        let target = format!("whois/{}", user_id);
        util::http!(GET &target, &config);
    }

    pub fn isadmin(config: &Config, user_id: &str) {
        let target = format!("users/{}/admin", user_id);
        util::http!(GET &target, &config);
    }

    pub fn setadmin_server(config: &Config, user_id: &str, admin: bool) {
        let target = format!("users/{}/admin", user_id);
        let body = UserAdminPromotionBody { admin }; 
        util::http!(PUT &target, &config, &body);
    }

    pub fn joined_rooms(config: &Config, user_id: &str) {
        let target = format!("users/{}/joined_rooms", user_id);
        util::http!(GET &target, &config);
    }
    
    pub fn account_data(config: &Config, user_id: &str) {
        let target = format!("users/{}/accountdata", user_id);
        util::http!(GET &target, &config);
    }

    pub fn list_medias(config: &Config, from: u64, order_by: &str, limit: u64, user_id: &str) {
        let target = format!("users/{}/media?from={}&order_by={}&limit={}", user_id, from, order_by, limit);
        util::http!(GET &target, &config);
    }

    pub fn list_devices(config: &Config, user_id: &str, device_id: Option<&String>) {
        let target = if let Some(device_id) = device_id {
            format!("users/{}/devices/{}", user_id, device_id)
        } else {
            format!("users/{}/devices", user_id)
        };
        util::http!(GET &target, &config);
    }

    pub fn list_pushers(config: &Config, user_id: &str) {
        let target = format!("users/{}/pushers", user_id);
        util::http!(GET &target, &config);
    }

    pub fn shadow_ban(config: &Config, user_id: &str) {
        let target = format!("users/{}/shadow_ban", user_id);
        let body = UserEmptyBody{};
        util::http!(POST &target, &config, &body);
    }

    pub fn shadow_unban(config: &Config, user_id: &str) {
        let target = format!("users/{}/shadow_ban", user_id);
        let body = UserEmptyBody{};
        util::http!(DELETE &target, &config, &body);
    }

    pub fn ratelimit(config: &Config, message_limit: Option<&u64>, burst_count: Option<&u64>, user_id: &str) {
        let target = format!("users/{}/override_ratelimit", user_id);
        if message_limit.is_none() && burst_count.is_none() {
            util::http!(GET &target, &config);
        } else {
           let body = if message_limit.is_some() && burst_count.is_some() {
                UserRatelimitBody{ messages_per_second: *message_limit.unwrap(), burst_count: *burst_count.unwrap() }
           } else {
                // Get ratelimit currenlty set
                let mut rate_limit = match util::get(&target, &config) {
                    Ok(response) => { 
                        if response.status() == reqwest::StatusCode::OK {
                            match response.json::<UserRatelimitBody>() {
                                Ok(data) => data,
                                _ => UserRatelimitBody{messages_per_second: 0, burst_count: 0},
                            }
                        } else {
                            panic!("[-]  No HTTP response body found.");
                        }
                    },
                    Err(_) => panic!("[-] No HTTP response body found."),
                };

                if message_limit.is_some() {
                    rate_limit.messages_per_second = *message_limit.unwrap();
                }

                if burst_count.is_some() {
                    rate_limit.burst_count = *burst_count.unwrap();
                }

                rate_limit
           };
           util::http!(POST &target, &config, &body);
        }
    }

    pub fn unratelimit(config: &Config, user_id: &str) {
        let target = format!("users/{}/override_ratelimit", user_id);
        let body = UserEmptyBody{};
        util::http!(DELETE &target, &config, &body);
    }

    pub fn isavailable(config: &Config, username: &str) {
        let target = format!("username_available?username={}", username);
        util::http!(GET &target, &config);
    }

    pub fn loginas(config: &Config, user_id: &str) {
        let target = format!("users/{}/login", user_id);
        let body = UserEmptyBody{};
        util::http!(POST &target, &config, &body);
    }

}
