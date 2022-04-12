use openssl::ecdsa::EcdsaSig;
use openssl::sha::sha256;
use serde::Serialize;
use std::collections::VecDeque;
use std::{
    convert::TryInto,
    sync::{Arc, Mutex},
};

use crate::acc::AcmeKey;
use crate::error;
use crate::jwt::*;
use crate::req::{req_expect_header, req_head, req_post};
use crate::util::base64url;

/// JWS payload and nonce handling for requests to the API.
///
/// Setup is:
///
/// 1. `Transport::new()`
/// 2. `call_jwk()` against newAccount url
/// 3. `set_key_id` from the returned `Location` header.
/// 4. `call()` for all calls after that.
#[derive(Clone, Debug)]
pub(crate) struct Transport {
    acme_key: AcmeKey,
    nonce_pool: Arc<NoncePool>,
}

impl Transport {
    pub async fn new(nonce_pool: &Arc<NoncePool>, acme_key: AcmeKey) -> Self {
        Transport {
            acme_key,
            nonce_pool: nonce_pool.clone(),
        }
    }

    /// Update the key id once it is known (part of setting up the transport).
    pub async fn set_key_id(&mut self, kid: String) {
        self.acme_key.set_key_id(kid);
    }

    /// The key used in the transport
    pub async fn acme_key(&self) -> &AcmeKey {
        &self.acme_key
    }

    /// Make call using the full jwk. Only for the first newAccount request.
    pub async fn call_jwk<T: Serialize + ?Sized>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<reqwest::Response, error::Error> {
        self.do_call(url, body, jws_with_jwk).await
    }

    /// Make call using the key id
    pub async fn call<T: Serialize + ?Sized>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<reqwest::Response, error::Error> {
        self.do_call(url, body, jws_with_kid).await
    }

    async fn do_call<
        T: Serialize + ?Sized,
        F: Fn(&str, String, &AcmeKey, &T) -> Result<String, error::Error>,
    >(
        &self,
        url: &str,
        body: &T,
        make_body: F,
    ) -> Result<reqwest::Response, error::Error> {
        // The ACME API may at any point invalidate all nonces. If we detect such an
        // error, we loop until the server accepts the nonce.
        loop {
            // Either get a new nonce, or reuse one from a previous request.
            let nonce = self.nonce_pool.get_nonce().await?;

            // Sign the body.
            let body = make_body(url, nonce, &self.acme_key, body)?;

            debug!("Call endpoint {}", url);

            // Post it to the URL
            let result = req_post(url, body).await?;

            // Regardless of the request being a success or not, there might be
            // a nonce in the response.
            self.nonce_pool.extract_nonce(&result);

            // if let Err(problem) = &result {
            //     if problem.is_bad_nonce() {
            //         // retry the request with a new nonce.
            //         debug!("Retrying on bad nonce");
            //         continue;
            //     }
            //     // it seems we sometimes make bad JWTs. Why?!
            //     if problem.is_jwt_verification_error() {
            //         debug!("Retrying on: {}", problem);
            //         continue;
            //     }
            // }

            return Ok(result);
        }
    }
}

/// Shared pool of nonces.
#[derive(Default, Debug)]
pub(crate) struct NoncePool {
    nonce_url: String,
    pool: Mutex<VecDeque<String>>,
}

impl NoncePool {
    pub async fn new(nonce_url: &str) -> Self {
        NoncePool {
            nonce_url: nonce_url.into(),
            ..Default::default()
        }
    }

    async fn extract_nonce(&self, res: &reqwest::Response) -> Result<(), reqwest::header::ToStrError> {
        if let Some(nonce) = res.headers().get("replay-nonce") {
            trace!("Extract nonce");
            let mut pool = self.pool.lock().unwrap();
            pool.push_back(nonce.to_str()?.to_string());
            if pool.len() > 10 {
                pool.pop_front();
            }
        }
        Ok(())
    }

    async fn get_nonce(&self) -> Result<String, error::Error> {
        {
            let mut pool = self.pool.lock().unwrap();
            if let Some(nonce) = pool.pop_front() {
                trace!("Use previous nonce");
                return Ok(nonce);
            }
        }
        debug!("Request new nonce");
        let res = req_head(&self.nonce_url).await?;
        Ok(req_expect_header(&res, "replay-nonce")?)
    }
}

async fn jws_with_kid<T: Serialize + ?Sized>(
    url: &str,
    nonce: String,
    key: &AcmeKey,
    payload: &T,
) -> Result<String, error::Error> {
    let protected = JwsProtected::new_kid(key.key_id(), url, nonce);
    jws_with(protected, key, payload).await
}

async fn jws_with_jwk<T: Serialize + ?Sized>(
    url: &str,
    nonce: String,
    key: &AcmeKey,
    payload: &T,
) -> Result<String, error::Error> {
    let jwk: Jwk = key.try_into()?;
    let protected = JwsProtected::new_jwk(jwk, url, nonce);
    jws_with(protected, key, payload).await
}

async fn jws_with<T: Serialize + ?Sized>(
    protected: JwsProtected,
    key: &AcmeKey,
    payload: &T,
) -> Result<String, error::Error> {
    let protected = {
        let pro_json = serde_json::to_string(&protected)?;
        base64url(pro_json.as_bytes())
    };
    let payload = {
        let pay_json = serde_json::to_string(payload)?;
        if pay_json == "\"\"" {
            // This is a special case produced by ApiEmptyString and should
            // not be further base64url encoded.
            "".to_string()
        } else {
            base64url(pay_json.as_bytes())
        }
    };

    let to_sign = format!("{}.{}", protected, payload);
    let digest = sha256(to_sign.as_bytes());
    let sig = EcdsaSig::sign(&digest, key.private_key())?;
    let r = sig.r().to_vec();
    let s = sig.s().to_vec();

    let mut v = Vec::with_capacity(r.len() + s.len());
    v.extend_from_slice(&r);
    v.extend_from_slice(&s);
    let signature = base64url(&v);

    let jws = Jws::new(protected, payload, signature);

    Ok(serde_json::to_string(&jws)?)
}
