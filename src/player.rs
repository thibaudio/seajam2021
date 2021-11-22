use crate::Models;
use crate::Speed;
use bevy::prelude::*;
use heron::prelude::*;

pub struct PlayerLeft;
pub struct PlayerRight;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage(
            "game_setup_actors",
            SystemStage::single(spawn_player.system()),
        )
        .add_system_set(
            SystemSet::new()
                .label("input")
                .with_system(keyboard_input_left.system())
                .with_system(keyboard_input_right.system()),
        )
        .add_system(collision_reader.system());
    }
}

fn spawn_player(mut commands: Commands, models: Res<Models>) {
    let starting_left_position = Vec3::new(-10.0, 0.0, 0.0);
    commands
        .spawn_bundle((
            Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_rotation_y(std::f32::consts::PI / -2.0),
                starting_left_position,
            )),
            GlobalTransform::identity(),
        ))
        .insert(PlayerLeft)
        .insert(Speed(12.0))
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Sphere { radius: 5. })
        .with_children(|parent| {
            parent.spawn_scene(models.player.clone());
        });

    commands
        .spawn_bundle((
            Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_rotation_y(std::f32::consts::PI / -2.0),
                Vec3::new(10.0, 0.0, 0.0),
            )),
            GlobalTransform::identity(),
        ))
        .insert(PlayerRight)
        .insert(Speed(12.0))
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Sphere { radius: 5. })
        .with_children(|parent| {
            parent.spawn_scene(models.player.clone());
        });
}

fn keyboard_input_left(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Speed), With<PlayerLeft>>,
) {
    if keys.pressed(KeyCode::A) && !keys.pressed(KeyCode::D) {
        for (mut transform, speed) in query.iter_mut() {
            *transform = Transform::from_translation(
                Vec3::new(-1.0, 0.0, 0.0) * time.delta_seconds() * speed.0,
            ) * *transform;
        }
    } else if keys.pressed(KeyCode::D) && !keys.pressed(KeyCode::A) {
        for (mut transform, speed) in query.iter_mut() {
            *transform = Transform::from_translation(
                Vec3::new(1.0, 0.0, 0.0) * time.delta_seconds() * speed.0,
            ) * *transform;
        }
    }
}

fn keyboard_input_right(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Speed), With<PlayerRight>>,
) {
    if keys.pressed(KeyCode::Left) && !keys.pressed(KeyCode::Right) {
        for (mut transform, speed) in query.iter_mut() {
            *transform = Transform::from_translation(
                Vec3::new(-1.0, 0.0, 0.0) * time.delta_seconds() * speed.0,
            ) * *transform;
        }
    } else if keys.pressed(KeyCode::Right) && !keys.pressed(KeyCode::Left) {
        for (mut transform, speed) in query.iter_mut() {
            *transform = Transform::from_translation(
                Vec3::new(1.0, 0.0, 0.0) * time.delta_seconds() * speed.0,
            ) * *transform;
        }
    }
}

fn collision_reader(mut ev_collision: EventReader<CollisionEvent>) {
    for ev in ev_collision.iter() {
        match ev {
            CollisionEvent::Started(data1, data2) => {
                println!(
                    "Entity {:?} and {:?} started to collide",
                    data1.rigid_body_entity(),
                    data2.rigid_body_entity()
                )
            }
            CollisionEvent::Stopped(data1, data2) => {
                println!(
                    "Entity {:?} and {:?} stopped to collide",
                    data1.rigid_body_entity(),
                    data2.rigid_body_entity()
                )
            }
        }
    }
}
