use crate::components::PanOrbitCamera;
use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_rapier3d::{
    physics::{
        ColliderBundle, NoUserData, RapierPhysicsPlugin, RigidBodyBundle, RigidBodyPositionSync,
    },
    prelude::{ColliderMaterial, ColliderShape},
};
use nalgebra::base::Vector3;

mod components;
mod systems;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup_graphics.system())
        .add_startup_system(setup_physics.system())
        .add_system(exit_on_esc_system.system())
        .add_system(systems::camera_movement_system.system())
        .add_system(systems::ball_move_system.system())
        .run();
}

fn setup_graphics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    let translation = Vec3::new(-2.0, 2.5, 5.0);
    let radius = translation.length();
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(PanOrbitCamera {
            radius,
            ..Default::default()
        });
}

fn setup_physics(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /* Create the ground. */
    let collider = ColliderBundle {
        shape: ColliderShape::halfspace(Vector3::y_axis()),
        ..Default::default()
    };
    commands.spawn_bundle(collider);

    /* Create the bouncing ball. */
    let rigid_body = RigidBodyBundle {
        position: Vec3::new(0., 10., 0.).into(),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::ball(0.5),
        material: ColliderMaterial {
            restitution: 0.7,
            ..Default::default()
        },
        ..Default::default()
    };
    let sphere = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 0.5,
            subdivisions: 5,
        })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        ..Default::default()
    };
    commands
        .spawn_bundle(rigid_body)
        .insert_bundle(sphere)
        .insert_bundle(collider)
        .insert(Transform::default())
        .insert(RigidBodyPositionSync::Discrete);
}
