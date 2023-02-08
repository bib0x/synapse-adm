use crate::config::Config;
use crate::helper;

use neoctl::http_bis;
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

    pub async fn list_all_by(
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
               format!("users?from={}&limit={}&order_by={}&guests={}&deactivated={}", 
                       from, limit, order_by, guests, deactivated) 
            }
        };

        http_bis!(GET &target, &config);
    }

    pub async fn show_details(config: &Config, user_id: &str) {
        let target = format!("users/{}", user_id);
        http_bis!(GET &target, &config);
    }

    pub async fn deactivate(config: &Config, user_id: &str) {
        let target = format!("deactivate/{}", user_id);
        let body = UserDeactivateBody { erase: false };
        http_bis!(POST &target, &config, &body);
    }

    pub async fn whois(config: &Config, user_id: &str) {
        let target = format!("whois/{}", user_id);
        http_bis!(GET &target, &config);
    }

    pub async fn isadmin(config: &Config, user_id: &str) {
        let target = format!("users/{}/admin", user_id);
        http_bis!(GET &target, &config);
    }

    pub async fn setadmin_server(config: &Config, user_id: &str, admin: bool) {
        let target = format!("users/{}/admin", user_id);
        let body = UserAdminPromotionBody { admin }; 
        http_bis!(PUT &target, &config, &body);
    }

    pub async fn joined_rooms(config: &Config, user_id: &str) {
        let target = format!("users/{}/joined_rooms", user_id);
        http_bis!(GET &target, &config);
    }
    
    pub async fn account_data(config: &Config, user_id: &str) {
        let target = format!("users/{}/accountdata", user_id);
        http_bis!(GET &target, &config);
    }

    pub async fn list_medias(config: &Config, from: u64, order_by: &str, limit: u64, user_id: &str) {
        let target = format!("users/{}/media?from={}&order_by={}&limit={}", user_id, from, order_by, limit);
        http_bis!(GET &target, &config);
    }

    pub async fn list_devices(config: &Config, user_id: &str, device_id: Option<&String>) {
        let target = if let Some(device_id) = device_id {
            format!("users/{}/devices/{}", user_id, device_id)
        } else {
            format!("users/{}/devices", user_id)
        };
        http_bis!(GET &target, &config);
    }

    pub async fn list_pushers(config: &Config, user_id: &str) {
        let target = format!("users/{}/pushers", user_id);
        http_bis!(GET &target, &config);
    }

    pub async fn shadow_ban(config: &Config, user_id: &str) {
        let target = format!("users/{}/shadow_ban", user_id);
        let body = UserEmptyBody{};
        http_bis!(POST &target, &config, &body);
    }

    pub async fn shadow_unban(config: &Config, user_id: &str) {
        let target = format!("users/{}/shadow_ban", user_id);
        let body = UserEmptyBody{};
        http_bis!(DELETE &target, &config, &body);
    }

    pub async fn ratelimit(config: &Config, message_limit: Option<&u64>, burst_count: Option<&u64>, user_id: &str) {
        let target = format!("users/{}/override_ratelimit", user_id);
        if message_limit.is_none() && burst_count.is_none() {
            http_bis!(GET &target, &config);
        } else {
           let body = if message_limit.is_some() && burst_count.is_some() {
                UserRatelimitBody{ messages_per_second: *message_limit.unwrap(), burst_count: *burst_count.unwrap() }
           } else {
                let client = helper::HttpClient::new(&config, &target);
                // Get ratelimit currenlty set
                let mut rate_limit = match client.get().await {
                    Ok(response) => { 
                        if response.status() == reqwest::StatusCode::OK {
                            match response.json::<UserRatelimitBody>().await {
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
           http_bis!(POST &target, &config, &body);
        }
    }

    pub async fn unratelimit(config: &Config, user_id: &str) {
        let target = format!("users/{}/override_ratelimit", user_id);
        let body = UserEmptyBody{};
        http_bis!(DELETE &target, &config, &body);
    }

    pub async fn isavailable(config: &Config, username: &str) {
        let target = format!("username_available?username={}", username);
        http_bis!(GET &target, &config);
    }

    pub async fn loginas(config: &Config, user_id: &str) {
        let target = format!("users/{}/login", user_id);
        let body = UserEmptyBody{};
        http_bis!(POST &target, &config, &body);
    }

}
