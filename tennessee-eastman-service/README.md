# Tennessee Eastman Service

Este diretório contém a implementação executável da **planta Tennessee Eastman** como
um serviço contínuo, desacoplado de qualquer lógica de controle.

A planta é tratada como um **processo físico autônomo**, que:
- mantém um estado interno
- evolui no tempo via EDOs
- recebe entradas externas
- publica saídas continuamente

---

## Estrutura do Projeto

```bash
tennessee-eastman-service/
├── core/
│ ├── dynamics.rs # Equações diferenciais da planta
│ ├── integrator.rs # Algoritmo de integração temporal
│ ├── state.rs # Estados dinâmicos (YY)
│ ├── inputs.rs # Variáveis manipuladas (XMV)
│ ├── outputs.rs # Variáveis medidas (XMEAS)
│ ├── params.rs # Constantes e parâmetros do modelo
│ ├── init.rs # Condições iniciais
│ └── lib.rs # API pública do core
│
├── service/
│ ├── main.rs # Loop principal de execução
│ ├── interface.rs # Interface planta ↔ mundo externo
│ └── config.rs # Configuração do processo (dt, modo, etc)
│
└── Cargo.toml
```  

## Separação de Responsabilidades

### `core/`
- Biblioteca pura
- Nenhum IO
- Nenhuma thread
- Nenhuma noção de tempo real
- Apenas matemática e estado

Pode ser executado mais rápido ou mais lento que o tempo real.

### `service/`
- Processo executável
- Mantém o clock
- Aplica entradas
- Coleta saídas
- Orquestra a simulação

---

## Modelo de Execução

Em alto nível:

loop:
1. ler entradas externas
2. aplicar entradas na planta
3. integrar dinâmica por Δt
4. publicar saídas

Este loop roda continuamente enquanto o serviço estiver ativo.

---

## Controle

**Este serviço NÃO implementa controladores.**

Qualquer forma de controle (PID, MPC, RL, etc) deve:
- estar fora do processo
- interagir apenas via interface
- tratar a planta como uma “caixa física”

---

## Objetivo Técnico

- Simulação fiel do benchmark Tennessee Eastman
- Arquitetura próxima de sistemas industriais reais
- Base sólida para estudos de controle, observabilidade e diagnóstico

---