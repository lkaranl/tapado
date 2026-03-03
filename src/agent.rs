use bevy::prelude::*;

use crate::grid::{
    grid_to_world, CellType, GridMap, TrailCell, COLOR_AGENT, COLOR_AGENT_CRASH, COLOR_TRAIL,
    GRID_H, GRID_W,
};
use crate::qlearning::{pick_action, update_qtable, Action, QTable};

#[derive(Resource)]
pub struct AgentStats {
    pub episode: u32,
    pub steps: u32,
    pub total_reward: f32,
    pub best_steps: u32,
    pub last_event: AgentEvent,
    pub crash_flash: f32, // timer do flash vermelho
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
        }
    }
}

#[derive(Resource)]
pub struct StepTimer(pub Timer);

#[derive(Resource)]
pub struct TurboMode(pub u32);

#[derive(Resource)]
pub struct Paused(pub bool);

#[derive(Component)]
pub struct AgentMarker {
    pub x: usize,
    pub y: usize,
}

const MAX_STEPS: u32 = 200;
const STEP_INTERVAL_NORMAL: f32 = 0.12;
const REWARD_GOAL: f32 = 100.0;
const REWARD_WALL: f32 = -10.0;
const REWARD_STEP: f32 = -0.5;

pub struct AgentPlugin;

impl Plugin for AgentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AgentStats>()
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

fn spawn_agent(mut commands: Commands) {
    let pos = grid_to_world(0, 0);
    commands.spawn((
        Sprite {
            color: COLOR_AGENT,
            custom_size: Some(Vec2::splat(38.0)),
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
) {
    if keys.just_pressed(KeyCode::KeyT) {
        turbo.0 = (turbo.0 + 1) % 6;
        let interval = match turbo.0 {
            0 => STEP_INTERVAL_NORMAL,
            1 => STEP_INTERVAL_NORMAL / 8.0,
            2 => STEP_INTERVAL_NORMAL / 16.0,
            3 => STEP_INTERVAL_NORMAL / 32.0,
            4 => STEP_INTERVAL_NORMAL / 64.0,
            5 => STEP_INTERVAL_NORMAL / 1000.0,
            _ => STEP_INTERVAL_NORMAL,
        };
        timer.0 = Timer::from_seconds(interval, TimerMode::Repeating);
    }

    if keys.just_pressed(KeyCode::Space) {
        paused.0 = !paused.0;
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
        let valid = nx < GRID_W && ny < GRID_H && grid.is_walkable(nx, ny);

        let (next_state, reward, done) = if !valid {
            // Bateu na parede ou saiu dos limites
            stats.last_event = AgentEvent::Crashed;
            stats.crash_flash = 0.35;
            (state, REWARD_WALL, false)
        } else {
            let next = (nx, ny);
            let cell = grid.cell(nx, ny);

            // Deixar trilha
            let trail_pos = grid_to_world(agent.x, agent.y).with_z(0.5);
            commands.spawn((
                Sprite {
                    color: COLOR_TRAIL,
                    custom_size: Some(Vec2::splat(52.0)),
                    ..default()
                },
                Transform::from_translation(trail_pos),
                TrailCell,
            ));

            agent.x = nx;
            agent.y = ny;
            transform.translation = grid_to_world(nx, ny).with_z(1.0);

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
        if done || stats.steps >= MAX_STEPS {
            stats.episode += 1;
            stats.steps = 0;
            stats.total_reward = 0.0;

            // Resetar agente para origem
            agent.x = 0;
            agent.y = 0;
            transform.translation = grid_to_world(0, 0).with_z(1.0);

            // Decair epsilon
            qtable.decay_epsilon();
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
