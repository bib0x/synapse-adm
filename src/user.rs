use crate::config::Config;
use crate::util;

use serde::Serialize;

pub struct User;

#[derive(Serialize)]
pub struct UserDeactivateBody {
    erase: bool
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

}
