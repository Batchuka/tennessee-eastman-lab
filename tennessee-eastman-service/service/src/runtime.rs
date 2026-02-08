
use te_core::plant::Plant;
use te_core::params::Params;
use te_core::bus::PlantBus;
use te_core::state::State;
use te_core::inputs::Inputs;
use te_core::outputs::Outputs;

use te_core::metadata::*;

use crate::config::Config;

pub fn run(config: Config) {

    let mut bus = PlantBus::new(
        State::new(50),
        Inputs::new(20, 10),
        Outputs::new(15),
    );

    let mut plant = Plant::new(Params::default());

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
