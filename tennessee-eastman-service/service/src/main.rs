use core::{Plant, params::Params};

fn main() {
    let params = Params::default();
    let mut plant = Plant::new(params);

    let dt = 1.0 / 3600.0; // 1 segundo

    loop {
        // futuramente: receber inputs externos
        plant.step(dt);

        // futuramente: publicar outputs
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
