use bevy::prelude::*;
use core::time::Duration;
use ndarray::Array3;

mod solver;

#[derive(Resource)]
struct Cube(Array3<u8>);

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
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    cube: Res<Cube>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // cubes
    for x in 0..4 {
        for y in 0..4 {
            for z in 0..4 {
                let index = *cube.0.get([x, y, z]).unwrap();
                let hue = (index - 1) as f32 / 64.0 * 360.0;
                let color = Color::hsla(hue, 1.0, 0.5, 0.2);
                commands
                    .spawn(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
                        material: materials.add(color.into()),
                        transform: Transform::from_xyz(
                            x as f32 * 0.5 - 1.0,
                            y as f32 * 0.5 - 1.0,
                            z as f32 * 0.5 - 1.0,
                        ),
                        ..default()
                    })
                    .insert(Index(index));
            }
        }
    }

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
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

        if index.0 + 6 >= highlight.current && index.0 <= highlight.current {
            material
                .base_color
                .set_a(1.0 - 0.1 * (highlight.current as f32 - index.0 as f32));
        } else {
            material.base_color.set_a(0.2);
        }
    })
}
