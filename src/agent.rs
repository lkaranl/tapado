use bevy::prelude::*;

use crate::grid::{
    CellType, GridMap, TrailCell, COLOR_AGENT, COLOR_AGENT_CRASH, COLOR_TRAIL,
};
use crate::qlearning::{pick_action, update_qtable, Action, QTable, Q_ALPHA, Q_GAMMA, EPSILON_START, EPSILON_MIN, EPSILON_DECAY};

#[derive(Resource)]
pub struct AgentStats {
    pub episode: u32,
    pub steps: u32,
    pub total_reward: f32,
    pub best_steps: u32,
    pub last_event: AgentEvent,
    pub crash_flash: f32, // timer do flash vermelho
    pub epsilon_converged_at: Option<u32>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AgentEvent {
    Normal,
    Crashed,
    GoalReached,
}

impl Default for AgentStats {
    fn default() -> Self {
        AgentStats {
            episode: 1,
            steps: 0,
            total_reward: 0.0,
            best_steps: u32::MAX,
            last_event: AgentEvent::Normal,
            crash_flash: 0.0,
            epsilon_converged_at: None,
        }
    }
}

#[derive(Resource)]
pub struct StepTimer(pub Timer);

#[derive(Resource)]
pub struct TurboMode(pub u32);

#[derive(Resource)]
pub struct Paused(pub bool);

/// Hiperparâmetros editáveis em tempo real via painel (tecla P)
#[derive(Resource)]
pub struct HyperParams {
    pub alpha: f32,
    pub gamma: f32,
    pub epsilon_start: f32,
    pub epsilon_min: f32,
    pub epsilon_decay: f32,
    pub selected: usize, // 0=alpha, 1=gamma, 2=eps_start, 3=eps_min, 4=eps_decay
    pub visible: bool,
}

impl Default for HyperParams {
    fn default() -> Self {
        HyperParams {
            alpha: Q_ALPHA,
            gamma: Q_GAMMA,
            epsilon_start: EPSILON_START,
            epsilon_min: EPSILON_MIN,
            epsilon_decay: EPSILON_DECAY,
            selected: 0,
            visible: false,
        }
    }
}

#[derive(Component)]
pub struct AgentMarker {
    pub x: usize,
    pub y: usize,
}

const STEP_INTERVAL_NORMAL: f32 = 0.12;
const REWARD_GOAL: f32 = 100.0;
const REWARD_WALL: f32 = -10.0;
const REWARD_STEP: f32 = -0.5;

pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AgentStats>()
            .init_resource::<HyperParams>()
            .insert_resource(StepTimer(Timer::from_seconds(
                STEP_INTERVAL_NORMAL,
                TimerMode::Repeating,
            )))
            .insert_resource(TurboMode(0))
            .insert_resource(Paused(false))
            .add_systems(Startup, spawn_agent)
            .add_systems(
                Update,
                (
                    handle_input,
                    agent_step.run_if(|p: Res<Paused>| !p.0),
                    update_agent_color,
                ),
            );
    }
}

fn spawn_agent(mut commands: Commands, grid: Res<GridMap>) {
    let pos = grid.to_world(0, 0);
    commands.spawn((
        Sprite {
            color: COLOR_AGENT,
            custom_size: Some(Vec2::splat(grid.cell_size * 0.7)),
            ..default()
        },
        Transform::from_translation(pos.with_z(1.0)),
        AgentMarker { x: 0, y: 0 },
    ));
}

fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut turbo: ResMut<TurboMode>,
    mut paused: ResMut<Paused>,
    mut timer: ResMut<StepTimer>,
    mut qtable: ResMut<QTable>,
    mut stats: ResMut<AgentStats>,
    mut params: ResMut<HyperParams>,
    mut agent_q: Query<(&mut AgentMarker, &mut Transform)>,
    grid: Res<GridMap>,
) {
    if keys.just_pressed(KeyCode::KeyT) {
        turbo.0 = (turbo.0 + 1) % 8;
        let interval = match turbo.0 {
            0 => STEP_INTERVAL_NORMAL,
            1 => STEP_INTERVAL_NORMAL / 8.0,
            2 => STEP_INTERVAL_NORMAL / 16.0,
            3 => STEP_INTERVAL_NORMAL / 32.0,
            4 => STEP_INTERVAL_NORMAL / 64.0,
            5 => STEP_INTERVAL_NORMAL / 1000.0,
            6 => STEP_INTERVAL_NORMAL / 5000.0,
            7 => STEP_INTERVAL_NORMAL / 10000.0,
            _ => STEP_INTERVAL_NORMAL,
        };
        timer.0 = Timer::from_seconds(interval, TimerMode::Repeating);
    }

    if keys.just_pressed(KeyCode::Space) {
        paused.0 = !paused.0;
    }

    // --- Painel de parâmetros ---
    if keys.just_pressed(KeyCode::KeyP) {
        params.visible = !params.visible;
    }

    if params.visible {
        // Navegar entre parâmetros
        if keys.just_pressed(KeyCode::ArrowUp) {
            params.selected = params.selected.saturating_sub(1);
        }
        if keys.just_pressed(KeyCode::ArrowDown) {
            params.selected = (params.selected + 1).min(4);
        }

        // Incremento de cada parâmetro
        let delta = match params.selected {
            0 => 0.01,  // alpha
            1 => 0.01,  // gamma
            2 => 0.05,  // epsilon_start
            3 => 0.001, // epsilon_min
            4 => 0.0001,// epsilon_decay
            _ => 0.01,
        };

        if keys.just_pressed(KeyCode::ArrowRight) {
            match params.selected {
                0 => params.alpha = (params.alpha + delta).min(1.0),
                1 => params.gamma = (params.gamma + delta).min(0.9999),
                2 => params.epsilon_start = (params.epsilon_start + delta).min(1.0),
                3 => params.epsilon_min = (params.epsilon_min + delta).min(params.epsilon_start),
                4 => params.epsilon_decay = (params.epsilon_decay + delta).min(0.1),
                _ => {}
            }
            qtable.set_params(params.alpha, params.gamma, params.epsilon_start, params.epsilon_min, params.epsilon_decay);
        }

        if keys.just_pressed(KeyCode::ArrowLeft) {
            match params.selected {
                0 => params.alpha = (params.alpha - delta).max(0.001),
                1 => params.gamma = (params.gamma - delta).max(0.0),
                2 => params.epsilon_start = (params.epsilon_start - delta).max(0.0),
                3 => params.epsilon_min = (params.epsilon_min - delta).max(0.0),
                4 => params.epsilon_decay = (params.epsilon_decay - delta).max(0.0001),
                _ => {}
            }
            qtable.set_params(params.alpha, params.gamma, params.epsilon_start, params.epsilon_min, params.epsilon_decay);
        }
    }

    if keys.just_pressed(KeyCode::KeyR) {
        // Resetar Q-Table usando parâmetros atuais do painel
        let mut new_qtable = QTable::default();
        new_qtable.set_params(params.alpha, params.gamma, params.epsilon_start, params.epsilon_min, params.epsilon_decay);
        new_qtable.epsilon = params.epsilon_start;
        *qtable = new_qtable;
        // Resetar estatísticas
        *stats = AgentStats::default();
        // Resetar posição do agente
        if let Ok((mut agent, mut transform)) = agent_q.get_single_mut() {
            agent.x = 0;
            agent.y = 0;
            transform.translation = grid.to_world(0, 0).with_z(1.0);
        }
    }
}

