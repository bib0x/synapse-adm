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
        match util::http_get_request(&target, &config) {
            Ok(response) => match response.text() {
                Ok(body)   => println!("{}", body),
                Err(_) => println!("[-] No HTTP response body found."),
            }
            Err(error) => println!("[-] {}", error),
        }
    }

    pub fn deactivate(config: &Config, user_id: &str) {
        let target = format!("deactivate/{}", user_id);
        let body = UserDeactivateBody { erase: false };
        match util::http_post_request(&target, &config, &body) {
            Ok(response) => match response.text() {
                Ok(body)   => println!("{}", body),
                Err(_) => println!("[-] No HTTP response body found."),
            }
            Err(error) => println!("[-] {}", error),
        }
    }

}
