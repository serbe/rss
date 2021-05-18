use std::time::Instant;

use chrono::{DateTime, Local};
use netc::Client;

use crate::errors::RssError;

#[derive(Clone, Debug)]
pub struct Proxy {
    pub insert: bool,
    pub update: bool,
    pub work: bool,
    pub anon: bool,
    pub checks: i32,
    pub hostname: String,
    pub host: String,
    pub port: String,
    pub scheme: String,
    pub create_at: DateTime<Local>,
    pub update_at: DateTime<Local>,
    pub response: i64,
}

impl Proxy {
    pub fn from(s: &str) -> Result<Self, RssError> {
        let raw = s;

        if raw.contains('#') {
            Err(RssError::ParseFragment(raw.to_string()))?
        }

        if raw.contains('?') {
            Err(RssError::ParseQuery(raw.to_string()))?
        }

        let (raw, scheme) = if let Some(pos) = raw.find("://") {
            (
                raw.get(pos + 3..)
                    .ok_or_else(|| RssError::ParseBadScheme(raw.to_string()))?,
                raw.get(..pos)
                    .ok_or_else(|| RssError::ParseBadScheme(raw.to_string()))?
                    .to_string(),
            )
        } else {
            Err(RssError::ParseMissingScheme(raw.to_string()))?
        };

        if raw.contains('@') {
            Err(RssError::ParseBadUserInfo(raw.to_string()))?
        };

        if raw.contains('/') {
            Err(RssError::ParseHavePath(s.to_string(), raw.to_string()))?
        };

        let (host, port) = if let Some(pos) = raw.rfind(':') {
            if let Some(start) = raw.find('[') {
                if let Some(end) = raw.find(']') {
                    if start == 0 && pos == end + 1 {
                        (
                            raw.get(..pos)
                                .ok_or_else(|| RssError::ParseHost(raw.to_string()))?
                                .to_string(),
                            raw.get(pos + 1..)
                                .ok_or_else(|| RssError::ParsePort(raw.to_string()))?
                                .to_string(),
                        )
                    } else {
                        Err(RssError::ParseIpv6(raw.to_string()))?
                    }
                } else {
                    Err(RssError::ParseIpv6(raw.to_string()))?
                }
            } else {
                (
                    raw.get(..pos)
                        .ok_or_else(|| RssError::ParseHost(raw.to_string()))?
                        .to_string(),
                    raw.get(pos + 1..)
                        .ok_or_else(|| RssError::ParsePort(raw.to_string()))?
                        .to_string(),
                )
            }
        } else {
            Err(RssError::ParsePort(raw.to_string()))?
        };

        let _ = port.parse::<u32>()?;

        Ok(Proxy {
            insert: false,
            update: false,
            work: false,
            anon: false,
            checks: 0,
            hostname: format!("{}://{}:{}", scheme, host, port),
            host,
            port,
            scheme,
            create_at: chrono::Local::now(),
            update_at: chrono::Local::now(),
            response: 0,
        })
    }
}

pub async fn check_proxy(
    proxy_url: &str,
    target_url: &str,
    my_ip: &str,
) -> Result<Proxy, RssError> {
    let dur = Instant::now();
    let mut client = Client::builder()
        .proxy(proxy_url)
        .get(target_url)
        .build()
        .await?;
    let body = client.send().await?.text()?;
    let mut proxy = Proxy::from(proxy_url)?;
    proxy.work = true;
    if !body.contains(my_ip) && body.matches("<p>").count() == 1 {
        proxy.anon = true;
    }
    proxy.create_at = Local::now();
    proxy.update_at = Local::now();
    proxy.response = dur.elapsed().as_micros() as i64;
    Ok(proxy)
}
