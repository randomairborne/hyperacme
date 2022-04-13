use lazy_static::lazy_static;
use serde::de::DeserializeOwned;

lazy_static! {
    static ref BASE64_CONFIG: base64::Config =
        base64::Config::new(base64::CharacterSet::UrlSafe, false);
}

pub(crate) fn base64url<T: ?Sized + AsRef<[u8]>>(input: &T) -> String {
    base64::encode_config(input, *BASE64_CONFIG)
}

pub(crate) async fn read_json<T: DeserializeOwned>(
    res: crate::req::ReqResult,
) -> Result<T, crate::error::Error> {
    debug!("{}", res.body);
    Ok(serde_json::from_str(&res.body)?)
}
