use std::future::Future;

use crate::Error;

pub trait DnsProvider {
    fn create_dns(&self, key: String, value: String) -> impl Future<Output = Result<(), Error>>;
    fn delete_dns(&self, key: String, value: String) -> impl Future<Output = Result<(), Error>>;
}

pub trait AcmeProvider {
    fn directory(&self) -> impl AsRef<str>;
}

const LETSENCRYPT_PROD: &str = "https://acme-v02.api.letsencrypt.org/directory";
const LETSENCRYPT_STAGING: &str = "https://acme-staging-v02.api.letsencrypt.org/directory";

/// This struct uses the LetsEncrypt Production ACME issuer in non-debug builds,
/// and the Staging issuer in debug builds.
pub struct DynamicLetsEncrypt;

impl AcmeProvider for DynamicLetsEncrypt {
    fn directory(&self) -> impl AsRef<str> {
        #[cfg(not(debug_assertions))]
        {
            LETSENCRYPT_PROD
        }
        #[cfg(debug_assertions)]
        {
            LETSENCRYPT_STAGING
        }
    }
}

pub struct LetsEncrypt;

impl AcmeProvider for LetsEncrypt {
    fn directory(&self) -> impl AsRef<str> {
        LETSENCRYPT_PROD
    }
}

pub struct LetsEncryptStaging;

impl AcmeProvider for LetsEncryptStaging {
    fn directory(&self) -> impl AsRef<str> {
        LETSENCRYPT_STAGING
    }
}
