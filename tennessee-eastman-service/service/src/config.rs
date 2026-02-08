pub struct Config {

    /// Passo de integração [s]. Define o Δt usado pelo integrador numérico.
    /// - Valores menores → mais precisão, mais custo computacional
    /// - Valores maiores → menos precisão, mais rápido
    pub dt: f64,

    /// Define se a simulação deve respeitar tempo real.
    /// - false → simulação em tempo lógico (o mais rápido possível)
    /// - true  → cada passo dorme dt segundos (simula planta "no ar")
    pub real_time: bool,
}
