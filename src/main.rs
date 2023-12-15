use bevy::prelude::*;

struct BallGamePlugin;

impl Plugin for BallGamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_assets).add_systems(Update, spawn_ball_on_click);
    }
}

struct Settings {
    play_area_size: UVec2,
}

#[derive(Component)]
struct BallIndex(usize);

impl std::ops::Deref for BallIndex {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

#[derive(Resource)]
struct BallLevels(Vec<(Color, f32)>);

impl std::ops::Deref for BallLevels {
    type Target = Vec<(Color, f32)>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Resource)]
struct BallMeshes(Vec<Handle<Mesh>>);

impl BallMeshes {
    // TODO: automatic updating of assets (based on lods)

    fn init(mut commands: Commands, mut assets: ResMut<Assets<Mesh>>, mut levels: Res<BallLevels>) {
        let radii = levels.iter().map(|(_, r)| r).collect::<HashSet>();
        let handles = Vec::with_capacity(radii.len());

        for radius in radii {
            let mesh = shape::Circle {
                radius,
                vertices: 32 * (radius.powf(0.4) as usize),
            };
            handles.push(meshes.add(mesh));
        }
    }
}

#[derive(Resource)]
struct Stages(Vec<(Handle<Mesh>, Handle<ColorMaterial>)>);

impl std::ops::Index<BallIndex> for Stages {
    type Output = Handle<Mesh>;

    fn index(&self, index: BallIndex) -> Self::Output {
        self.0[index].clone()
    }
}

impl Stages {
    fn init(mut commands: Commands, meshes: Res<BallMeshes>, mut materials: ResMut<Assets<ColorMaterial>>) {
        let mut stages = vec![vec![], vec![]];
        let mut radii = [0.0; 10];

        for i, r in radii.iter_mut().enumerate() {
            *r = (i as f32 / std::f32::consts::PI).sqrt();
        }
        for i in 0..10 {
            let ball = (
                Color::hsl((i as f32) * 36.0, 100.0, 50.0),
                radii[i],
            );
            stages[0].push(level);
        }
        for f in (0..10).map(|i| i as f32) {
            let ball = (
                Color::hsl(((i as f32) + 0.5) * 36.0, 100.0, 50.0),
                radii[i],
            );
            stages[1].push(level);
        }
        commands.insert_resource(Self(stages));
    }
}

impl std::default::Default for Stages {
    fn default() -> Self {
        
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
    mut handles: ResMut<BallLods>,
) {
    handles.balls = (0..20)
        .map(|i| {
            let vertices = 2 + (2.0).pow(i) as usize;

            assets
                .add(
                    shape::Circle { radius: 1.0, vertices, }
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
