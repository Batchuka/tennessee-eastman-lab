/* NOTAS
Aqui implementamos a estrutura de entradas do modelo Tennessee Eastman. As variáveis manipuladas (MVs)
e as variáveis de perturbação (DVs) são representadas como vetores de ponto flutuante. Cada MV e DV
corresponde a um atuador ou uma perturbação específica no processo modelado. A estrutura Inputs inclui um método
construtor para inicializar os vetores com tamanhos especificados, preenchidos com zeros.
*/

#[derive(Clone)]
pub struct Inputs {
    /// Manipulated variables (atuadores)
    pub mv: Vec<f64>, // ex: 12

    /// Disturbance flags / magnitudes
    pub dv: Vec<f64>, // ex: 20
}

impl Inputs {
    
    pub fn new(n_mv: usize, n_dv: usize) -> Self {
        Self {
            mv: vec![0.0; n_mv],
            dv: vec![0.0; n_dv],
        }
    }
}
