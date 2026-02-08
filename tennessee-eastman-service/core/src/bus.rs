/* NOTAS:
PlantBus representa o barramento de comunicação da planta.

Ele contém:
- entradas (atuadores / distúrbios)
- estado interno (variáveis dinâmicas)
- saídas (medições)
- tempo lógico da simulação

Este barramento é compartilhado entre:
- a planta (Plant), que atualiza estado e saídas
- o serviço, que controla o avanço do tempo e entradas
*/

use crate::inputs::Inputs;
use crate::outputs::Outputs;
use crate::state::State;

pub struct PlantBus {
    /// Entradas manipuladas e distúrbios
    pub inputs: Inputs,

    /// Estado dinâmico interno da planta
    pub state: State,

    /// Saídas medidas da planta
    pub outputs: Outputs,

    /// Tempo lógico da simulação [s]
    pub time: f64,
}

impl PlantBus {
    pub fn new(state: State, inputs: Inputs, outputs: Outputs) -> Self {
        Self {
            state,
            inputs,
            outputs,
            time: 0.0,
        }
    }
}