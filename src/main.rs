use bevy::{input::mouse::MouseMotion, prelude::*};
use core::time::Duration;
use ndarray::Array3;
use std::{f32::consts::PI, ops::Add};

mod solver;

#[derive(Resource)]
struct Cube(Array3<u8>);

#[derive(Component)]
struct ParentCube;

#[derive(Component)]
struct Index(u8);

fn main() {
    let solution = solver::solve();
    println!("{:?}", solution);

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Cube(solution.unwrap().1))
        .insert_resource(Highlight {
            timer: Timer::new(Duration::from_millis(100), TimerMode::Repeating),
            current: 0,
        })
        .add_startup_system(setup)
        .add_system(update_highlight)
        .add_system(update_colors)
        .add_system(mouse_input)
        .add_system(keyboard_input)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    cube: Res<Cube>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            VisibilityBundle::default(),
            TransformBundle::default(),
            ParentCube,
        ))
        .with_children(|builder| {
            // cubes
            for x in 0..4 {
                for y in 0..4 {
                    for z in 0..4 {
                        let index = *cube.0.get([x, y, z]).unwrap();
                        let hue = (index - 1) as f32 / 64.0 * 360.0;
                        let color = Color::hsla(hue, 1.0, 0.5, 0.2);
                        builder
                            .spawn(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
                                material: materials.add(color.into()),
                                transform: Transform::from_xyz(
                                    x as f32 * 0.5 - 0.75,
                                    y as f32 * 0.5 - 0.75,
                                    z as f32 * 0.5 - 0.75,
                                ),
                                ..default()
                            })
                            .insert(Index(index));
                    }
                }
            }
        });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(-1.0, 12.0, -10.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(1.0, 5.0, 4.0).looking_at(Vec3::ZERO, Vec3::Z),
        ..default()
    });
}

#[derive(Resource)]
struct Highlight {
    timer: Timer,
    current: u8,
}

fn update_highlight(time: Res<Time>, mut highlight: ResMut<Highlight>) {
    // tick the timer
    highlight.timer.tick(time.delta());

    if highlight.timer.finished() {
        highlight.current = (highlight.current + 1) % 65;
    }
}

fn update_colors(
    highlight: Res<Highlight>,
    query: Query<(&Handle<StandardMaterial>, &Index)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    query.for_each(|(handle, index)| {
        let material = materials.get_mut(handle).unwrap();

        if index.0 <= highlight.current {
            material.base_color.set_a(0.95);
        } else {
            material.base_color.set_a(0.05);
        }
    })
}

fn mouse_input(
    buttons: Res<Input<MouseButton>>,
    mut motion_evr: EventReader<MouseMotion>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<ParentCube>)>,
    mut cube: Query<&mut Transform, (With<ParentCube>, Without<Camera>)>,
) {
    if buttons.pressed(MouseButton::Left) {
        for ev in motion_evr.iter() {
            for ref mut transform in cube.iter_mut() {
                transform.rotate_around(Vec3::ZERO, Quat::from_rotation_z(ev.delta.x * PI / 120.0));
            }

            for ref mut transform in camera.iter_mut() {
                transform.translation.z += ev.delta.y / 15.0;
                transform.look_at(Vec3::ZERO, Vec3::Z)
            }
        }
    }
}

fn keyboard_input(keys: Res<Input<KeyCode>>, mut highlight: ResMut<Highlight>) {
    if keys.any_just_pressed([KeyCode::Right, KeyCode::Left, KeyCode::Up, KeyCode::Down]) {
        highlight.timer.pause();
    }

    if keys.any_just_pressed([KeyCode::Right, KeyCode::Up]) {
        highlight.current = highlight.current.add(1).min(65);
    }
    if keys.any_just_released([KeyCode::Left, KeyCode::Down]) {
        highlight.current = highlight.current.saturating_sub(1);
    }
}
