pub trait DynamicModel {
    fn derivatives(
        &self,
        state: &State,
        inputs: &Inputs,
        params: &Params,
    ) -> Vec<f64>;
}
