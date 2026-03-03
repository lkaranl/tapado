# 🏗️ Estrutura do Projeto — TAPADO

Este documento descreve a organização do código-fonte e a responsabilidade de cada módulo no simulador de Q-Learning. O projeto é construído utilizando a engine **Bevy** (ECS - Entity Component System).

---

## 📁 Pasta `src/`

A lógica do projeto está dividida em 5 arquivos principais, cada um cuidando de um aspecto específico da simulação:

### 1. `main.rs` (O Maestro)
É o ponto de entrada da aplicação.
- **Função**: Configura a janela, inicializa a engine Bevy e registra todos os **Plugins** necessários (`GridPlugin`, `AgentPlugin`, `UiPlugin`).
- **O que faz**: Define a cor de fundo, spawna a câmera e garante que todos os sistemas de outros arquivos sejam executados na ordem correta.

### 2. `grid.rs` (O Ambiente)
Define o mundo onde o agente vive.
- **Função**: Gerencia o labirinto, as paredes e o objetivo.
- **O que faz**: 
    - Contém o mapa (matriz 10x10).
    - Define as cores de cada elemento (vazio, parede, objetivo).
    - Implementa o sistema de **Trail** (o rastro verde que o agente deixa).
    - Gerencia o efeito de "pulso" visual no objetivo.

### 3. `agent.rs` (O Cérebro Motor)
Gerencia o estado físico e as ações do agente.
- **Função**: Controla o movimento, colisões e estatísticas.
- **O que faz**:
    - Mantém a posição `(x, y)` do agente no grid.
    - **handle_input**: Processa comandos do teclado (R para reset, P para parâmetros, setas para ajuste).
    - **agent_step**: É o coração da simulação; decide a próxima ação, pede ao Q-Learning para aprender e lida com o que acontece se o agente bater ou ganhar.
    - Controla os efeitos visuais de "flash" vermelho quando o agente bate na parede.

### 4. `qlearning.rs` (A Inteligência)
Contém a "matemática" pura do aprendizado de máquina.
- **Função**: Implementa o algoritmo de Q-Learning.
- **O que faz**:
    - Define a **Q-Table** (tabela que guarda o conhecimento do agente).
    - **update_qtable**: Implementa a fórmula de Bellman para atualizar os valores de recompensa.
    - Gerencia o **Epsilon (ε)**: Decide se o agente deve explorar algo novo ou usar o que já sabe.
    - Aplica o decaimento (decay) do aprendizado a cada episódio.

### 5. `ui.rs` (A Interface)
Cuida de tudo que é desenhado sobre a simulação (o HUD).
- **Função**: Exibe informações em tempo real e menus.
- **O que faz**:
    - **setup_hud**: Desenha os textos laterais (Episódio, Passos, Melhor, Epsilon).
    - **update_hud**: Atualiza esses valores a cada quadro (frame).
    - **Painel de Parâmetros**: Implementa o overlay que aparece ao apertar 'P', permitindo ajustar Alpha, Gamma e Epsilon em tempo real.

---

## 🏎️ Fluxo de Funcionamento

1. O `main.rs` inicia e carrega o labirinto (`grid.rs`).
2. O `agent.rs` escolhe uma direção baseado na tabela em `qlearning.rs`.
3. O agente se move no `grid.rs`. 
4. O resultado (recompensa ou punição) é enviado de volta para a `QTable` em `qlearning.rs` para ele aprender.
5. O `ui.rs` mostra o progresso no canto da tela para o usuário.
