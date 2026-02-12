/* NOTAS:
Aqui definimos a estrutura principal do modelo Tennessee Eastman, que inclui o estado do sistema,
as entradas, as saídas e os parâmetros do modelo. A estrutura Plant encapsula todos esses componentes
e fornece métodos para inicialização, atualização do estado com base nas entradas e obtenção das saídas.
Esta implementação segue a abordagem descrita por Downs e Vogel (1993) no desenvolvimento do modelo Tennessee Eastman.
*/

use crate::state::State;
use crate::params::Params;

pub struct Plant {
    pub state: State,
    pub inputs: Inputs,
    pub outputs: Outputs,
    pub params: Params,
}

impl Plant {
    
    /// Inicialização padrão: estado zerado
    pub fn new(params: Params) -> Self { 
        let state = State::new(params.n_states);

        Self::from_state(state, params)
    }

    /// Inicialização explícita a partir de valores de estado conhecidos
    pub fn with_state_values(values: &[f64], params: Params) -> Self {
        let mut state = State::new(params.n_states);
        state.set(values);

        Self::from_state(state, params)
    }

    fn from_state(state: State, params: Params) -> Self {
        Self {
            state,
            inputs: Inputs::new(params.n_mv, params.n_dv),
            outputs: Outputs::new(params.n_outputs),
            params,
        }
    }

    pub fn set_inputs(&mut self, inputs: Inputs) {
        self.inputs = inputs;
    }

    pub fn step(&mut self, dt: f64) {
        integrator::step_euler(&mut self.state, &self.inputs, dt);
    }
}
