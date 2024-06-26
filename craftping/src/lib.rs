use std::time::Duration;

use elytra_ping::parse::FancyText;
use elytra_ping::JavaServerInfo;
use elytra_ping::PingError;
use error::CraftPingError;
use error::CraftPingErrorKind;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Name;
use trust_dns_resolver::TokioAsyncResolver;

pub mod error;

#[derive(Debug)]
pub struct Response {
    pub motd: String,
    pub favicon: Option<String>,
    pub players_max: u32,
    pub players_online: u32,
    pub version: u32,
    pub version_name: String,
}

fn format(fncy: FancyText) -> String {
    let mut str = "<span style=\"".to_owned();
    if let Some(color) = fncy.color {
        str.push_str(format!("color: {};", color).as_str());
    }
    if Some(true) == fncy.bold {
        str.push_str("font-weight: bold;");
    }
    if Some(true) == fncy.italic {
        str.push_str("font-style: italic;");
    }
    if Some(true) == fncy.underlined {
        str.push_str("text-decoration: underline");
    }
    if Some(true) == fncy.strikethrough {
        str.push_str("text-decoration: line-through;");
    }
    str.push_str("\">");
    if let Some(text) = fncy.text {
        str.push_str(text.as_str());
    }
    str.push_str("</span>");
    str
}

fn iterate(text: FancyText) -> Vec<String> {
    text.extra
        .unwrap()
        .iter()
        .map(|v| {
            let mut final_str = String::new();
            let fncy: FancyText = v.to_owned().into();
            final_str += &format(fncy.clone());
            if fncy.extra.is_some() {
                final_str += &iterate(fncy).join("");
            }
            final_str
        })
        .collect::<Vec<String>>()
}

pub async fn ping(addr: String, port: u16) -> Result<Response, CraftPingError> {
    let ping = ping_srv(&addr, port).await;
    match ping {
        Ok(v) => {
            let v = v.0;
            let fncy: FancyText = v.description.into();

            Ok(Response {
                motd: extract_documentation(fncy),
                favicon: v.favicon,
                version: v.version.as_ref().map(|v| v.protocol).unwrap_or(47),
                version_name: v.version.map(|v| v.name).unwrap_or("".to_owned()),
                players_max: v.players.as_ref().map(|v| v.max).unwrap_or(0),
                players_online: v.players.map(|v| v.online).unwrap_or(0),
            })
        }
        Err(_) => {
            let (addr, port) = dns(&addr).await?;
            let ping = ping_srv(&addr.to_string(), port).await?;
            let ping = ping.0;
            let fncy: FancyText = ping.description.into();

            Ok(Response {
                motd: extract_documentation(fncy),
                favicon: ping.favicon,
                version: ping.version.as_ref().map(|v| v.protocol).unwrap_or(47),
                version_name: ping.version.map(|v| v.name).unwrap_or("".to_owned()),
                players_max: ping.players.as_ref().map(|v| v.max).unwrap_or(0),
                players_online: ping.players.map(|v| v.online).unwrap_or(0),
            })
        }
    }
}

fn extract_documentation(fncy: FancyText) -> String {
    if fncy.extra.is_some() {
        iterate(fncy)
    } else {
        vec![fncy.text.unwrap_or("".to_string())]
    }
    .join("")
}

async fn ping_srv(addr: &str, port: u16) -> Result<(JavaServerInfo, Duration), PingError> {
    elytra_ping::ping_or_timeout((addr.to_string(), port), std::time::Duration::from_secs(3)).await
}

async fn dns(addr: &str) -> Result<(Name, u16), CraftPingError> {
    let resolver = TokioAsyncResolver::tokio(ResolverConfig::default(), ResolverOpts::default());
    let service = format!("_minecraft._tcp.{}", addr);

    let res = resolver.srv_lookup(service).await?;

    if let Some(srv) = res.iter().next() {
        Ok((srv.target().clone(), srv.port()))
    } else {
        Err(CraftPingErrorKind::Failed.into())
    }
}
