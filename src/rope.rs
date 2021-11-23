use crate::player::PlayerLeft;
use crate::player::PlayerRight;
use bevy::prelude::*;
use heron::prelude::*;

const CURRENT: f32 = 100.0;
const ITERS: u32 = 10;
pub struct RopePlugin;
impl Plugin for RopePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage_after(
            "game_setup_actors",
            "game_setup_rope",
            SystemStage::single(spawn_rope.system()),
        )
        .add_system(simulate_points.system().label("spoints"))
        .add_system(simulate_sticks.system().after("spoints"));
    }
}

#[derive(Clone)]
pub struct RopePoint {
    previous_position: Vec3,
    locked: bool,
}

pub struct RopeStick {
    pointA_entity: Entity,
    pointB_entity: Entity,
    length: f32,
}

fn spawn_rope(
    mut commands: Commands,
    query_left: Query<(Entity, &Transform), With<PlayerLeft>>,
    query_right: Query<(Entity, &Transform), With<PlayerRight>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if let (
        Ok((dolphin_left, transform_dolphin_left)),
        Ok((dolphin_right, transform_dolphin_right)),
    ) = (query_left.single(), query_right.single())
    {
        let rope_point_left = RopePoint {
            previous_position: transform_dolphin_left.translation,
            locked: true,
        };
        let rope_point_right = RopePoint {
            previous_position: transform_dolphin_right.translation,
            locked: true,
        };

        commands
            .entity(dolphin_left)
            .insert(rope_point_left.clone());
        commands
            .entity(dolphin_right)
            .insert(rope_point_right.clone());
        let sub = 9;
        let increment = (transform_dolphin_right.translation.x
            - transform_dolphin_left.translation.x)
            / sub as f32;

        let mut points: Vec<Entity> = Vec::new();
        points.push(dolphin_left);
        for i in 1..(sub - 1) {
            let entity = commands
                //.spawn()
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Icosphere {
                        radius: 0.1,
                        subdivisions: 32,
                    })),
                    transform: Transform::from_translation(
                        transform_dolphin_left.translation
                            + Vec3::new(i as f32 * increment, 0., 0.),
                    ),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.5).into()),
                    ..Default::default()
                })
                .insert(RopePoint {
                    previous_position: transform_dolphin_left.translation
                        + Vec3::new(i as f32 * increment, 0., 0.),
                    locked: false,
                })
                .id();
            points.push(entity);
        }
        points.push(dolphin_right);

        for i in 0..(sub - 1) {
            commands.spawn().insert(RopeStick {
                pointA_entity: points[i],
                pointB_entity: points[i + 1],
                length: increment + 1 as f32,
            });
        }
    }
}

fn simulate_points(time: Res<Time>, mut query: Query<(&mut Transform, &mut RopePoint)>) {
    for (mut transform, mut point) in query.iter_mut() {
        if point.locked {
            continue;
        }

        let position_before_update = transform.translation;
        transform.translation =
            transform.translation + transform.translation - point.previous_position;
        transform.translation = transform.translation
            + Vec3::new(0., 0., 1.) * CURRENT * time.delta_seconds() * time.delta_seconds();
        point.previous_position = position_before_update;
    }
}

fn simulate_sticks(
    mut query: Query<&mut RopeStick>,
    mut q: QuerySet<(
        Query<(&Transform, &RopePoint)>,
        Query<(&mut Transform, &RopePoint)>,
    )>,
) {
    for i in 0..ITERS {
        for stick in query.iter_mut() {
            let mut stick_center = Vec3::ZERO;
            let mut stick_dir = Vec3::ZERO;
            if let (Ok((pointA_transform, pointA)), Ok((pointB_transform, pointB))) = (
                q.q0().get(stick.pointA_entity),
                q.q0().get(stick.pointB_entity),
            ) {
                stick_center = (pointA_transform.translation + pointB_transform.translation) / 2.0;
                stick_dir =
                    (pointA_transform.translation - pointB_transform.translation).normalize();
            }
            if let Ok((mut pointA_transform_write, pointA)) =
                q.q1_mut().get_mut(stick.pointA_entity)
            {
                if !pointA.locked {
                    pointA_transform_write.translation =
                        stick_center + stick_dir * stick.length / 2.0;
                }
            }

            if let Ok((mut pointB_transform_write, pointB)) =
                q.q1_mut().get_mut(stick.pointB_entity)
            {
                if !pointB.locked {
                    pointB_transform_write.translation =
                        stick_center - stick_dir * stick.length / 2.0;
                }
            }
        }
    }
}
