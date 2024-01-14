use crate::error::Error;
use std::future::Future;

pub trait DnsProvider {
    fn create_dns(&self, key: String, value: String) -> impl Future<Output = Result<(), Error>>;
    fn delete_dns(&self, key: String, value: String) -> impl Future<Output = Result<(), Error>>;
}
