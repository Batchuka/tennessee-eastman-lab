
use crate::{inputs::Inputs, state::State};

pub fn derivatives(
    state: &State,
    inputs: &Inputs,
    dx: &mut [f64],
) {

    // TODO: (TEFUNC):
    // Implementar aqui as equações diferenciais do Tennessee Eastman Process.
    // Esta função deve calcular ẋ = f(x, u, p), usando:
    // - state.x  → estados atuais (YY)
    // - inputs   → variáveis manipuladas e distúrbios (XMV / IDV)
    // O resultado deve ser escrito em dx, que representa as derivadas dos estados.

    // Exemplo: duas EDOs acopladas
    // ẋ₁ = -x₁ + u
    // ẋ₂ =  x₁ - 2·x₂

    dx[0] = -state.x[0] + inputs.mv[0];
    dx[1] =  state.x[0] - 2.0 * state.x[1];
}

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
