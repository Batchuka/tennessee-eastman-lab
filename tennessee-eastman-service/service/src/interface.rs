/* NOTAS:
A planta é um sistema dinâmico contínuo que recebe entradas e produz saídas, evoluíndo seu estado
segundo as equações diferenciais que a definem. No TE original:
- Entradas (atuadores): 22 variáveis manipuladas (XMV)
- Estados Internos: 50 estados dinâmicos (YY)
- Saídas (medidas): 41 variáveis medidas (XMEAS)

Essa comunicação entre a planta e o controlador é feita através dessas variáveis. E como elas são
representadas no código? São um barramento de dados simples, usando vetores de ponto flutuante (f64).
*/

pub struct PlantBus {
    pub fn new() -> Self {
        Self {
            inputs: Inputs::default(),
            outputs: Outputs::default(),
            state: State::new(50),
            time: 0.0,
        }
    }
}