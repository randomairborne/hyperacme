use reqwest::header::{HeaderName, HeaderValue};

use crate::api::ApiProblem;
use crate::error;

pub(crate) type ReqResult<T> = std::result::Result<T, ApiProblem>;

pub(crate) async fn req_get(url: &str) -> Result<reqwest::Response, error::Error> {
    let client = newclient().await;
    let req = client
        .get(url)
        .header(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("application/jose+json"),
        )
        .build()?;
    Ok(client.execute(req).await?)
}

pub(crate) async fn req_head(url: &str) -> Result<reqwest::Response, error::Error> {
    let client = newclient().await;
    let req = client
        .head(url)
        .header(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("application/jose+json"),
        )
        .build()?;
    Ok(client.execute(req).await?)
}

pub(crate) async fn req_post(url: &str, body: String) -> Result<reqwest::Response, error::Error> {
    let client = newclient().await;
    let req = client
        .post(url)
        .header(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("application/jose+json"),
        )
        .body(body)
        .build()?;
    Ok(client.execute(req).await?)
}

pub(crate) async fn req_handle_error(
    res: reqwest::Response,
) -> Result<reqwest::Response, error::Error> {
    // ok responses pass through
    if res.status() == 200 {
        return Ok(res);
    }

    let problem = if res
        .headers()
        .get("Content-Type")
        .ok_or(error::Error::NoContentType)?
        == "application/problem+json"
    {
        // if we were sent a problem+json, deserialize it
        let body = res.text().await?;
        serde_json::from_str(&body).unwrap_or_else(|e| ApiProblem {
            _type: "problemJsonFail".into(),
            detail: Some(format!(
                "Failed to deserialize application/problem+json ({}) body: {}",
                e.to_string(),
                body
            )),
            subproblems: None,
        })
    } else {
        // some other problem
        let status = format!("{} {}", res.status(), res.text().await?);
        let body = res.text().await?;
        let detail = format!("{} body: {}", status, body);
        ApiProblem {
            _type: "httpReqError".into(),
            detail: Some(detail),
            subproblems: None,
        }
    };

    Err(problem)
}

pub(crate) fn req_expect_header(
    res: &reqwest::Response,
    name: &str,
) -> Result<String, error::Error> {
    let header_str = res
        .headers()
        .get(name)
        .ok_or_else(|| error::Error::GeneralError("Header extraction error!".to_string()))?
        .to_str()?;
    Ok(header_str.to_string())
}

pub(crate) async fn newclient() -> reqwest::Client {
    reqwest::Client::new()
}
