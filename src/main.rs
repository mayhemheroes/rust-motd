use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use systemstat::{Platform, System};

mod components;
use components::banner::{disp_banner, BannerCfg};
use components::filesystem::{disp_filesystem, FilesystemsCfg};
use components::ssl_certs::{disp_ssl, SSLCertsCfg};
use components::uptime::{disp_uptime, UptimeCfg};
mod constants;

#[derive(Debug, Deserialize)]
struct Config {
    banner: Option<BannerCfg>,
    service_status: Option<ServiceStatusCfg>,
    uptime: Option<UptimeCfg>,
    ssl_certificates: Option<SSLCertsCfg>,
    filesystems: Option<FilesystemsCfg>,
    fail_2_ban: Option<Fail2BanCfg>,
    last_login: Option<LastLoginCfg>,
}

type ServiceStatusCfg = HashMap<String, String>;

#[derive(Debug, Deserialize)]
struct Fail2BanCfg {
    jails: Vec<String>,
}

type LastLoginCfg = HashMap<String, usize>;

fn main() {
    match fs::read_to_string("default_config.toml") {
        Ok(config_str) => {
            let config: Config = toml::from_str(&config_str).unwrap();
            let sys = System::new();

            if let Some(banner_config) = config.banner {
                disp_banner(banner_config);
            }

            if let Some(uptime_config) = config.uptime {
                disp_uptime(uptime_config, &sys)
                    .unwrap_or_else(|err| println!("Uptime error: {}", err));
            }

            if let Some(ssl_certificates_config) = config.ssl_certificates {
                disp_ssl(ssl_certificates_config)
                    .unwrap_or_else(|err| println!("SSL Certificate error: {}", err));
            }

            if let Some(filesystems) = config.filesystems {
                disp_filesystem(filesystems, &sys)
                    .unwrap_or_else(|err| println!("Filesystem error: {}", err));
            }
        }
        Err(e) => println!("Error reading config file: {}", e),
    }
}
