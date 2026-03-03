use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
}

impl Action {
    pub const ALL: [Action; 4] = [Action::Up, Action::Down, Action::Left, Action::Right];
}

type State = (usize, usize);

// Hiperparâmetros do Q-Learning - Ajuste aqui!
pub const Q_ALPHA: f32 = 0.15;         // Taxa de aprendizado (quão rápido aceita novas informações)
pub const Q_GAMMA: f32 = 0.99;         // Fator de desconto (importância das recompensas futuras)
pub const EPSILON_START: f32 = 1.0;    // Taxa de exploração inicial (1.0 = 100% aleatório)
pub const EPSILON_MIN: f32 = 0.0000000000000001;      // Menor taxa possível de exploração (ex: 5%)
pub const EPSILON_DECAY: f32 = 0.0015; // O quanto cai a cada episódio

#[derive(Resource)]
pub struct QTable {
    pub table: HashMap<(State, Action), f32>,
    pub epsilon: f32,
    pub alpha: f32,
    pub gamma: f32,
    epsilon_min: f32,
    epsilon_decay: f32,
}

impl Default for QTable {
    fn default() -> Self {
        QTable {
            table: HashMap::new(),
            epsilon: EPSILON_START,
            alpha: Q_ALPHA,
            gamma: Q_GAMMA,
            epsilon_min: EPSILON_MIN,
            epsilon_decay: EPSILON_DECAY,
        }
    }
}

impl QTable {
    pub fn get(&self, state: State, action: Action) -> f32 {
        *self.table.get(&(state, action)).unwrap_or(&0.0)
    }

    pub fn set(&mut self, state: State, action: Action, value: f32) {
        self.table.insert((state, action), value);
    }

    pub fn best_action(&self, state: State) -> Action {
        Action::ALL
            .iter()
            .copied()
            .max_by(|&a, &b| {
                self.get(state, a)
                    .partial_cmp(&self.get(state, b))
                    .unwrap()
            })
            .unwrap_or(Action::Up)
    }

    pub fn best_value(&self, state: State) -> f32 {
        Action::ALL
            .iter()
            .map(|&a| self.get(state, a))
            .fold(f32::NEG_INFINITY, f32::max)
    }

    pub fn decay_epsilon(&mut self) {
        self.epsilon = (self.epsilon - self.epsilon_decay).max(self.epsilon_min);
    }
}

pub fn pick_action(qtable: &QTable, state: State) -> Action {
    let mut rng = rand::thread_rng();
    if rng.r#gen::<f32>() < qtable.epsilon {
        // Exploração aleatória
        let idx = rng.gen_range(0..4);
        Action::ALL[idx]
    } else {
        // Exploração gananciosa
        qtable.best_action(state)
    }
}

pub fn update_qtable(
    qtable: &mut QTable,
    state: State,
    action: Action,
    reward: f32,
    next_state: State,
) {
    let current_q = qtable.get(state, action);
    let best_next = qtable.best_value(next_state);
    // Equação de Bellman
    let new_q = current_q + qtable.alpha * (reward + qtable.gamma * best_next - current_q);
    qtable.set(state, action, new_q);
}

pub struct QLearningPlugin;

impl Plugin for QLearningPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<QTable>();
    }
}
