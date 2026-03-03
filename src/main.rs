mod agent;
mod grid;
mod qlearning;
mod ui;

use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};

use agent::AgentPlugin;
use grid::GridPlugin;
use qlearning::QLearningPlugin;
use ui::UiPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "🧠 TAPADO — Q-Learning Simulator".to_string(),
                        resolution: WindowResolution::new(1024.0, 720.0),
                        present_mode: PresentMode::AutoVsync,
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .insert_resource(ClearColor(Color::srgb(0.02, 0.02, 0.05)))
        .add_plugins((QLearningPlugin, GridPlugin, AgentPlugin, UiPlugin))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
