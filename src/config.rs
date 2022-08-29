pub struct Config {
    pub hostname: String,
    pub port: u32,
    pub token: String,
    pub version: u8,
}


impl Config {

    pub fn new(hostname: &str, port: u32, token: &str, version: u8) -> Self {
        Config {
            hostname: hostname.to_string(),
            port,
            token: token.to_string(),
            version
        }
    }

}
