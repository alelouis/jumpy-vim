// Jumpy Vim
// Author : Alexis LOUIS <me@alelouis.eu>

use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use keyframe::{ease, functions::EaseInOut};
use rand::Rng;
use std::time::Duration;

mod moves;
struct Player;
struct Score(i32);
struct Marker;
struct FromPos(Position);
pub struct Position {
    x: i32,
    y: i32,
}
struct WorldGrid {
    data: Vec<Vec<u8>>,
}

fn main() {
    // Main app
    App::build()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(WorldGrid {
            data: vec![
                vec![0, 0, 2, 1, 1, 1, 1, 3, 0, 2, 1, 1, 3, 0, 0],
                vec![2, 1, 1, 1, 3, 0, 0, 0, 0, 2, 1, 1, 3, 0, 0],
                vec![0, 0, 2, 1, 1, 1, 1, 3, 0, 2, 1, 1, 3, 0, 0],
                vec![2, 1, 1, 1, 3, 0, 0, 0, 0, 2, 1, 1, 3, 0, 0],
                vec![0, 0, 2, 1, 1, 1, 1, 3, 0, 2, 1, 1, 3, 0, 0],
                vec![2, 1, 1, 1, 3, 0, 0, 0, 0, 2, 1, 1, 3, 0, 0],
                vec![0, 0, 2, 1, 1, 1, 1, 3, 0, 2, 1, 1, 3, 0, 0],
                vec![2, 1, 1, 1, 3, 0, 0, 0, 0, 2, 1, 1, 3, 0, 0],
                vec![0, 0, 2, 1, 1, 1, 1, 3, 0, 2, 1, 1, 3, 0, 0],
            ],
        })
        .insert_resource(WindowDescriptor {
            title: "Jumpy Vim".to_string(),
            width: 375.0,
            height: 550.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_world_grid.system())
        .add_startup_system(setup_player.system())
        .add_startup_system(setup_marker.system())
        .add_startup_system(setup_2d_camera.system())
        .add_startup_system(setup_score.system())
        .add_system(process_kb.system())
        .add_system(animate.system())
        .add_system(check_marker.system())
        .run();
}

fn grid_to_world(x: i32, y: i32) -> (f32, f32){
    let width = 375.;
    let height = 550.; 
    let sprite_size = 25.;
    let footer_size = 100.;
    let x_world = x as f32 * sprite_size - width / 2. + sprite_size/2.;
    let y_world = y as f32 * 2. * sprite_size - height/2. + sprite_size/2. + footer_size;

    (x_world, y_world)
}

fn animate(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &mut Transform,
        &Position,
        &FromPos,
        &mut Timer,
        With<Player>,
    )>,
) {
    for (entity, mut transform, position, frompos, mut timer, _) in query.iter_mut() {
        timer.tick(Duration::from_millis(30));
        let timer_duration = timer.duration().as_millis() as f32;
        let elapsed_time = timer.elapsed().as_millis() as f32;
        let t = elapsed_time / timer_duration;

        let (old_x, old_y) = grid_to_world(frompos.0.x, frompos.0.y);
        let (new_x, new_y) = grid_to_world(position.x, position.y);

        let translation = &mut transform.translation;
        translation.x = ease(EaseInOut, old_x, new_x, t);
        translation.y = ease(EaseInOut, old_y, new_y, t);

        if t == 1.0 {
            commands.entity(entity).remove::<Timer>();
            commands.entity(entity).remove::<FromPos>();
        }
    }
}

fn setup_2d_camera(mut commands: Commands) {
    // Setups ortho 2D camera

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 1000.0));
    commands.spawn_bundle(camera);
}

fn setup_score(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                bottom: Val::Px(15.0),
                left: Val::Px(15.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text::with_section(
            "Score = 0",
            TextStyle {
                font: asset_server.load("fonts/CozetteVector.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                ..Default::default()
            },
        ),
        ..Default::default()
    }).insert(Score(0));
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Setups player entity and sprite
    let initial_grid_position = Position { x: 2, y: 1 };
    let initial_world_position = grid_to_world(initial_grid_position.x, initial_grid_position.y);
    let red_sprite = materials.add(asset_server.load("red.png").into());
    commands
        .spawn_bundle(SpriteBundle {
            material: red_sprite,
            transform: Transform {
                translation: Vec3::new(
                    initial_world_position.0,
                    initial_world_position.1,
                    1.,
                ),
                rotation: Quat::from_rotation_z(0.),
                scale: Vec3::splat(1.),
            },
            sprite: Sprite::new(Vec2::splat(25.)),
            ..Default::default()
        })
        .insert(Player)
        .insert(initial_grid_position);
}

fn setup_marker(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Setups player entity and sprite
    let initial_grid_position = Position { x: 1, y: 1 };
    let initial_world_position = grid_to_world(initial_grid_position.x, initial_grid_position.y);
    let yellow_sprite = materials.add(asset_server.load("yellow.png").into());
    commands
        .spawn_bundle(SpriteBundle {
            material: yellow_sprite,
            transform: Transform {
                translation: Vec3::new(
                    initial_world_position.0,
                    initial_world_position.1,
                    1.,
                ),
                rotation: Quat::from_rotation_z(0.),
                scale: Vec3::splat(1.),
            },
            sprite: Sprite::new(Vec2::splat(25.)),
            ..Default::default()
        })
        .insert(Marker)
        .insert(initial_grid_position);
}

