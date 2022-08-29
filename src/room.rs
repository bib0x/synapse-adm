use crate::config::Config;
use crate::util;

pub struct Room;

impl Room {

    pub fn find_all(config: &Config) {
        match util::do_simple_request("rooms", &config) {
            Ok(response) => match response.text() {
                Ok(body)   => println!("{}", body),
                Err(_) => println!("[-] No HTTP response body found."),
            }
            Err(error) => println!("[-] {}", error),
        }
    }

}
