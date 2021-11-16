mod player;
mod rope;

use crate::player::PlayerPlugin;
use bevy::core::FixedTimestep;
use bevy::{pbr::AmbientLight, prelude::*};
use heron::prelude::*;
use rand::Rng;

const TRASH_SPAWN_FREQUENCY: f64 = 120.0 / 60.0;
const DOLPHIN_MODEL: &str = "models/dolphin/dolphin.gltf#Scene0";

//region: Resources
pub struct Models {
    player: Handle<Scene>,
}
//endregion Resources

//region: Components
struct Speed(f32);

struct Trash;
//endregion: Components

fn main() {
    App::build()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0 / 5.0f32,
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::new()
                .with_system(spawn_trash.system())
                .with_run_criteria(FixedTimestep::step(TRASH_SPAWN_FREQUENCY)),
        )
        .add_system(move_trash.system())
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.0, 5.0, 20.0)
            .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        ..Default::default()
    });
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(3.0, 5.0, 3.0),
        ..Default::default()
    });

    commands.insert_resource(Models {
        player: asset_server.load(DOLPHIN_MODEL),
    })
}

fn spawn_trash(mut commands: Commands, ass: Res<AssetServer>) {
    let mut rng = rand::thread_rng();
    for _i in 0..rng.gen_range(1..4) {
        let trash_gltf = ass.load("models/trash/trash.gltf#Scene0");
        commands
            .spawn_bundle((
                Transform::from_xyz(rng.gen_range(-10.0..10.0), 0.0, -30.0),
                GlobalTransform::identity(),
            ))
            .insert(Trash)
            //.insert(Speed(12.0))
            .insert(Velocity::from_linear(Vec3::Z * 12.0))
            .insert(RigidBody::Dynamic)
            .insert(CollisionShape::Cuboid {
                half_extends: Vec3::new(0.5, 0.5, 0.5),
                border_radius: None,
            })
            .with_children(|parent| {
                parent.spawn_scene(trash_gltf);
            });
    }
    let percentage = rng.gen_range(0..100);
    let treshold = 70;
    if percentage > treshold {
        let obstacle_gltf = ass.load("models/dolphin/dolphin.gltf#Scene0");
        commands
            .spawn_bundle((
                Transform::from_xyz(rng.gen_range(-10.0..10.0), 0.0, -30.0),
                GlobalTransform::identity(),
            ))
            .insert(Trash)
            .insert(Speed(12.0))
            .with_children(|parent| {
                parent.spawn_scene(obstacle_gltf);
            });
    }
}

fn move_trash(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &Speed), With<Trash>>,
) {
    for (e, mut transform, speed) in query.iter_mut() {
        *transform =
            Transform::from_translation(Vec3::new(0.0, 0.0, 1.0) * time.delta_seconds() * speed.0)
                * *transform;
        if transform.translation.z > 30.0 {
            commands.entity(e).despawn();
        }
    }
}
