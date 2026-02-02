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
