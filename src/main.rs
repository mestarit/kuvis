use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup, spawn_balls))
        .add_systems(Update, (ui, despawn_dropped_rbs))
        .run()
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Light + camera
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 20000.0,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 15.0, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..default()
    });

    let wall_2d = Rect::new(-5.0, -5.0, 5.0, 5.0);
    let wall_halfthick = 0.05;
    // Spawn the ground
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(5.0, 0.05, 5.0),
        Sleeping::disabled(),
        Ccd::enabled(),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                min_x: wall_2d.min.x,
                max_x: wall_2d.max.x,
                min_y: -wall_halfthick,
                max_y: wall_halfthick,
                min_z: wall_2d.min.y,
                max_z: wall_2d.max.y,
            })),
            material: materials.add(Color::rgb(0.5, 0.9, 0.3).into()),
            ..Default::default()
        },
    ));

    let wall_halfheight = 10.0;
    commands.spawn((
        RigidBody::Fixed,
        Collider::compound(vec![
            (
                Vec3::new(wall_2d.min.y, wall_halfheight, 0.0),
                default(),
                Collider::cuboid(
                    wall_halfthick,
                    wall_halfheight,
                    (wall_2d.max.y - wall_2d.min.y) / 2.0,
                ),
            ),
            (
                Vec3::new(wall_2d.max.y, wall_halfheight, 0.0),
                default(),
                Collider::cuboid(
                    wall_halfthick,
                    wall_halfheight,
                    (wall_2d.max.y - wall_2d.min.y) / 2.0,
                ),
            ),
            (
                Vec3::new(0.0, wall_halfheight, wall_2d.min.x),
                default(),
                Collider::cuboid(
                    (wall_2d.max.x - wall_2d.min.x) / 2.0,
                    wall_halfheight,
                    wall_halfthick,
                ),
            ),
            (
                Vec3::new(0.0, wall_halfheight, wall_2d.max.x),
                default(),
                Collider::cuboid(
                    (wall_2d.max.x - wall_2d.min.x) / 2.0,
                    wall_halfheight,
                    wall_halfthick,
                ),
            ),
        ]),
        Sleeping::disabled(),
        Ccd::enabled(),
        TransformBundle::default(),
    ));

    // Add control window
    commands.spawn(Window {
        title: "Controls".to_owned(),
        ..default()
    });
}

fn ui(
    mut egui_ctx: Query<&mut EguiContext, Without<PrimaryWindow>>,
    rbs: Query<(), With<RigidBody>>,

    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(mut ctx) = egui_ctx.get_single_mut() else {
        return;
    };
    egui::CentralPanel::default().show(ctx.get_mut(), |ui| {
        ui.label(format!("Number of rigid bodies: {}", rbs.iter().count()));
        if ui.button("Spawn (more) balls").clicked() {
            spawn_balls(commands, meshes, materials);
        }
        ui.text_edit_multiline(&mut "adsopdfg");
    });
}

fn despawn_dropped_rbs(
    mut commands: Commands,
    rbs: Query<(Entity, &Transform), With<RigidBody>>,
) {
    for (ent, transform) in rbs.iter() {
        if transform.translation.y < -5.0 {
            commands.entity(ent).despawn();
        }
    }
}

fn spawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 0.1;
    let distance = radius * 2.5;
    for y in 0..=2 {
        let distance = distance + (y as f32) / 50.0;
        for x in -20..=20 {
            for z in -20..=20 {
                commands.spawn((
                    RigidBody::Dynamic,
                    Velocity {
                        linvel: Vec3::ZERO,
                        angvel: Vec3::new((x as f32).sin(), 0., (z as f32).sin()),
                    },
                    Collider::ball(radius),
                    Friction {
                        coefficient: 0.7,
                        combine_rule: default(),
                    },
                    PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::UVSphere {
                            radius,
                            ..Default::default()
                        })),
                        material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
                        transform: Transform::from_xyz(
                            x as f32 * distance,
                            10.0 + y as f32 * distance,
                            z as f32 * distance,
                        ),
                        ..Default::default()
                    },
                ));
            }
        }
    }
}
