mod config;
mod helpers;
mod models;
mod services;

fn main() {
    let cnfg = config::Config::init().expect("Failed to init config");
    services::init_services(cnfg.clone());
}
