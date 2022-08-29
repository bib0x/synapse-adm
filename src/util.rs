use reqwest;
use reqwest::header;
use reqwest::header::HeaderValue;

use crate::config::Config;

pub fn get_endpoint(target: &str, config: &Config) -> String {
    format!("http://{}:{}/_synapse/admin/v{}/{}", config.hostname, config.port, config.version, target)
}

pub fn get_http_client(config: &Config) -> reqwest::Result<reqwest::blocking::Client> {
        let value = format!("Bearer {}", config.token);
        let mut token_value = HeaderValue::from_str(&value).unwrap();
        token_value.set_sensitive(true);

        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, token_value);

        reqwest::blocking::Client::builder()
            .default_headers(headers)
            .build()
}

pub fn do_simple_request(target: &str, config: &Config) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let endpoint = get_endpoint(target, &config);
        let client = get_http_client(&config)?;

        client.get(endpoint).send()
}
