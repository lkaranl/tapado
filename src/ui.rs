use bevy::prelude::*;

use crate::agent::{AgentEvent, AgentStats, Paused, TurboMode};
use crate::qlearning::QTable;

// Marcadores de texto do HUD
#[derive(Component)]
pub struct HudEpisode;
#[derive(Component)]
pub struct HudSteps;
#[derive(Component)]
pub struct HudBest;
#[derive(Component)]
pub struct HudEpsilon;
#[derive(Component)]
pub struct HudMessage;
#[derive(Component)]
pub struct HudStatus;
#[derive(Component)]
pub struct HudEpsilonBar;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_hud)
            .add_systems(Update, update_hud);
    }
}

const PANEL_X: f32 = 380.0;

fn setup_hud(mut commands: Commands) {
    // Painel de fundo (glassmorphism simulado)
    commands.spawn((
        Sprite {
            color: Color::srgba(0.06, 0.07, 0.15, 0.95),
            custom_size: Some(Vec2::new(240.0, 680.0)),
            ..default()
        },
        Transform::from_xyz(PANEL_X, 0.0, 0.8),
    ));

    // Borda do painel
    commands.spawn((
        Sprite {
            color: Color::srgba(0.3, 0.5, 1.0, 0.3),
            custom_size: Some(Vec2::new(242.0, 682.0)),
            ..default()
        },
        Transform::from_xyz(PANEL_X, 0.0, 0.79),
    ));

    // Barra de epsilon (fundo)
    commands.spawn((
        Sprite {
            color: Color::srgba(0.15, 0.15, 0.25, 1.0),
            custom_size: Some(Vec2::new(180.0, 12.0)),
            ..default()
        },
        Transform::from_xyz(PANEL_X, -120.0, 0.85),
    ));

    // Barra de epsilon (preenchimento)
    commands.spawn((
        Sprite {
            color: Color::srgb(0.4, 0.8, 0.3),
            custom_size: Some(Vec2::new(180.0, 12.0)),
            ..default()
        },
        Transform::from_xyz(PANEL_X - 90.0, -120.0, 0.86),
        HudEpsilonBar,
    ));

    let text_style_title = TextFont {
        font_size: 22.0,
        ..default()
    };
    let text_style_label = TextFont {
        font_size: 14.0,
        ..default()
    };
    let text_style_value = TextFont {
        font_size: 18.0,
        ..default()
    };
    let text_style_msg = TextFont {
        font_size: 15.0,
        ..default()
    };

    // Título
    commands.spawn((
        Text2d::new("TAPADO"),
        text_style_title.clone(),
        TextColor(Color::srgb(0.965, 0.788, 0.055)),
        Transform::from_xyz(PANEL_X, 240.0, 0.9),
    ));

    commands.spawn((
        Text2d::new("Q-Learning Simulator"),
        TextFont {
            font_size: 11.0,
            ..default()
        },
        TextColor(Color::srgba(0.6, 0.6, 0.8, 0.8)),
        Transform::from_xyz(PANEL_X, 220.0, 0.9),
    ));

    // Separador
    commands.spawn((
        Sprite {
            color: Color::srgba(0.4, 0.3, 0.8, 0.5),
            custom_size: Some(Vec2::new(180.0, 1.0)),
            ..default()
        },
        Transform::from_xyz(PANEL_X, 208.0, 0.9),
    ));

    // Episódio
    commands.spawn((
        Text2d::new("EPISÓDIO"),
        text_style_label.clone(),
        TextColor(Color::srgba(0.7, 0.7, 0.9, 0.9)),
        Transform::from_xyz(PANEL_X, 185.0, 0.9),
    ));
    commands.spawn((
        Text2d::new("1"),
        text_style_value.clone(),
        TextColor(Color::WHITE),
        Transform::from_xyz(PANEL_X, 163.0, 0.9),
        HudEpisode,
    ));

    // Passos
    commands.spawn((
        Text2d::new("PASSOS"),
        text_style_label.clone(),
        TextColor(Color::srgba(0.7, 0.7, 0.9, 0.9)),
        Transform::from_xyz(PANEL_X, 138.0, 0.9),
    ));
    commands.spawn((
        Text2d::new("0"),
        text_style_value.clone(),
        TextColor(Color::WHITE),
        Transform::from_xyz(PANEL_X, 116.0, 0.9),
        HudSteps,
    ));

    // Melhor
    commands.spawn((
        Text2d::new("MELHOR"),
        text_style_label.clone(),
        TextColor(Color::srgba(0.7, 0.7, 0.9, 0.9)),
        Transform::from_xyz(PANEL_X, 90.0, 0.9),
    ));
    commands.spawn((
        Text2d::new("—"),
        text_style_value.clone(),
        TextColor(Color::srgb(0.965, 0.788, 0.055)),
        Transform::from_xyz(PANEL_X, 68.0, 0.9),
        HudBest,
    ));

    // Separador
    commands.spawn((
        Sprite {
            color: Color::srgba(0.4, 0.3, 0.8, 0.5),
            custom_size: Some(Vec2::new(180.0, 1.0)),
            ..default()
        },
        Transform::from_xyz(PANEL_X, 48.0, 0.9),
    ));

    // Epsilon label
    commands.spawn((
        Text2d::new("EPSILON (ε)"),
        text_style_label.clone(),
        TextColor(Color::srgba(0.7, 0.7, 0.9, 0.9)),
        Transform::from_xyz(PANEL_X, 28.0, 0.9),
    ));
    commands.spawn((
        Text2d::new("1.000"),
        text_style_value.clone(),
        TextColor(Color::srgb(0.4, 0.8, 0.3)),
        Transform::from_xyz(PANEL_X, 6.0, 0.9),
        HudEpsilon,
    ));

    // Barra de epsilon label
    commands.spawn((
        Sprite {
            color: Color::srgba(0.4, 0.3, 0.8, 0.5),
            custom_size: Some(Vec2::new(180.0, 1.0)),
            ..default()
        },
        Transform::from_xyz(PANEL_X, -140.0, 0.9),
    ));

    // Mensagem cômica
    commands.spawn((
        Text2d::new("Que burrada!"),
        text_style_msg.clone(),
        TextColor(Color::srgb(1.0, 0.6, 0.2)),
        Transform::from_xyz(PANEL_X, -175.0, 0.9),
        HudMessage,
    ));

    // Status (pausa/turbo)
    commands.spawn((
        Text2d::new(""),
        TextFont {
            font_size: 13.0,
            ..default()
        },
        TextColor(Color::srgba(0.5, 0.9, 0.5, 0.9)),
        Transform::from_xyz(PANEL_X, -210.0, 0.9),
        HudStatus,
    ));

    // Separador
    commands.spawn((
        Sprite {
            color: Color::srgba(0.4, 0.3, 0.8, 0.5),
            custom_size: Some(Vec2::new(180.0, 1.0)),
            ..default()
        },
        Transform::from_xyz(PANEL_X, -230.0, 0.9),
    ));

    // Controles
    commands.spawn((
        Text2d::new("[SPACE] Pausar"),
        TextFont {
            font_size: 12.0,
            ..default()
        },
        TextColor(Color::srgba(0.5, 0.5, 0.7, 0.8)),
        Transform::from_xyz(PANEL_X, -252.0, 0.9),
    ));
    commands.spawn((
        Text2d::new("[T] Turbo (8/16/32/64/1K/5Kx)"),
        TextFont {
            font_size: 12.0,
            ..default()
        },
        TextColor(Color::srgba(0.5, 0.5, 0.7, 0.8)),
        Transform::from_xyz(PANEL_X, -270.0, 0.9),
    ));
}

