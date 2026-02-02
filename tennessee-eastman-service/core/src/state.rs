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
}
