use reqwest::{Client as HttpClient, ClientBuilder as HttpClientBuilder};
pub struct Client {
    client: HttpClient,
}

impl Client {
    pub fn new() -> Client {
        let client = HttpClientBuilder::new()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()
            .unwrap();
        Self { client }
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}
