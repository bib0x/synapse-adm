#[macro_export]
macro_rules! http_bis{
    (GET $target:expr,$config:expr) => {
        let client = helper::HttpClient::new($config, $target);
        let response = client.get().await;
        helper::HttpClient::print_response(response).await;
    };
    (POST $target:expr,$config:expr,$body:expr) => {
        let client = helper::HttpClient::new($config, $target);
        let response = client.post($body).await;
        helper::HttpClient::print_response(response).await;
    };
    (PUT $target:expr,$config:expr,$body:expr) => {
        let client = helper::HttpClient::new($config, $target);
        let response = client.put($body).await;
        helper::HttpClient::print_response(response).await;
    };
    (DELETE $target:expr,$config:expr,$body:expr) => {
        let client = helper::HttpClient::new($config, $target);
        let response = client.delete($body).await;
        helper::HttpClient::print_response(response).await;
    };
}

#[macro_export]
macro_rules! http_funcs_with_body {
    ($($func:ident),*) => {
        $(
            pub async fn $func<T>(self, body: &T) -> Result<reqwest::Response, reqwest::Error> 
            where
                T: serde::ser::Serialize
            {
                let response = self.client.$func(self.target)
                                        .json(body)
                                        .send()
                                        .await?;

                Ok(response)
            }
        )*
    }
}