use bevy::prelude::*;
use heron::prelude::*;

const CURRENT: f32 = 3.0;
const ITERS: u32 = 5;
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(simulate_points.system().label("spoints"))
            .add_system(simulate_sticks.system().after("spoints"));
    }
}

pub struct RopePoint {
    previous_position: Vec3,
    locked: bool,
}

pub struct RopeStick {
    pointA_entity: Entity,
    pointB_entity: Entity,
    length: f32,
}

fn simulate_points(time: Res<Time>, mut query: Query<(&mut Transform, &mut RopePoint)>) {
    for (mut transform, mut point) in query.iter_mut() {
        if point.locked {
            continue;
        }

        let position_before_update = transform.translation;
        transform.translation =
            transform.translation + transform.translation - point.previous_position;
        transform.translation =
            Vec3::new(0., 0., 1.) * CURRENT * time.delta_seconds() * time.delta_seconds();
        point.previous_position = position_before_update;
    }
}

fn simulate_sticks(
    mut query: Query<&mut RopeStick>,
    mut point_query: Query<(&mut Transform, &RopePoint)>,
) {
    for i in 0..ITERS {
        for mut stick in query.iter_mut() {
            let Ok((mut pointA_transform, pointA)) = point_query.get_mut(stick.pointA_entity);
            let Ok((mut pointB_transform, pointB)) = point_query.get_mut(stick.pointB_entity);

            let stickCenter = (pointA_transform.translation + pointB_transform.translation) / 2.0;
            let stickDir =
                (pointA_transform.translation - pointB_transform.translation).normalize();
            if !pointA.locked {
                pointA_transform.translation = stickCenter + stickDir * stick.length / 2.0;
            }
            if !pointB.locked {
                pointB_transform.translation = stickCenter - stickDir * stick.length / 2.0;
            }
        }
    }
}