fn setup_world_grid(
    mut commands: Commands,
    world: Res<WorldGrid>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Setups world grid and draw initial sprites)

    // 0 no brick
    // 1 brick
    // 2 start brick
    // 3 end brick

    let data = &world.data;
    let black_sprite = materials.add(asset_server.load("black.png").into());
    let white_sprite = materials.add(asset_server.load("white.png").into());
    let blue_sprite = materials.add(asset_server.load("blue.png").into());
    let green_sprite = materials.add(asset_server.load("green.png").into());

    for (i, row) in data.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            let material = match value {
                0 => black_sprite.clone(),
                1 => white_sprite.clone(),
                2 => white_sprite.clone(),
                3 => white_sprite.clone(),
                _ => panic!("Invalid identifier encountered."),
            };
            let world_position = grid_to_world(j as i32, i as i32);
            commands.spawn_bundle(SpriteBundle {
                material: material,
                transform: Transform {
                    translation: Vec3::new(
                        world_position.0,
                        world_position.1,
                        0.,
                    ),
                    rotation: Quat::from_rotation_z(0.),
                    scale: Vec3::splat(1.),
                },
                sprite: Sprite::new(Vec2::splat(25.)),
                ..Default::default()
            });
        }
    }
}

fn check_marker(
    mut q: QuerySet<(
        Query<(&Position, With<Marker>)>,
        Query<(&Position, With<Player>)>,
        Query<(&mut Position, &mut Transform, With<Marker>)>,
        Query<(&mut Text, &mut Score, With<Score>)>,
    )>,
    world: Res<WorldGrid>,
) {
    // Getting marker position
    let mut m_pos = Position { x: 0, y: 0 };
    for (marker_position, _) in q.q0_mut().iter_mut() {
        m_pos.x = marker_position.x;
        m_pos.y = marker_position.y;
    }
    // Getting player position
    let mut p_pos = Position { x: 0, y: 0 };
    for (player_position, _) in q.q1_mut().iter_mut() {
        p_pos.x = player_position.x;
        p_pos.y = player_position.y;
    }
    // Checking equality
    if (p_pos.x == m_pos.x) && (p_pos.y == m_pos.y) {

        // Sampling new marker position
        let new_position = sample_position(&m_pos, &world.data);
        for (mut position, mut transform, _) in q.q2_mut().iter_mut() {
            position.x = new_position.x;
            position.y = new_position.y;
            let world_position = grid_to_world(position.x, position.y);
            let translation = &mut transform.translation;
            // Update marker position
            translation.x = world_position.0;
            translation.y = world_position.1;
        }
        // Update score
        for (mut text, mut score, _) in q.q3_mut().iter_mut() {
            score.0 += 1;
            text.sections[0].value = format!("Score = {}", score.0);
        }
    }
}

fn sample_position(p: &Position, w: &Vec<Vec<u8>>) -> Position {
    let mut x = rand::thread_rng().gen_range(0..w[0].len());
    let mut y = rand::thread_rng().gen_range(0..w.len());
    while (w[y][x] == 0) || ((x as i32 == p.x) && (y as i32 == p.y)) {
        x = rand::thread_rng().gen_range(0..w[0].len());
        y = rand::thread_rng().gen_range(0..w.len());
    }
    Position {
        x: x as i32,
        y: y as i32,
    }
}

fn process_kb(
    mut kb: EventReader<KeyboardInput>,
    mut commands: Commands,
    world: Res<WorldGrid>,
    mut query: Query<(Entity, &mut Position, Option<&mut Timer>, With<Player>)>,
) {
    // Processes incoming keyboard messages as vim commands and moves player/marker

    use bevy::input::ElementState;
    for ev in kb.iter() {
        if ev.state == ElementState::Pressed {
            for (entity, mut position, timer, _) in query.iter_mut() {
                let p = Position {
                    x: position.x,
                    y: position.y,
                };
                let p_next = match ev.key_code {
                    Some(KeyCode::H) => moves::vim_move_h(&p, &world.data),
                    Some(KeyCode::J) => moves::vim_move_j(&p, &world.data),
                    Some(KeyCode::K) => moves::vim_move_k(&p, &world.data),
                    Some(KeyCode::L) => moves::vim_move_l(&p, &world.data),
                    Some(KeyCode::W) => moves::vim_move_w(&p, &world.data),
                    Some(KeyCode::B) => moves::vim_move_b(&p, &world.data),
                    Some(KeyCode::E) => moves::vim_move_e(&p, &world.data),
                    _ => Position {
                        x: position.x,
                        y: position.y,
                    },
                };
                position.x = p_next.x;
                position.y = p_next.y;

                if timer.is_none() {
                    commands
                        .entity(entity)
                        .insert(Timer::from_seconds(1.0, false))
                        .insert(FromPos(p));
                } else {
                    timer.unwrap().reset();
                }
            }
        }
    }
}
