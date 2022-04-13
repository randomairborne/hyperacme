use reqwest::header::{HeaderName, HeaderValue};

use crate::error;

pub(crate) async fn req_get(url: &str) -> Result<crate::req::ReqResult, error::Error> {
    let client = newclient().await;
    let req = client
        .get(url)
        .header(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("application/jose+json"),
        )
        .build()?;
    Ok(crate::req::ReqResult::from_response(client.execute(req).await?).await?)
}

pub(crate) async fn req_head(url: &str) -> Result<crate::req::ReqResult, error::Error> {
    let client = newclient().await;
    let req = client
        .head(url)
        .header(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("application/jose+json"),
        )
        .build()?;
    Ok(crate::req::ReqResult::from_response(client.execute(req).await?).await?)
}

pub(crate) async fn req_post(
    url: &str,
    body: String,
) -> Result<crate::req::ReqResult, error::Error> {
    let client = newclient().await;
    let req = client
        .post(url)
        .header(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("application/jose+json"),
        )
        .body(body)
        .build()?;
    Ok(crate::req::ReqResult::from_response(client.execute(req).await?).await?)
}

pub(crate) fn req_expect_header(
    res: &crate::req::ReqResult,
    name: &str,
) -> Result<String, error::Error> {
    let header_str = res
        .headers
        .get(name)
        .ok_or_else(|| error::Error::GeneralError("Header extraction error!".to_string()))?
        .to_str()?;
    Ok(header_str.to_string())
}

pub(crate) async fn newclient() -> reqwest::Client {
    reqwest::Client::new()
}

pub struct ReqResult {
    pub body: String,
    pub status: u16,
    pub headers: reqwest::header::HeaderMap,
}

impl ReqResult {
    pub async fn from_response(resp: reqwest::Response) -> Result<ReqResult, error::Error> {
        Ok(ReqResult {
            status: resp.status().as_u16(),
            headers: resp.headers().clone(),
            body: resp.text().await?,
        })
    }
}
