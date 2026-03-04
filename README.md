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

### O que significa cada um?

- **`Q_ALPHA` (Taxa de Aprendizado):** Determina o quanto o agente confia em uma *nova* informação em detrimento do que ele já sabia antes.
  - Se for **0.0**, ele não aprende nada de novo.
  - Se for **1.0**, ele esquece tudo o que sabia e só confia no que acabou de descobrir (fica muito instável no aprendizado).
  - Em **0.15**, há um equilíbrio perfeito onde a memória antiga é pesada harmoniosamente com a nova experiência.

- **`Q_GAMMA` (Fator de Desconto):** Define quão "míope" ou "visionário" o agente é.
  - Se for **0.0**, ele só liga para a recompensa imediata (nunca acharia o fim do labirinto).
  - Se for perto de **1.0 (ex: 0.99)**, ele entende que a recompensa final de ouro é valiosa, criando um "rastro de valor" que puxa ele do ínicio ao fim num caminho ininterrupto. Sem esse 0.99, o agente ficaria andando em círculos.

- **`EPSILON_START`:** A probabilidade do agente fazer uma ação 100% aleatória no seu primeiro segundo de vida. (`1.0 = 100%`). Em 100%, ele parece um robô quebrado batendo na parede. Isso é vital para que ele possa mapear cada canto do mundo.

- **`EPSILON_MIN`:** O piso duro do processo. Depois de muitas mortes, sua taxa de ações aleatórias vai decair, mas ela deve parar em algum lugar. Com `0.05`, significa que _mesmo depois_ de alcançar maestria e o nível de gênio, ele ainda reserva 5% de chance de dar um "passo maluco". Isso impede o agente de ficar viciado num caminho que encontrou cedo, mas que talvez não seja o melhor. Se você colocar `0.000001`, ele vai atingir um caminho ótimo global absoluto!

- **`EPSILON_DECAY`:** O quão rápido a aleatoriedade (Epsilon) perde força. Em `0.0015`, quer dizer que após aproximadamente 600 vidas vivendo caoticamente, ele chega à "maturidade", onde não chuta mais e vira puramente ganancioso em seus cálculos.

Você pode brincar com esses valores para ver se ela converge mais rápido ou se fica presa em becos sem saída!

---

## 🏎️ Controles e Velocidade

Como a matemática da matriz Q-Learning em memória é extremamente leve (uma tabela Hash), a renderização da janela é o único gargalo. Para você não precisar esperar horas para o agente convergir, há um sistema em camadas de **Tempo Acelerado (Turbo)**:

- **Tecla `T`:** Alterna ciclicamente entre os níveis de velocidade do mundo.
  - O ciclo é: `1x (Normal) -> 8x -> 16x -> 32x -> 64x -> 1000x -> 5000x`.
  - No modo **5000x**, o processador recalcula quase um episódio inteiro por frame gráfico (16ms). É praticamente instantâneo!
- **Tecla `Espaço`:** Pausa/Retoma a simulação por completo.

---


## 🛑 O Algoritmo Algum Dia Termina?

**A resposta computacional é não.** Ele não possui um `break;` pra desligar o software após ficar inteligente. Ele continuará treinando as rotas ao infinito.

Porém, do ponto de vista do **Aprendizado de Máquina**, ele "Ganha o Jogo" quando dizemos que o modelo **convergou**:

1. **A Q-Table se estabilizou:** As contas da equação de Bellman não alteram mais drasticamente os valores das notas (o agente sabe de tudo).
2. **Epsilon mínimo bateu no piso:** Ele parou a fase de Exploração Caótica e entrou no modo Matemático Puro (Ganância).
3. E, visualmente no simulador: Você passa a observar o agente nascer no canto esquerdo da tela e voar numa linha ininterrupta e robótica de passos exatos e curtos até a linha de chegada dezenas de vezes em sequência sem desvio falho. (Isso costuma ocorrer entre a 300ª e a 500ª vida no modelo do TAPADO).

---

## ⚡ Métricas de Velocidade (Turbo)

A unidade de medida fundamental do simulador é o **Intervalo entre Passos** (em segundos). Ao ativar o modo Turbo, nós dividimos esse intervalo base, permitindo que o agente tome milhares de decisões matemáticas dentro de um único frame de processamento (60 FPS).

### Entendendo a Velocidade do Agente

Para tornar o aprendizado mais rápido, nós não mudamos a "força" da IA, mas sim diminuímos o tempo que ela gasta "pensando" entre um passo e outro. Funciona assim:

1.  **O Ritmo Natural (`1x`):** 
    O agente é configurado para esperar **0,12 segundos** entre cada ação. Imagine um metrônomo batendo: nesse ritmo, o agente consegue dar cerca de **8 passos por segundo**.

2.  **Aceleração por Divisão:** 
    Quando você ativa o Turbo, nós dividimos esse tempo de espera. No **Turbo 8x**, o robô passa a esperar apenas **0,015 segundos**. Como o tempo de espera ficou 8 vezes menor, ele consegue dar 8 vezes mais passos no mesmo segundo (aprox. **66 passos/seg**).

3.  **O Modo "Infinito" (Turbo INF):** 
    No nível máximo (50.000x), o tempo de espera é reduzido para um valor invisível: **0,0000024 segundos**. 
    Nesse modo, o agente tenta realizar mais de **416 mil cálculos por segundo**. É tanta velocidade que o robô parece "teletransportar" pelo mapa, processando milhares de tentativas de uma só vez entre cada atualização da tela.

Abaixo está a tabela comparativa:

| Modo Turbo | Multiplicador | Intervalo (Segundos) | **Passos por Segundo (SPS)** |
| :--- | :--- | :--- | :--- |
| **Normal (0x)** | 1.0 | 0.12s | **~8** |
| **8x** | 8.0 | 0.015s | **~66** |
| **16x** | 16.0 | 0.0075s | **~133** |
| **32x** | 32.0 | 0.00375s | **~266** |
| **64x** | 64.0 | 0.00187s | **~533** |
| **1000x** | 1.000,0 | 0.00012s | **~8.333** |
| **5000x** | 5.000,0 | 0.000024s | **~41.666** |
| **10000x** | 10.000,0 | 0.000012s | **~83.333** |
| **INF (50k)** | 50.000,0 | 0.0000024s | **~416.666** |

> [!NOTE]
> Os valores de SPS acima de 5.000 são teóricos e dependem puramente da potência da sua CPU (como o Mac M4). No modo INF, o agente processa quase **meio milhão de decisões por segundo**, o que torna o treinamento em grades gigantes (como 150x150) extremamente rápido.

---

## 📄 Licença

Este projeto está sob a licença **GNU General Public License v3.0**. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.
