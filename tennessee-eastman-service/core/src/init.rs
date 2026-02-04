/* NOTAS
Essa lógica de inicialização é um placeholder. A inicialização correta do estado
deve carregar o estado steady-state do modelo Tennessee Eastman. É baseado na subrotina
FORTRAN TEINIT, fornecida por Downs e Vogel (1993).
*/

use crate::State;

pub fn initialize(state: &mut State) {
    // Placeholder: carregar steady-state do TE
    for x in state.x.iter_mut() {
        *x = 0.0;
    }
}
