use reqwest;
use reqwest::header;
use reqwest::header::HeaderValue;

use crate::config::Config;

#[macro_export]
macro_rules! http{
    (GET $target:expr,$config:expr) => {
        let result = util::http_get_request($target, $config);
        util::http_print_result(result);
    };
    (POST $target:expr,$config:expr,$body:expr) => {
        let result = util::http_post_request($target, $config, $body);
        util::http_print_result(result);
    };
    (PUT $target:expr,$config:expr,$body:expr) => {
        let result = util::http_put_request($target, $config, $body);
        util::http_print_result(result);
    };
    (DELETE $target:expr,$config:expr,$body:expr) => {
        let result = util::http_delete_request($target, $config, $body);
        util::http_print_result(result);
    };
}

#[macro_export]
macro_rules! json_stdout{
    ($msg:expr)=>{
        println!("{}", serde_json::json!({"output": $msg}))
    }
}

pub use http;
pub use json_stdout;

pub fn build_endpoint(target: &str, config: &Config) -> String {
    format!("http://{}:{}/_synapse/admin/v{}/{}", config.hostname, config.port, config.version, target)
}

pub fn new_http_client(config: &Config) -> reqwest::Result<reqwest::blocking::Client> {
    let value = format!("Bearer {}", config.token);
    let mut token_value = HeaderValue::from_str(&value).unwrap();
    token_value.set_sensitive(true);

    let mut headers = header::HeaderMap::new();
    headers.insert(header::AUTHORIZATION, token_value);

    reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
}

pub fn http_put_request<T>(target: &str, config: &Config, body: &T) -> Result<reqwest::blocking::Response, reqwest::Error>
where
    T: serde::ser::Serialize
{
    let endpoint = build_endpoint(target, &config);
    let client = new_http_client(&config)?;

    client.put(endpoint).json(body).send()
}

pub fn http_post_request<T>(target: &str, config: &Config, body: &T) -> Result<reqwest::blocking::Response, reqwest::Error>
where
    T: serde::ser::Serialize
{
    let endpoint = build_endpoint(target, &config);
    let client = new_http_client(&config)?;

    client.post(endpoint).json(body).send()
}

pub fn http_delete_request<T>(target: &str, config: &Config, body: &T) -> Result<reqwest::blocking::Response, reqwest::Error>
where
    T: serde::ser::Serialize
{
    let endpoint = build_endpoint(target, &config);
    let client = new_http_client(&config)?;

    client.delete(endpoint).json(body).send()
}

pub fn http_get_request(target: &str, config: &Config) -> Result<reqwest::blocking::Response, reqwest::Error> {
    let endpoint = build_endpoint(target, &config);
    let client = new_http_client(&config)?;
    client.get(endpoint).send()
}

pub fn http_print_result(result: Result<reqwest::blocking::Response, reqwest::Error>) {
        match result { 
            Ok(response) => match response.text() {
                Ok(body) => println!("{}", body),
                Err(_)   => json_stdout!("No HTTP response body found."),
            }
            Err(error)   => {
                let message = format!("{}", error);
                json_stdout!(message);
            }
        }
}
