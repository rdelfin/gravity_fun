use bevy::prelude::*;
use bevy_rapier3d::prelude::{RigidBodyForces, RigidBodyMassProps, RigidBodyVelocity};

pub fn ball_move_system(
    keys: Res<Input<KeyCode>>,
    mut rigid_bodies: Query<(
        &mut RigidBodyForces,
        &mut RigidBodyVelocity,
        &RigidBodyMassProps,
    )>,
) {
    if keys.just_pressed(KeyCode::Space) {
        for (_, mut rb_vel, rb_mprops) in rigid_bodies.iter_mut() {
            rb_vel.apply_impulse(rb_mprops, Vec3::new(0.0, 2.0, 0.0).into());
        }
    }
}
