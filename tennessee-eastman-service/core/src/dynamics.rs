use crate::{State, Inputs};

pub fn derivatives(
    state: &State,
    inputs: &Inputs,
    dx: &mut [f64],
) {
    // Placeholder: aqui entra o TEFUNC traduzido
    for i in 0..state.x.len() {
        dx[i] = 0.0; // f(x, u)
    }
}
