# 🧠 Inteligência Artificial: TAPADO (Q-Learning)

**TAPADO** (Treinamento de Agente Para Aprendizado de Objetivos) é uma demonstração visual de **Aprendizado por Reforço** (Reinforcement Learning), focado especificamente no algoritmo **Q-Learning**.

O objetivo deste experimento é observar como uma Inteligência Artificial sem conhecimento prévio do mundo (inicialmente "tapada") consegue, através de tentativa e erro matemático, descobrir o caminho perfeito para o sucesso.

---

## 🤖 Como o Algoritmo Funciona?

O Q-Learning é um algoritmo livre de modelo (*model-free*). Ele não precisa de um "mapa" do labirinto ou das regras da física embutidos na sua mente. Ele aprende puramente experimentando e anotando os resultados.

O cérebro do agente é composto por três pilares fundamentais:

### 1. A Q-Table (A Memória)
O agente possui uma grande tabela interna (chamada *Q-Table*). As linhas dessa tabela representam **Onde ele está no mundo** (o Estado / *State*), e as colunas representam **O que ele pode fazer** (a Ação / *Action* – Cima, Baixo, Esquerda, Direita).

Cada cruzamento dessa tabela guarda uma nota chamada **Q-Value**.
- Quando o robô nasce, a matriz está inteira preenchida com `zero`. Ele não sabe se andar para a direita é bom ou se leva para a morte.
- O objetivo do agente ao longo de sua vida é preencher esses "zeros" com notas que dizem: *"Se você está no bloco X, andar para Cima tem nota 90 de sucesso, mas andar para Baixo tem nota -10"*.

### 2. A Equação de Bellman (O Aprendizado)
Como ele atualiza essas notas? Toda vez que o agente dá um passo, ele recebe uma "recompensa" do ambiente (positiva ou negativa) e recalcula sua Q-Table usando a mágica da **Equação de Bellman**:

```text
Q(Novo) = Q(Atual) + α × [Recompensa + (γ × Max_Q(Próximo_Passo)) - Q(Atual)]
```
Onde:
- **Recompensa ($R$):** A dor ou glória imediata.
  - Bateu na Parede? **-10 (Dor aguda)**
  - Encontrou o Ouro? **+100 (Sucesso absoluto)**
  - Deu um passo num bloco vazio? **-0.5 (Micro-punição existencial)**. Isso serve para o cérebro dele entender que ficar andando à toa ou enrolando custa caro. Ele deve achar a rota mais curta.
- **Taxa de Aprendizado ($α$ / Alpha):** Controla o quão rápido o agente desconsidera o passado em favor da nova informação (ex: `0.15`).
- **Fator de Desconto ($γ$ / Gamma):** A variável mais importante. Ela varia de 0 a 1 (ex: `0.99`). Ela determina o quão "visionário" o agente é. Ele não olha apenas para a recompensa imediata, mas multiplica a maior nota que o **próximo** bloco possui pelo Gama. Assim, o valor do bloco com o "Ouro" (+100) vai se espalhando retroativamente pelos blocos adjacentes como um rastro de migalhas `(+99 -> +98 -> +97)`.

### 3. A Política Epsilon-Greedy (Exploração vs. Ganância)
Se o agente sempre escolhesse a ação com a maior nota na tabela, ele poderia encontrar um caminho medíocre e ficar viciado nele para sempre, sem saber que existe um atalho na outra ponta.

Para resolver isso, usamos uma variável chamada **Epsilon ($ε$)**:
- **O Nascimento ($ε = 1.0$):** O agente começa sendo 100% caótico. Toda decisão é tirada num dado viciado. Isso força a IA a andar aleatoriamente (bater a cabeça na parede mil vezes) para descobrir partes inexploradas do mundo. Isso é chamado de **Exploração (Exploration)**.
- **A Maturação:** Toda vez que ele morre ou ganha, nós multiplicamos o Epsilon por uma taxa de decaimento (ex: reduz `0.0015` por vida).
- **O Gênio ($ε = 0.05$):** Depois de centenas de mortes, a aversão ao asfalto cessa. O Epsilon bate no teto mínimo e para. Agora, em **95%** das vezes, ele age com pura **Ganância (Exploitation)**: ele olha pra Q-Table e escolhe exatamente a maior nota. Todavia, ainda mantemos 5% de aleatoriedade no cérebro dele eternamente para garantir que ele continue adaptável caso a vida mude de regras no meio do caminho.

---

## ⚙️ Ajustando o Cérebro (Hiperparâmetros)

Se você quer ver como a IA reage a taxas diferentes de aprendizado, todas as variáveis cruciais foram extraídas para o topo do arquivo `src/qlearning.rs`:

```rust
// Hiperparâmetros do Q-Learning - Ajuste aqui!
pub const Q_ALPHA: f32 = 0.15;         // Taxa de aprendizado
pub const Q_GAMMA: f32 = 0.99;         // Fator de desconto
pub const EPSILON_START: f32 = 1.0;    // Taxa de exploração inicial
pub const EPSILON_MIN: f32 = 0.05;     // Piso mínimo de exploração
pub const EPSILON_DECAY: f32 = 0.0015; // Decaimento por episódio
```
Você pode brincar com esses valores para ver se ela converge mais rápido ou se fica presa em becos sem saída!

---

## 🏎️ Controles e Velocidade

Como a matemática da matriz Q-Learning em memória é extremamente leve (uma tabela Hash), a renderização da janela é o único gargalo. Para você não precisar esperar horas para o agente convergir, há um sistema em camadas de **Tempo Acelerado (Turbo)**:

- **Tecla `T`:** Alterna ciclicamente entre os níveis de velocidade do mundo.
  - O ciclo é: `1x (Normal) -> 8x -> 16x -> 32x -> 64x -> 1000x`.
  - No modo **1000x**, o processador recalcula centenas de passos do agente _por frame_ gráfico da tela (16ms). Teoricamente, você pode treinar até o nível de Gênio em questão de poucos segundos.
- **Tecla `Espaço`:** Pausa/Retoma a simulação por completo.

---


## 🛑 O Algoritmo Algum Dia Termina?

**A resposta computacional é não.** Ele não possui um `break;` pra desligar o software após ficar inteligente. Ele continuará treinando as rotas ao infinito.

Porém, do ponto de vista do **Aprendizado de Máquina**, ele "Ganha o Jogo" quando dizemos que o modelo **convergou**:

1. **A Q-Table se estabilizou:** As contas da equação de Bellman não alteram mais drasticamente os valores das notas (o agente sabe de tudo).
2. **Epsilon mínimo bateu no piso:** Ele parou a fase de Exploração Caótica e entrou no modo Matemático Puro (Ganância).
3. E, visualmente no simulador: Você passa a observar o agente nascer no canto esquerdo da tela e voar numa linha ininterrupta e robótica de passos exatos e curtos até a linha de chegada dezenas de vezes em sequência sem desvio falho. (Isso costuma ocorrer entre a 300ª e a 500ª vida no modelo do TAPADO).
