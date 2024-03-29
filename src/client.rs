use jose_jws::Signature;
use reqwest::{Client as HttpClient, ClientBuilder as HttpClientBuilder};

use crate::{AcmeProvider, Directory, Error};

pub struct Client {
    client: HttpClient,
    directory: Directory,
    account_private_key: jose_b64::serde::Bytes
}

impl Client {
    pub async fn new(environment: impl AcmeProvider) -> Result<Client, Error> {
        let client = HttpClientBuilder::new()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()?;
        let directory_bytes = client
            .get(environment.directory().as_ref())
            .send()
            .await?
            .bytes()
            .await?;
        let directory = serde_json::from_slice(&directory_bytes)?;
        Ok(Self { client, directory })
    }

    pub fn directory(&self) -> &Directory {
        &self.directory
    }

    pub(crate) async fn request<T>(url: String) -> Result<T, Error> {
        let payload = ();
        jose_jws::Flattened {
            payload: None,
            signature: Signature {
                header: None,
                protected: None,
                signature: ,
            },
        };
        unimplemented!()
    }
}
