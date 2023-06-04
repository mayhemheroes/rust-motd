use honggfuzz::fuzz;
use rust_motd::config::Config;

fn main() {
    loop {
        fuzz!(|data: &str| {
            let _: Result<Config, toml::de::Error> = toml::from_str(&data);
        });
    }
}
