use async_trait::async_trait;
use chrono::{Duration, TimeZone, Utc};
use openssl::x509::X509;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use termion::{color, style};
use thiserror::Error;

use crate::component::Component;
use crate::config::global_config::GlobalConfig;
use crate::constants::INDENT_WIDTH;
use crate::default_prepare;

#[derive(Debug, Deserialize)]
enum SortMethod {
    #[serde(alias = "alphabetical")] // Alias used to match lowercase spelling as well
    Alphabetical,
    #[serde(alias = "expiration")] // Alias used to match lowercase spelling as well
    Expiration,
    #[serde(alias = "manual")] // Alias used to match lowercase spelling as well
    Manual,
}

impl Default for SortMethod {
    fn default() -> Self {
        SortMethod::Manual
    }
}

#[derive(Debug, Deserialize)]
pub struct SSLCerts {
    #[serde(default)]
    sort_method: SortMethod,
    certs: HashMap<String, String>,
}

#[async_trait]
impl Component for SSLCerts {
    async fn print(self: Box<Self>, global_config: &GlobalConfig, _width: Option<usize>) {
        self.print_or_error(global_config)
            .unwrap_or_else(|err| println!("SSL Certificate error: {}", err));
        println!();
    }
    default_prepare!();
}

#[derive(Error, Debug)]
pub enum SSLCertsError {
    #[error("Failed to parse timestamp")]
    ChronoParse(#[from] chrono::ParseError),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error(transparent)]
    ErrorStack(#[from] openssl::error::ErrorStack),
}

struct CertInfo {
    name: String,
    status: String,
    expiration: systemstat::DateTime<systemstat::Utc>,
}

impl SSLCerts {
    pub fn print_or_error(self, global_config: &GlobalConfig) -> Result<(), SSLCertsError> {
        let mut cert_infos: Vec<CertInfo> = Vec::new();

        println!("SSL Certificates:");
        for (name, path) in self.certs {
            let cert = File::open(&path)?;
            let cert = BufReader::new(cert);
            let cert: Vec<u8> = cert.bytes().collect::<Result<_, _>>()?;
            let cert = X509::from_pem(&cert)?;

            let expiration =
                Utc.datetime_from_str(&format!("{}", cert.not_after()), "%B %_d %T %Y %Z")?;

            let now = Utc::now();
            let status = if expiration < now {
                format!("{}expired on{}", color::Fg(color::Red), style::Reset)
            } else if expiration < now + Duration::days(30) {
                format!("{}expiring on{}", color::Fg(color::Yellow), style::Reset)
            } else {
                format!("{}valid until{}", color::Fg(color::Green), style::Reset)
            };
            cert_infos.push(CertInfo {
                name,
                status,
                expiration,
            });
        }

        match self.sort_method {
            SortMethod::Alphabetical => {
                cert_infos.sort_by(|a, b| a.name.cmp(&b.name));
            }
            SortMethod::Expiration => {
                cert_infos.sort_by(|a, b| a.expiration.cmp(&b.expiration));
            }
            SortMethod::Manual => {}
        }

        for cert_info in cert_infos.into_iter() {
            println!(
                "{}{} {} {}",
                " ".repeat(INDENT_WIDTH as usize),
                cert_info.name,
                cert_info.status,
                cert_info.expiration.format(&global_config.time_format)
            );
        }

        Ok(())
    }
}
