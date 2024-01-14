#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Reqwest HTTP Error")]
    Reqwest(#[from] reqwest::Error),
}
