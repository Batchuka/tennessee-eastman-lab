/* NOTAS
Aqui iremos implementar a estrutura de estado do modelo Tennessee Eastman. Se em dynamics.rs nós 
definimos como o sistema evolui ao longo do tempo, aqui definimos quais são as variáveis de estado
que representam o sistema em um dado instante. A estrutura State inclui um método construtor para 
inicializar o vetor de estados com o tamanho especificado.
*/

#[derive(Clone)]
pub struct State {
    /// Estados dinâmicos da planta (ex: 50 estados do TE)
    pub x: Vec<f64>,
}

impl State {
    
    pub fn new(n: usize) -> Self {
        Self {
            x: vec![0.0; n],
        }
    }

    pub fn set(&mut self, values: &[f64]) {
        assert!(
            values.len() == self.x.len(),
            "State::set: tamanho inválido"
        );
        self.x.copy_from_slice(values);
    }
}
