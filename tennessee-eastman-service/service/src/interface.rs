/* NOTAS:
A planta é um sistema dinâmico contínuo que recebe entradas e produz saídas, evoluíndo seu estado
segundo as equações diferenciais que a definem. No TE original:
- Entradas (atuadores): 22 variáveis manipuladas (XMV)
- Estados Internos: 50 estados dinâmicos (YY)
- Saídas (medidas): 41 variáveis medidas (XMEAS)

Essa comunicação entre a planta e o controlador é feita através dessas variáveis. E como elas são
representadas no código? São um barramento de dados simples, usando vetores de ponto flutuante (f64).
*/

struct PlantBus {
    // Entradas (atuadores)
    inputs: Inputs,

    // Saídas (sensores)
    outputs: Outputs,

    // Estado interno (não visível)
    state: State,

    // Tempo
    time: f64,
}