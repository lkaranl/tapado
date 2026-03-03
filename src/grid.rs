use bevy::prelude::*;
use rand::Rng;

pub const GRID_W: usize = 50;
pub const GRID_H: usize = 50;

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
    pub width: usize,
    pub height: usize,
    pub cell_size: f32,
    pub offset_x: f32,
    pub offset_y: f32,
    pub cells: Vec<Vec<CellType>>,
}

impl Default for GridMap {
    fn default() -> Self {
        let available_w = 680.0;
        let available_h = 600.0;
        
        let cell_w = available_w / GRID_W as f32;
        let cell_h = available_h / GRID_H as f32;
        let cell_size = cell_w.min(cell_h).min(60.0);
        
        let total_w = cell_size * GRID_W as f32;
        let total_h = cell_size * GRID_H as f32;
        
        let center_x = -130.0;
        let center_y = 0.0;
        
        let offset_x = center_x - (total_w / 2.0) + (cell_size / 2.0);
        let offset_y = center_y - (total_h / 2.0) + (cell_size / 2.0);

        let mut cells = vec![vec![CellType::Empty; GRID_W]; GRID_H];

        if GRID_W == 10 && GRID_H == 10 {
            use CellType::*;
            #[rustfmt::skip]
            let static_cells = [
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
            cells = static_cells.iter().map(|row| row.to_vec()).collect();
        } else {
            let mut rng = rand::thread_rng();
            for y in 0..GRID_H {
                for x in 0..GRID_W {
                    if (x == 0 && y == 0) || (x == GRID_W - 1 && y == GRID_H - 1) {
                        continue;
                    }
                    if rng.gen_bool(0.20) {
                        cells[y][x] = CellType::Wall;
                    }
                }
            }
            cells[GRID_H - 1][GRID_W - 1] = CellType::Goal;
        }

        Self {
            width: GRID_W,
            height: GRID_H,
            cell_size,
            offset_x,
            offset_y,
            cells,
        }
    }
}

impl GridMap {
    pub fn cell(&self, x: usize, y: usize) -> CellType {
        self.cells[y][x]
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        self.cells[y][x] != CellType::Wall
    }

    pub fn to_world(&self, x: usize, y: usize) -> Vec3 {
        Vec3::new(
            self.offset_x + x as f32 * self.cell_size,
            self.offset_y + y as f32 * self.cell_size,
            0.0,
        )
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

fn setup_grid(mut commands: Commands, grid: Res<GridMap>) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let cell_type = grid.cell(x, y);
            let color = match cell_type {
                CellType::Empty => COLOR_EMPTY,
                CellType::Wall => COLOR_WALL,
                CellType::Goal => COLOR_GOAL,
            };
            let pos = grid.to_world(x, y);

            let mut entity = commands.spawn((
                Sprite {
                    color,
                    custom_size: Some(Vec2::splat(grid.cell_size - 2.0)),
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
