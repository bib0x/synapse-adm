use crate::config::Config;
use crate::util;

//use reqwest;
//use serde::{Deserialize, Serialize};

pub struct User;

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

}
