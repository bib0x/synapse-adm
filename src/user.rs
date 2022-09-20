use crate::config::Config;
use crate::util;

use serde::Serialize;

pub struct User;

#[derive(Serialize)]
pub struct UserDeactivateBody {
    erase: bool
}

impl User {

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
