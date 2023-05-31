use honggfuzz::fuzz;
use rust_motd::{config::Config, component::{BoxedComponent, Constraints}};

fn main() {
    loop {
        fuzz!(|data: &str| {
            let config: Config = toml::from_str(&data).unwrap();
            let global_config = &config.global;

            let (_components, constraints): (Vec<BoxedComponent>, Vec<Option<Constraints>>) = config
                .components
                .into_iter()
                .map(|component| component.prepare(&global_config))
                .unzip();

            let _width = constraints
                .into_iter()
                .flatten()
                .filter_map(|x| x.min_width)
                .max();
        });
    }
}
