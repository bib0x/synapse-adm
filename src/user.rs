use crate::config::Config;
use crate::util;

use serde::Serialize;

pub struct User;

#[derive(Serialize)]
pub struct UserDeactivateBody {
    erase: bool
}

#[derive(Serialize)]
pub struct UserAdminPromotionBody {
    admin: bool
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

        util::http_get!(&target, &config);
    }

    pub fn show_details(config: &Config, user_id: &str) {
        let target = format!("users/{}", user_id);
        util::http_get!(&target, &config);
    }

    pub fn deactivate(config: &Config, user_id: &str) {
        let target = format!("deactivate/{}", user_id);
        let body = UserDeactivateBody { erase: false };
        util::http_post!(&target, &config, &body);
    }

    pub fn whois(config: &Config, user_id: &str) {
        let target = format!("whois/{}", user_id);
        util::http_get!(&target, &config);
    }

    pub fn isadmin(config: &Config, user_id: &str) {
        let target = format!("users/{}/admin", user_id);
        util::http_get!(&target, &config);
    }

    pub fn setadmin_server(config: &Config, user_id: &str, admin: bool) {
        let target = format!("users/{}/admin", user_id);
        let body = UserAdminPromotionBody { admin }; 
        util::http_put!(&target, &config, &body);
    }

    pub fn joined_rooms(config: &Config, user_id: &str) {
        let target = format!("users/{}/joined_rooms", user_id);
        util::http_get!(&target, &config);
    }
    
    pub fn account_data(config: &Config, user_id: &str) {
        let target = format!("users/{}/accountdata", user_id);
        util::http_get!(&target, &config);
    }

    pub fn list_medias(config: &Config, from: u64, order_by: &str, limit: u64, user_id: &str) {
        let target = format!("users/{}/media?from={}&order_by={}&limit={}", user_id, from, order_by, limit);
        util::http_get!(&target, &config);
    }

    pub fn list_devices(config: &Config, user_id: &str, device_id: Option<&String>) {
        let target = if let Some(device_id) = device_id {
            format!("users/{}/devices/{}", user_id, device_id)
        } else {
            format!("users/{}/devices", user_id)
        };
        util::http_get!(&target, &config);
    }

}
