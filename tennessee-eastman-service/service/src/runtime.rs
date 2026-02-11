
use te_core::plant::Plant;
use te_core::params::Params;
use te_core::bus::PlantBus;
use te_core::state::State;
use te_core::inputs::Inputs;
use te_core::outputs::Outputs;

use te_core::metadata::*;

use crate::config::Config;

pub fn run(config: Config) {

    // Cria o barramento da planta (PlantBus), que representa o “estado observado” do processo:
    // - State::new(50): vetor dos estados dinâmicos internos da planta (inventários, energias, composições),
    //   ou seja, as variáveis que evoluem segundo as EDOs do modelo químico.
    // - Inputs::new(20, 10): entradas do processo:
    //   * mv → variáveis manipuladas (válvulas, vazões, utilidades),
    //   * dv → distúrbios (variações externas, falhas, mudanças de feed).
    // - Outputs::new(15): medições do processo (XMEAS), equivalentes aos sinais de instrumentos
    //   como TI, PI, FI, LI que um operador enxergaria.
    let mut bus = PlantBus::new(
        State::new(50),
        Inputs::new(20, 10),
        Outputs::new(15),
    );

    // Instancia o modelo da planta (Plant) com seus parâmetros físicos e de processo
    // (cinética, propriedades, constantes). Conceitualmente, aqui você cria o
    // “processo químico virtual” que será integrado no tempo.
    let mut plant = Plant::new(Params::default());

    loop {
        // Integra as equações diferenciais da planta por um passo de tempo (dt):
        // do ponto de vista de controle, é a dinâmica do processo reagindo às
        // entradas (XMV/DV) e aos estados atuais.
        plant.step(config.dt);
        
        
        // Avança o tempo lógico da simulação:
        // em termos de processo, é o relógio da planta (quanto tempo ela já operou).
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
