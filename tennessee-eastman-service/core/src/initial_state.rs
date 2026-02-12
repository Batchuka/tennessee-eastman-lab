use serde::Deserialize;
use std::fs;

const N_STATES: usize = 50;

#[derive(Debug, Deserialize)]
pub struct InitialState {
    pub state: StateSections,
}

#[derive(Debug, Deserialize)]
pub struct StateSections {
    pub reactor: ReactorSection,
    pub separator: SeparatorSection,
    pub stripper: StripperSection,
    pub compressor: CompressorSection,
    pub analyzers: AnalyzerSection,
}

#[derive(Debug, Deserialize)]
pub struct ReactorSection {
    pub vapor_holdup_kmol: Components,
    pub liquid_holdup_kmol: Components,
    pub energy: f64,
    pub pressure_kpa: f64,
    pub liquid_volume_m3: f64,
}

#[derive(Debug, Deserialize)]
pub struct SeparatorSection {
    pub vapor_holdup_kmol: Components,
    pub liquid_holdup_kmol: Components,
    pub energy: f64,
    pub pressure_kpa: f64,
    pub liquid_volume_m3: f64,
}

#[derive(Debug, Deserialize)]
pub struct StripperSection {
    pub liquid_holdup_kmol: Components,
    pub energy: f64,
    pub pressure_kpa: f64,
    pub liquid_volume_m3: f64,
}

#[derive(Debug, Deserialize)]
pub struct CompressorSection {
    pub work_kw: f64,
}

#[derive(Debug, Deserialize)]
pub struct AnalyzerSection {
    pub reactor_feed_delay: f64,
    pub purge_delay: f64,
    pub product_delay: f64,
}

#[derive(Debug, Deserialize)]
pub struct Components {
    pub A: f64,
    pub B: f64,
    pub C: f64,
    pub D: f64,
    pub E: f64,
    pub F: f64,
    pub G: f64,
    pub H: f64,
}

impl InitialState {

    pub fn from_file(path: &str) -> Result<Self, String> {
        let content = fs::read_to_string(path)
            .map_err(|e| format!("Erro lendo arquivo: {}", e))?;

        let parsed: InitialState = toml::from_str(&content)
            .map_err(|e| format!("Erro parseando TOML: {}", e))?;

        parsed.validate()?;
        Ok(parsed)
    }

    fn validate(&self) -> Result<(), String> {

        // Exemplo de validações físicas básicas
        if self.state.reactor.pressure_kpa <= 0.0 {
            return Err("Reactor pressure must be > 0".into());
        }

        if self.state.separator.pressure_kpa <= 0.0 {
            return Err("Separator pressure must be > 0".into());
        }

        if self.state.stripper.pressure_kpa <= 0.0 {
            return Err("Stripper pressure must be > 0".into());
        }

        Ok(())
    }
}

impl InitialState {

    pub fn flatten(&self) -> [f64; N_STATES] {

        let mut x = [0.0; N_STATES];
        let mut i = 0;

        // =========================
        // REACTOR
        // =========================
        push_components(&mut x, &mut i, &self.state.reactor.vapor_holdup_kmol);
        push_components(&mut x, &mut i, &self.state.reactor.liquid_holdup_kmol);

        x[i] = self.state.reactor.energy; i+=1;
        x[i] = self.state.reactor.pressure_kpa; i+=1;
        x[i] = self.state.reactor.liquid_volume_m3; i+=1;

        // =========================
        // SEPARATOR
        // =========================
        push_components(&mut x, &mut i, &self.state.separator.vapor_holdup_kmol);
        push_components(&mut x, &mut i, &self.state.separator.liquid_holdup_kmol);

        x[i] = self.state.separator.energy; i+=1;
        x[i] = self.state.separator.pressure_kpa; i+=1;
        x[i] = self.state.separator.liquid_volume_m3; i+=1;

        // =========================
        // STRIPPER
        // =========================
        push_components(&mut x, &mut i, &self.state.stripper.liquid_holdup_kmol);

        x[i] = self.state.stripper.energy; i+=1;
        x[i] = self.state.stripper.pressure_kpa; i+=1;
        x[i] = self.state.stripper.liquid_volume_m3; i+=1;

        // =========================
        // COMPRESSOR
        // =========================
        x[i] = self.state.compressor.work_kw; i+=1;

        // =========================
        // ANALYZERS
        // =========================
        x[i] = self.state.analyzers.reactor_feed_delay; i+=1;
        x[i] = self.state.analyzers.purge_delay; i+=1;
        x[i] = self.state.analyzers.product_delay; i+=1;

        assert!(i == N_STATES, "Flatten não preencheu 50 estados.");

        x
    }
}

fn push_components(x: &mut [f64], i: &mut usize, c: &Components) {
    x[*i] = c.A; *i+=1;
    x[*i] = c.B; *i+=1;
    x[*i] = c.C; *i+=1;
    x[*i] = c.D; *i+=1;
    x[*i] = c.E; *i+=1;
    x[*i] = c.F; *i+=1;
    x[*i] = c.G; *i+=1;
    x[*i] = c.H; *i+=1;
}
