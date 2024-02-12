use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup, spawn_balls))
        .add_systems(Update, print_balls)
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
            ..Default::default()
        },
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 15.0, 0.0)
            .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        ..Default::default()
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

    let wall_halfheight = 5.0;
    commands.spawn((
        RigidBody::Fixed,
        Collider::compound(vec![
           (Vec3::new(wall_2d.min.y, wall_halfheight, 0.0), default(), Collider::cuboid(wall_halfthick, wall_halfheight, (wall_2d.max.y - wall_2d.min.y) / 2.0)),
           (Vec3::new(wall_2d.max.y, wall_halfheight, 0.0), default(), Collider::cuboid(wall_halfthick, wall_halfheight, (wall_2d.max.y - wall_2d.min.y) / 2.0)),
           (Vec3::new(0.0, wall_halfheight, wall_2d.min.x), default(), Collider::cuboid((wall_2d.max.x - wall_2d.min.x) / 2.0, wall_halfheight, wall_halfthick)),
           (Vec3::new(0.0, wall_halfheight, wall_2d.max.x), default(), Collider::cuboid((wall_2d.max.x - wall_2d.min.x) / 2.0, wall_halfheight, wall_halfthick)),
        ]),
        Sleeping::disabled(),
        Ccd::enabled(),
        TransformBundle::default()
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
    let radius = 0.1;
    let distance = radius * 1.7;
    for x in -25..25 {
        for y in 0..2 {
            for z in -25..25 {
                commands.spawn((
                    RigidBody::Dynamic,
                    //Velocity::default(),
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
                            1.0 + y as f32 * distance,
                            z as f32 * distance,
                        ),
                        ..Default::default()
                    },
                ));
            }
        }
    }
}
