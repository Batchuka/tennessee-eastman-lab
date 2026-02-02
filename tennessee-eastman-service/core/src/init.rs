use crate::State;

pub fn initialize(state: &mut State) {
    // Placeholder: carregar steady-state do TE
    for x in state.x.iter_mut() {
        *x = 0.0;
    }
}
