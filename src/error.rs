use crate::api::ApiProblem;
pub use anyhow::{anyhow, bail, Context, Error, Result};
pub use log::{debug, trace};

impl From<ApiProblem> for Error {
    fn from(x: ApiProblem) -> Error {
        anyhow!("{}", x)
    }
}
