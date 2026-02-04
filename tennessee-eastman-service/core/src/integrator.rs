/* NOTAS
Aqui utilizamos o método de integração de Euler explícito para avançar o estado do sistema
ao longo do tempo. Esta abordagem simples calcula as derivadas do estado atual e as utiliza
para atualizar o estado com base em um passo de tempo fornecido. Assim como fizeram
nos autores originais do modelo Tennessee Eastman, Downs e Vogel (1993), este método
é adequado para simulações em tempo real devido à sua eficiência computacional.
*/

use crate::{State, Inputs};
use crate::dynamics::derivatives;

pub fn step_euler(
    state: &mut State,
    inputs: &Inputs,
    dt: f64,
) {
    let mut dx = vec![0.0; state.x.len()];
    derivatives(state, inputs, &mut dx);

    for i in 0..state.x.len() {
        state.x[i] += dx[i] * dt;
    }
}
