# Tennessee Eastman Process — Simulation Service

Este repositório contém uma implementação moderna do **Tennessee Eastman Process (TEP)**,
um benchmark clássico de controle de processos industriais, originalmente proposto por
Downs & Vogel (1990).

O objetivo deste projeto é **simular a planta como um processo contínuo**, desacoplado de
qualquer lógica de controle, permitindo que controladores externos interajam com a planta
via uma interface bem definida.

---

## Visão Geral da Arquitetura

O projeto é dividido em dois níveis principais:

```bash
- tennessee-eastman-process → projeto original
- tennessee-eastman-service → versão digital twin que será um serviço
 ├── core/ # Modelo matemático da planta (EDOs)
 └── service/ # Processo executável e interface externa
```

### Princípios de Arquitetura

- A **planta é determinística** e não contém lógica de controle
- O **controle é sempre externo**
- O tempo avança de forma explícita via integração numérica
- Entradas e saídas são os únicos pontos de interação com o mundo externo

## O que este projeto NÃO é

- Não é um controlador
- Não é um sistema de otimização
- Não é um simulador gráfico
- Não usa REST para controle da planta

Este projeto implementa **a planta**, não o sistema de controle.

## Referências

- Downs, J. J., & Vogel, E. F. (1993).  
  *A Plant-Wide Industrial Process Control Problem*.  
  Computers & Chemical Engineering, 17(3), 245–255.

## Status

Em desenvolvimento — foco atual em:
- Modelagem correta da dinâmica
- Arquitetura limpa core ↔ service
- Execução contínua e eficiente