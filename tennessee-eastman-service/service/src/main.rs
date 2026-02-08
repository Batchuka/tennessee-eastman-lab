mod config;
mod runtime;

use config::Config;

fn main() {
    
    let config = Config {
        dt: 0.1,
        real_time: true,
    };

    runtime::run(config);
}
