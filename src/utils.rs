use crate::errors::RpcError;

pub fn my_ip() -> Result<String, RpcError> {
    Ok(reqwest::get("https://api.ipify.org")?.text()?)
}