fn update_hud(
    stats: Res<AgentStats>,
    qtable: Res<QTable>,
    paused: Res<Paused>,
    turbo: Res<TurboMode>,
    mut q_episode: Query<&mut Text2d, (With<HudEpisode>, Without<HudSteps>, Without<HudBest>, Without<HudEpsilon>, Without<HudMessage>, Without<HudStatus>)>,
    mut q_steps: Query<&mut Text2d, (With<HudSteps>, Without<HudEpisode>, Without<HudBest>, Without<HudEpsilon>, Without<HudMessage>, Without<HudStatus>)>,
    mut q_best: Query<&mut Text2d, (With<HudBest>, Without<HudEpisode>, Without<HudSteps>, Without<HudEpsilon>, Without<HudMessage>, Without<HudStatus>)>,
    mut q_epsilon: Query<&mut Text2d, (With<HudEpsilon>, Without<HudEpisode>, Without<HudSteps>, Without<HudBest>, Without<HudMessage>, Without<HudStatus>)>,
    mut q_message: Query<(&mut Text2d, &mut TextColor), (With<HudMessage>, Without<HudEpisode>, Without<HudSteps>, Without<HudBest>, Without<HudEpsilon>, Without<HudStatus>)>,
    mut q_status: Query<&mut Text2d, (With<HudStatus>, Without<HudEpisode>, Without<HudSteps>, Without<HudBest>, Without<HudEpsilon>, Without<HudMessage>)>,
    mut q_bar: Query<(&mut Sprite, &mut Transform), With<HudEpsilonBar>>,
) {
    // Episódio
    if let Ok(mut text) = q_episode.get_single_mut() {
        text.0 = format!("{}", stats.episode);
    }

    // Passos
    if let Ok(mut text) = q_steps.get_single_mut() {
        text.0 = format!("{}", stats.steps);
    }

    // Melhor
    if let Ok(mut text) = q_best.get_single_mut() {
        if stats.best_steps == u32::MAX {
            text.0 = "—".to_string();
        } else {
            text.0 = format!("{} passos", stats.best_steps);
        }
    }

    // Epsilon
    let eps = qtable.epsilon;
    if let Ok(mut text) = q_epsilon.get_single_mut() {
        if let Some(ep) = stats.epsilon_converged_at {
            text.0 = format!("{:.3} (no Ep. {})", eps, ep);
        } else {
            text.0 = format!("{:.3}", eps);
        }
    }

    // Barra de epsilon
    if let Ok((mut sprite, mut transform)) = q_bar.get_single_mut() {
        let width = 180.0 * eps;
        sprite.custom_size = Some(Vec2::new(width, 12.0));
        transform.translation.x = PANEL_X - 90.0 + width / 2.0;

        // Cor muda de verde → amarelo → vermelho conforme epsilon sobe
        let r = eps;
        let g = 1.0 - eps * 0.5;
        sprite.color = Color::srgb(r * 0.6 + 0.1, g * 0.8 + 0.1, 0.2);
    }

    // Mensagem cômica
    if let Ok((mut text, mut color)) = q_message.get_single_mut() {
        let (msg, col) = match stats.last_event {
            AgentEvent::GoalReached => ("Cheguei la! Sucesso!", Color::srgb(0.3, 1.0, 0.4)),
            AgentEvent::Crashed => match eps {
                e if e > 0.6 => ("Ai, a parede!", Color::srgb(1.0, 0.4, 0.3)),
                _ => ("Erro calculado.", Color::srgb(1.0, 0.5, 0.3)),
            },
            AgentEvent::Normal => match eps {
                e if e > 0.8 => ("Que burrada!", Color::srgb(1.0, 0.6, 0.2)),
                e if e > 0.5 => ("Hm, to pensando...", Color::srgb(0.9, 0.8, 0.3)),
                e if e > 0.25 => ("Comeco a entender...", Color::srgb(0.5, 0.9, 0.4)),
                _ => ("Sou um genio!", Color::srgb(0.2, 0.8, 1.0)),
            },
        };
        text.0 = msg.to_string();
        color.0 = col;
    }

    // Status (pausa/turbo)
    if let Ok(mut text) = q_status.get_single_mut() {
        text.0 = match (paused.0, turbo.0) {
            (true, _) => "PAUSADO".to_string(),
            (false, 1) => "TURBO 8x".to_string(),
            (false, 2) => "TURBO 16x".to_string(),
            (false, 3) => "TURBO 32x".to_string(),
            (false, 4) => "TURBO 64x".to_string(),
            (false, 5) => "TURBO 1000x".to_string(),
            (false, 6) => "TURBO 5000x".to_string(),
            (false, _) => String::new(),
        };
    }
}
