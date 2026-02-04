mod config;
mod interface;

use config::Config;
use interface::PlantBus;

fn main() {
    let config = Config {
        dt: 0.1,
        real_time: false,
    };

    let mut plant = PlantBus::new();

    loop {
        // placeholder: aqui entra integração no próximo passo
        plant.time += config.dt;

        // prova de vida
        println!("t = {}", plant.time);

        if config.real_time {
            std::thread::sleep(std::time::Duration::from_secs_f64(config.dt));
        }
    }
}
