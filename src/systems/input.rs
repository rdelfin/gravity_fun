use crate::components::Controllable;
use bevy::prelude::*;
use bevy_rapier3d::prelude::{RigidBodyForces, RigidBodyMassProps, RigidBodyVelocity};

pub fn ball_move_system(
    keys: Res<Input<KeyCode>>,
    mut rigid_bodies: Query<(
        &mut RigidBodyForces,
        &mut RigidBodyVelocity,
        &RigidBodyMassProps,
        &Controllable,
    )>,
) {
    for (mut rb_forces, mut rb_vel, rb_mprops, _) in rigid_bodies.iter_mut() {
        if keys.pressed(KeyCode::W) {
            rb_forces.torque = Vec3::new(-0.3, 0., 0.).into();
        } else if keys.pressed(KeyCode::S) {
            rb_forces.torque = Vec3::new(0.3, 0., 0.).into();
        }
        if keys.pressed(KeyCode::A) {
            rb_forces.torque = Vec3::new(0., 0., 0.3).into();
        } else if keys.pressed(KeyCode::D) {
            rb_forces.torque = Vec3::new(0., 0., -0.3).into();
        }

        if keys.just_pressed(KeyCode::Space) {
            rb_vel.apply_impulse(rb_mprops, Vec3::new(0.0, 2.0, 0.0).into());
        }
    }
}
