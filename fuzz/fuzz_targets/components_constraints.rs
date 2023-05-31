use honggfuzz::fuzz;
use rust_motd::{config::Config, component::{BoxedComponent, Constraints}};

fn main() {
    loop {
        fuzz!(|data: &str| {
            let config: Config = toml::from_str(&data).unwrap();
            let global_config = &config.global;

            let (_components, _constraints): (Vec<BoxedComponent>, Vec<Option<Constraints>>) = config
                .components
                .into_iter()
                .map(|component| component.prepare(global_config))
                .unzip();
        });
    }
}
