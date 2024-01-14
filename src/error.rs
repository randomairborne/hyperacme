use std::fmt::{Display, Formatter};

use serde::Deserialize;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Reqwest HTTP Error")]
    Reqwest(#[from] reqwest::Error),
    #[error("JSON Ser/De error")]
    SerdeJson(#[from] serde_json::Error),
    #[error("ACME error: {0}")]
    Acme(#[from] AcmeError),
}

#[derive(thiserror::Error, Debug, serde::Deserialize)]
pub struct AcmeError {
    #[serde(rename = "type")]
    kind: AcmeErrorKind,
    detail: String,
    subproblems: Option<Vec<AcmeSubproblem>>,
}

impl Display for AcmeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.detail)?;
        if let Some(subproblems) = &self.subproblems {
            write!(f, " [")?;
            for (idx, problem) in subproblems.iter().enumerate() {
                write!(f, "({problem})")?;
                if idx != subproblems.len() - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "]")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AcmeSubproblem {
    #[serde(rename = "type")]
    pub kind: AcmeErrorKind,
    pub detail: String,
    pub identifier: Option<AcmeIdentifier>,
}

impl Display for AcmeSubproblem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.detail)?;
        if let Some(identifier) = &self.identifier {
            write!(f, " ({identifier})")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct AcmeIdentifier {
    #[serde(rename = "type")]
    pub kind: String,
    pub value: String,
}

impl Display for AcmeIdentifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.value)
    }
}

#[derive(thiserror::Error, Debug, Copy, Clone, serde::Deserialize)]
#[non_exhaustive]
pub enum AcmeErrorKind {
    #[error("The request specified an account that does not exist")]
    #[serde(rename = "urn:ietf:params:acme:error:accountDoesNotExist")]
    AccountDoesNotExist,
    #[error("The request specified a certificate to be revoked that has already been revoked")]
    #[serde(rename = "urn:ietf:params:acme:error:alreadyRevoked")]
    AlreadyRevoked,
    #[error("The CSR is unacceptable (e.g., due to a short key)")]
    #[serde(rename = "urn:ietf:params:acme:error:badCSR")]
    BadCSR,
    #[error("The client sent an unacceptable anti-replay nonce")]
    #[serde(rename = "urn:ietf:params:acme:error:badNonce")]
    BadNonce,
    #[error("The JWS was signed by a public key the server does not support")]
    #[serde(rename = "urn:ietf:params:acme:error:badPublicKey")]
    BadPublicKey,
    #[error("The revocation reason provided is not allowed by the server")]
    #[serde(rename = "urn:ietf:params:acme:error:badRevocationReason")]
    BadRevocationReason,
    #[error("The JWS was signed with an algorithm the server does not support")]
    #[serde(rename = "urn:ietf:params:acme:error:badSignatureAlgorithm")]
    BadSignatureAlgorithm,
    #[error("Certification Authority Authorization (CAA) records forbid the CA from issuing a certificate")]
    #[serde(rename = "urn:ietf:params:acme:error:caa")]
    Caa,
    #[error("Specific error conditions are indicated in the `subproblems` array")]
    #[serde(rename = "urn:ietf:params:acme:error:compound")]
    Compound,
    #[error("The server could not connect to validation target")]
    #[serde(rename = "urn:ietf:params:acme:error:connection")]
    Connection,
    #[error("There was a problem with a DNS query during identifier validation")]
    #[serde(rename = "urn:ietf:params:acme:error:dns")]
    Dns,
    #[error("The request must include a value for the \"externalAccountBinding\" field")]
    #[serde(rename = "urn:ietf:params:acme:error:externalAccountRequired")]
    ExternalAccountRequired,
    #[error("Response received didn't match the challenge's requirements")]
    #[serde(rename = "urn:ietf:params:acme:error:incorrectResponse")]
    IncorrectResponse,
    #[error("A contact URL for an account was invalid")]
    #[serde(rename = "urn:ietf:params:acme:error:invalidContact")]
    InvalidContact,
    #[error("The request message was malformed")]
    #[serde(rename = "urn:ietf:params:acme:error:malformed")]
    Malformed,
    #[error("The request attempted to finalize an order that is not ready to be finalized")]
    #[serde(rename = "urn:ietf:params:acme:error:orderNotReady")]
    OrderNotReady,
    #[error("The request exceeds a rate limit")]
    #[serde(rename = "urn:ietf:params:acme:error:rateLimited")]
    Ratelimited,
    #[error("The server will not issue certificates for the identifier")]
    #[serde(rename = "urn:ietf:params:acme:error:rejectedIdentifier")]
    RejectedIdentifier,
    #[error("The server experienced an internal error")]
    #[serde(rename = "urn:ietf:params:acme:error:serverInternal")]
    ServerInternal,
    #[error("The server received a TLS error during validation")]
    #[serde(rename = "urn:ietf:params:acme:error:tls")]
    Tls,
    #[error("The client lacks sufficient authorization")]
    #[serde(rename = "urn:ietf:params:acme:error:unauthorized")]
    Unauthorized,
    #[error("A contact URL for an account used an unsupported protocol scheme")]
    #[serde(rename = "urn:ietf:params:acme:error:unsupportedContact")]
    UnsupportedContact,
    #[error("An identifier is of an unsupported type")]
    #[serde(rename = "urn:ietf:params:acme:error:unsupportedIdentifier")]
    UnsupportedIdentifier,
    #[error("Visit the `instance` url and take actions specified there")]
    #[serde(rename = "urn:ietf:params:acme:error:userActionRequired")]
    UserActionRequired,
}
