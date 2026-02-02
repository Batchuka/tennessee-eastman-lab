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
