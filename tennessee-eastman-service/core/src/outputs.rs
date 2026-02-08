/* NOTAS
Aqui será a lógica de medição de XMEAS do modelo Tennessee Eastman. As saídas do processo são representadas
como um vetor de ponto flutuante, onde cada elemento corresponde a uma medição específica do processo. 
A estrutura Outputs inclui um método construtor para inicializar o vetor com o tamanho especificado.
*/

#[derive(Clone)]
pub struct Outputs {
    /// Process measurements
    pub xmeas: Vec<f64>, // ex: 41
}

impl Outputs {
    pub fn new(n: usize) -> Self {
        Self {
            xmeas: vec![0.0; n],
        }
    }
}
