use synapse_adm::http_funcs_with_body;
use reqwest::header;

use crate::config::Config;

pub struct HttpClient {
    pub client: reqwest::Client,
    pub target: String,
}

impl HttpClient {

    pub fn new(config: &Config, target: &str) -> Self {
        let token: String = if let Some(t) = &config.token {
            t.to_owned()
        } else {
            String::default()
        };
        let value = format!("Bearer {}", token);
        let mut token_value = header::HeaderValue::from_str(&value).unwrap();
        token_value.set_sensitive(true);

        let mut headers = header::HeaderMap::new();
        headers.insert(header::AUTHORIZATION, token_value);

        HttpClient {
            client: reqwest::Client::builder()
                        .default_headers(headers)
                        .build()
                        .expect("impossible to create HTTP client"),
            target: format!("http://{}:{}/_synapse/admin/v{}/{}", config.hostname, config.port, config.version, target)
        }  
    }

    pub async fn print_response(result: Result<reqwest::Response, reqwest::Error>) {
        match result { 
            Ok(response) => match response.text().await {
                Ok(body) => println!("{}", body),
                Err(_)   => eprintln!("{}", serde_json::json!({"error": "No HTTP response body found".to_string()}))
            },
            Err(error)   => {
                eprintln!("{}", serde_json::json!({"error": error.to_string()}))
            }
        }
    }

    pub async fn get(self) ->  Result<reqwest::Response, reqwest::Error> {
        let response = self.client.get(self.target)
                                  .send()
                                  .await?;
        Ok(response)
    }

    http_funcs_with_body!(post, put, delete);
}