fn agent_step(
    time: Res<Time>,
    mut timer: ResMut<StepTimer>,
    mut qtable: ResMut<QTable>,
    mut stats: ResMut<AgentStats>,
    grid: Res<GridMap>,
    mut commands: Commands,
    mut agent_q: Query<(&mut AgentMarker, &mut Transform)>,
) {
    timer.0.tick(time.delta());
    let ticks = timer.0.times_finished_this_tick();
    if ticks == 0 {
        return;
    }

    let (mut agent, mut transform) = agent_q.single_mut();

    for _ in 0..ticks {
        let state = (agent.x, agent.y);
        let action = pick_action(&qtable, state);

        let (nx, ny) = apply_action(agent.x, agent.y, action);

        // Verificar limites e paredes
        let valid = nx < grid.width && ny < grid.height && grid.is_walkable(nx, ny);

        let (next_state, reward, done) = if !valid {
            // Bateu na parede ou saiu dos limites
            stats.last_event = AgentEvent::Crashed;
            stats.crash_flash = 0.35;
            (state, REWARD_WALL, false)
        } else {
            let next = (nx, ny);
            let cell = grid.cell(nx, ny);

            // Deixar trilha
            let trail_pos = grid.to_world(agent.x, agent.y).with_z(0.5);
            commands.spawn((
                Sprite {
                    color: COLOR_TRAIL,
                    custom_size: Some(Vec2::splat(grid.cell_size * 0.95)),
                    ..default()
                },
                Transform::from_translation(trail_pos),
                TrailCell,
            ));

            agent.x = nx;
            agent.y = ny;
            transform.translation = grid.to_world(nx, ny).with_z(1.0);

            if cell == CellType::Goal {
                stats.last_event = AgentEvent::GoalReached;
                if stats.steps + 1 < stats.best_steps {
                    stats.best_steps = stats.steps + 1;
                }
                (next, REWARD_GOAL, true)
            } else {
                stats.last_event = AgentEvent::Normal;
                (next, REWARD_STEP, false)
            }
        };

        update_qtable(&mut qtable, state, action, reward, next_state);
        stats.total_reward += reward;
        stats.steps += 1;

        // Fim de episódio
        let max_steps = (grid.width * grid.height) as u32 * 2;
        if done || stats.steps >= max_steps {
            stats.episode += 1;
            stats.steps = 0;
            stats.total_reward = 0.0;

            // Resetar agente para origem
            agent.x = 0;
            agent.y = 0;
            transform.translation = grid.to_world(0, 0).with_z(1.0);

            // Decair epsilon
            let was_converged = qtable.epsilon <= crate::qlearning::EPSILON_MIN;
            qtable.decay_epsilon();
            
            // Registra quando atingiu o piso (e só registra a 1ª vez)
            if !was_converged && qtable.epsilon <= crate::qlearning::EPSILON_MIN && stats.epsilon_converged_at.is_none() {
                stats.epsilon_converged_at = Some(stats.episode);
            }
        }
    }
}

fn update_agent_color(
    time: Res<Time>,
    mut stats: ResMut<AgentStats>,
    mut agent_q: Query<&mut Sprite, With<AgentMarker>>,
) {
    if let Ok(mut sprite) = agent_q.get_single_mut() {
        if stats.crash_flash > 0.0 {
            stats.crash_flash -= time.delta_secs();
            sprite.color = COLOR_AGENT_CRASH;
        } else {
            sprite.color = COLOR_AGENT;
        }
    }
}

pub fn apply_action(x: usize, y: usize, action: Action) -> (usize, usize) {
    match action {
        Action::Up => (x, y.wrapping_add(1)),
        Action::Down => (x, y.wrapping_sub(1)),
        Action::Left => (x.wrapping_sub(1), y),
        Action::Right => (x.wrapping_add(1), y),
    }
}
