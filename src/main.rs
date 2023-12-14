use bevy::prelude::*;

struct BallGamePlugin;

impl Plugin for BallGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(schedule, systems)
    }
}

struct Level {
    color: Rgb,
    radius: f32,
}

impl Level {
    fn new(color: Color, radius: f32) -> Self {
        Self { color, radius }
    }
}

#[derive(Resource)]
struct Stages(Vec<Vec<Level>>);

impl std::default::Default for Stages {
    fn default() -> Self {
        let mut stages = vec![vec![], vec![]];
        let radius = |f: f32| (f / std::f32::consts::PI).sqrt();

        for f in (0..10).map(|i| i as f32) {
            let level = Level::new(
                Color::hsl(f * 36.0, 100.0, 50.0),
                radius(f),
            );
            stages[0].push(level);
        }
        for f in (0..10).map(|i| i as f32) {
            let level = Level::new(
                Color::hsl((f + 0.5) * 36.0, 100.0, 50.0),
                radius(f),
            );
            stages[1].push(level);
        }
        Self(stages)
    }
}

impl std::ops::Deref for Stages {
    type Target = Vec<Vec<Level>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Stages {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Default, Component)]
struct Ball;

#[derive(Default, Component)]
struct Tier {
    level: u32,
    stage: u32,
}

#[derive(Default, Bundle)]
struct BallBundle {
    marker: Ball,
    tier: Tier,
}

impl BallBundle {
    fn new(level: u32, stage: u32) -> Self {
        Self {
            marker: Ball,
            tier: Tier { level, stage },
        }
    }
}

fn spawn_ball_on_click(mut commands: Commands, mouse_input: Input<MouseButton>) {
    if !mouse_input.pressed(MouseButton::Left) {
        return;
    }
    commands.spawn(BallBundle::default());
}

fn init_assets(
    mut assets: ResMut<Assets<Mesh>>,
    mut handles: ResMut<BallMeshes>,
    stages: Res<Stages>,
) {
    handles.balls = stages
        .iter()
        .map(|r| {
            assets
                .add(
                    shape::Circle {
                        radius: 0.5,
                        vertices: BALL_MIN_VERTICES * r.powf(0.4) as usize,
                    }
                    .into(),
                )
                .into()
        })
        .collect();
}

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, BallGamePlugin))
        .run();
}
