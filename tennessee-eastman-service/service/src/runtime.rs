
use te_core::plant::Plant;
use te_core::params::Params;
use te_core::initial_state::InitialState;

use te_core::metadata::*;

use crate::config::Config;

pub fn run(config: Config) {

    let initial = InitialState::from_file(&config.initial_state_path).unwrap();
    let flat = initial.flatten();

    let mut plant = Plant::with_state_values(&flat, Params::default());


    loop {
        plant.step(config.dt);
        bus.time += config.dt;

        // --- XMEAS ---
        for meta in MEASUREMENTS {
            let value = bus.outputs.xmeas[meta.index];
            println!("[{}] {} = {:.4} {}", meta.tag, meta.name, value, meta.unit );
        }

        // --- XMV ---
        for meta in MANIPULATED {
            let value = bus.inputs.mv[meta.index];
            println!("[{}] {} = {:.2} {}",meta.tag, meta.name, value, meta.unit);
        }

        if config.real_time {
            std::thread::sleep(std::time::Duration::from_secs_f64(config.dt));
        }
    }
}
