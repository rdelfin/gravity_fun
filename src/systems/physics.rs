use crate::components::{AttractedBody, GravitationalBody};
use bevy::prelude::*;
use bevy_rapier3d::prelude::{RigidBodyForces, RigidBodyMassProps, RigidBodyVelocity};
use log::info;
use nalgebra::Vector3;

pub fn gravity_system(
    mut attracted_query: Query<(
        &mut RigidBodyForces,
        &RigidBodyVelocity,
        &RigidBodyMassProps,
        &Transform,
        &AttractedBody,
    )>,
    mut body_query: Query<(&Transform, &GravitationalBody)>,
) {
    let gravity_data: Vec<_> = body_query
        .iter()
        .map(|(transform, body)| (transform.translation, body.f))
        .collect();

    for (mut rb_forces, _, rb_mprops, transform, _) in attracted_query.iter_mut() {
        // Add up all the gravitational forces from all the surrounding bodies and apply them on
        // the object
        let grav_force = gravity_data
            .iter()
            .fold(Vector3::new(0.0, 0.0, 0.0), |sum, data| {
                let r: Vector3<f32> = (transform.translation - data.0).into();
                let r_len = r.norm();
                let r_unit = if r_len == 0. {
                    Vector3::new(0.0, 0.0, 0.0)
                } else {
                    r.normalize()
                };
                // Here, data.1 is a constant scalar = G*m_body
                (sum + r_unit * data.1 / r_len.powi(2))
            });
        info!("Gravity force: {:?}", gravity_data);
        rb_forces.force += grav_force;
    }
}
