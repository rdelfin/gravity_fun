use bevy::{input::system::exit_on_esc_system, prelude::*};
use bevy_rapier3d::{
    physics::{
        ColliderBundle, NoUserData, RapierConfiguration, RapierPhysicsPlugin, RigidBodyBundle,
        RigidBodyPositionSync,
    },
    prelude::{ColliderMaterial, ColliderShape},
};
use nalgebra::base::Vector3;
use simple_logger::SimpleLogger;

mod components;
mod systems;

fn main() {
    SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();

    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(RapierConfiguration {
            gravity: Vector3::new(0., 0., 0.),
            ..Default::default()
        })
        .add_startup_system(setup_graphics.system())
        .add_startup_system(setup_physics.system())
        .add_system(exit_on_esc_system.system())
        .add_system(systems::camera_movement_system.system())
        .add_system(systems::ball_move_system.system())
        .add_system(systems::gravity_system.system())
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // light
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });
    // camera
    let translation = Vec3::new(-20.0, 20.0, 0.0);
    let radius = translation.length();
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .insert(components::PanOrbitCamera {
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
    let planet = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Icosphere {
            radius: 5.,
            subdivisions: 5,
        })),
        material: materials.add(Color::rgb(0.0, 1.0, 0.0).into()),
        ..Default::default()
    };
    let collider = ColliderBundle {
        shape: ColliderShape::ball(5.),
        ..Default::default()
    };
    commands
        .spawn_bundle(planet)
        .insert_bundle(collider)
        .insert(components::GravitationalBody { f: 20. })
        .insert(Transform::default());

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
        .insert(components::Controllable)
        .insert(components::AttractedBody::default())
        .insert(Transform::default())
        .insert(RigidBodyPositionSync::Discrete);
}
