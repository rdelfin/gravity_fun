use crate::components::PanOrbitCamera;
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
    render::camera::PerspectiveProjection,
};

pub fn camera_movement_system(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &PerspectiveProjection)>,
) {
    let orbit_button = MouseButton::Right;
    let pan_button = MouseButton::Middle;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;
    let window = get_primary_window_size(&windows);

    if input_mouse.pressed(pan_button) {
        rotation_move += ev_motion.iter().fold(Vec2::ZERO, |acc, ev| acc + ev.delta);
    } else if input_mouse.pressed(orbit_button) {
        pan += ev_motion.iter().fold(Vec2::ZERO, |acc, ev| acc + ev.delta);
    }
    scroll += ev_scroll.iter().fold(0.0, |acc, ev| acc + ev.y);
    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        if orbit_button_changed {
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation;
            transform.rotation = transform.rotation * pitch;
        } else if pan.length_squared() > 0.0 {
            any = true;
            pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if any {
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation =
                pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}
