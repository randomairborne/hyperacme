use openssl::ec::EcKey;
use openssl::pkey;
use ring::p;

use crate::cert::EC_GROUP_P256;
use crate::error;

#[derive(Clone, Debug)]
pub(crate) struct AcmeKey {
    private_key: EcKey<pkey::Private>,
    /// set once we contacted the ACME API to figure out the key id
    key_id: Option<String>,
}

impl AcmeKey {
    pub(crate) fn new() -> Result<AcmeKey, error::Error> {
        let pri_key = EcKey::generate(&*EC_GROUP_P256)?;
        Ok(Self::from_key(pri_key))
    }

    pub(crate) fn from_pem(pem: &[u8]) -> Result<AcmeKey, error::Error> {
        let pri_key = EcKey::private_key_from_pem(pem)?;
        Ok(Self::from_key(pri_key))
    }

    fn from_key(private_key: EcKey<pkey::Private>) -> AcmeKey {
        AcmeKey {
            private_key,
            key_id: None,
        }
    }

    pub(crate) fn to_pem(&self) -> Result<Vec<u8>, error::Error> {
        let pem = self.private_key.private_key_to_pem()?;
        Ok(pem)
    }

    pub(crate) fn private_key(&self) -> &EcKey<pkey::Private> {
        &self.private_key
    }

    pub(crate) fn key_id(&self) -> &str {
        self.key_id.as_ref().unwrap()
    }

    pub(crate) fn set_key_id(&mut self, kid: String) {
        self.key_id = Some(kid)
    }
}
