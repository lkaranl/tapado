use bevy::prelude::*;

pub const GRID_W: usize = 10;
pub const GRID_H: usize = 10;
pub const CELL_SIZE: f32 = 54.0;
pub const GRID_OFFSET_X: f32 = -360.0;
pub const GRID_OFFSET_Y: f32 = -243.0;

// Paleta de cores
pub const COLOR_EMPTY: Color = Color::srgb(0.118, 0.129, 0.188);
pub const COLOR_WALL: Color = Color::srgb(0.235, 0.169, 0.369);
pub const COLOR_GOAL: Color = Color::srgb(0.965, 0.788, 0.055);
pub const COLOR_TRAIL: Color = Color::srgba(0.2, 0.8, 0.4, 0.25);
pub const COLOR_AGENT: Color = Color::srgb(0.18, 0.8, 0.95);
pub const COLOR_AGENT_CRASH: Color = Color::srgb(1.0, 0.25, 0.25);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CellType {
    Empty,
    Wall,
    Goal,
}

#[derive(Resource)]
pub struct GridMap {
    pub cells: [[CellType; GRID_W]; GRID_H],
}

impl Default for GridMap {
    fn default() -> Self {
        use CellType::*;
        #[rustfmt::skip]
        let cells = [
            [Empty, Empty, Empty, Wall,  Empty, Empty, Empty, Empty, Empty, Empty],
            [Empty, Wall,  Empty, Wall,  Empty, Wall,  Wall,  Empty, Empty, Empty],
            [Empty, Wall,  Empty, Empty, Empty, Empty, Wall,  Empty, Wall,  Empty],
            [Empty, Empty, Empty, Wall,  Wall,  Empty, Empty, Empty, Wall,  Empty],
            [Empty, Wall,  Wall,  Empty, Empty, Empty, Wall,  Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Wall,  Empty, Wall,  Wall,  Empty, Empty],
            [Empty, Wall,  Empty, Empty, Empty, Empty, Empty, Empty, Wall,  Empty],
            [Empty, Wall,  Empty, Wall,  Wall,  Empty, Wall,  Empty, Empty, Empty],
            [Empty, Empty, Empty, Empty, Empty, Empty, Empty, Wall,  Empty, Empty],
            [Empty, Empty, Wall,  Empty, Empty, Empty, Empty, Empty, Empty, Goal ],
        ];
        GridMap { cells }
    }
}

impl GridMap {
    pub fn cell(&self, x: usize, y: usize) -> CellType {
        self.cells[y][x]
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        self.cells[y][x] != CellType::Wall
    }
}

// Componentes de entidades do grid
#[derive(Component)]
#[allow(dead_code)]
pub struct GridCell {
    pub x: usize,
    pub y: usize,
}

#[derive(Component)]
pub struct TrailCell;

#[derive(Component)]
pub struct GoalPulse;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GridMap>()
            .add_systems(Startup, setup_grid)
            .add_systems(Update, (pulse_goal, fade_trails));
    }
}

pub fn grid_to_world(x: usize, y: usize) -> Vec3 {
    Vec3::new(
        GRID_OFFSET_X + x as f32 * CELL_SIZE,
        GRID_OFFSET_Y + y as f32 * CELL_SIZE,
        0.0,
    )
}

fn setup_grid(mut commands: Commands, grid: Res<GridMap>) {
    for y in 0..GRID_H {
        for x in 0..GRID_W {
            let cell_type = grid.cell(x, y);
            let color = match cell_type {
                CellType::Empty => COLOR_EMPTY,
                CellType::Wall => COLOR_WALL,
                CellType::Goal => COLOR_GOAL,
            };
            let pos = grid_to_world(x, y);

            let mut entity = commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::splat(CELL_SIZE - 2.0)),
                    ..default()
                },
                Transform::from_translation(pos),
                GridCell { x, y },
            ));

            if cell_type == CellType::Goal {
                entity.insert(GoalPulse);
            }
        }
    }
}

fn pulse_goal(time: Res<Time>, mut query: Query<&mut Transform, With<GoalPulse>>) {
    for mut transform in &mut query {
        let scale = 1.0 + 0.06 * (time.elapsed_secs() * 3.0).sin();
        transform.scale = Vec3::splat(scale);
    }
}

fn fade_trails(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Sprite), With<TrailCell>>,
) {
    for (entity, mut sprite) in &mut query {
        let alpha = sprite.color.alpha() - time.delta_secs() * 0.4;
        if alpha <= 0.01 {
            commands.entity(entity).despawn();
        } else {
            sprite.color.set_alpha(alpha);
        }
    }
}
