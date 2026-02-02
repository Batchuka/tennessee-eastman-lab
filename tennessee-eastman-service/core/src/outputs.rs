#[derive(Clone)]
pub struct Outputs {
    /// Process measurements
    pub y: Vec<f64>, // ex: 41
}

impl Outputs {
    pub fn new(n: usize) -> Self {
        Self {
            y: vec![0.0; n],
        }
    }
}
