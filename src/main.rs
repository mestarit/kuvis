use std::thread::spawn;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup, spawn_balls))
        .add_systems(Update, print_balls)
        .run()
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 20000.0,
            range: 100.0,
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 10.0, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
    });

    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(10.0, 0.1, 10.0),
        GravityScale(0.5),
        Sleeping::disabled(),
        Ccd::enabled(),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: -5.0,
                max_x: 5.0,
                min_y: -0.05,
                max_y: 0.05,
                min_z: -5.0,
                max_z: 5.0,
            })),
            material: materials.add(Color::rgb(0.5, 0.9, 0.3).into()),
            ..Default::default()
        },
    ));
}

fn print_balls(query: Query<&Transform, With<RigidBody>>) {
    for rb in query.iter() {
        println!("RigidBody: {:?}", rb);
    }
}

fn spawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for i in 0..10 {
        commands.spawn((
            RigidBody::Dynamic,
            Velocity {
                linvel: Vec3::ZERO,
                angvel: Vec3::ZERO,
            },
            Collider::ball(0.5),
            GravityScale(0.5),
            Sleeping::disabled(),
            Ccd::enabled(),
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::UVSphere {
                    radius: 0.5,
                    ..Default::default()
                })),
                material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
                transform: Transform::from_translation(Vec3::new(i as f32, 10.0, 0.0)),
                ..Default::default()
            },
        ));
    }
}
