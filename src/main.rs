// Jumpy Vim : Quick Game Dev in the context of Game Off 2021
// https://itch.io/jam/game-off-2021
// Author : Alexis LOUIS <me@alelouis.eu>

use bevy::prelude::*;
use bevy::input::keyboard::KeyboardInput;

struct Player;
struct Marker;
struct Position {
    x: i32,
    y: i32
}
struct WorldGrid {
    data: Vec<Vec<u8>>
}

fn main() {
    // Main app

    App::build()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(WindowDescriptor { 
            title: "Jumpy Vim".to_string(), 
            width: 300.0, 
            height: 300.0, 
            ..Default::default()})
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_world_grid.system())
        .add_startup_system(setup_player.system())
        .add_startup_system(setup_2d_camera.system())
        .add_system(process_kb.system())
        .run();
}

fn setup_2d_camera(
    mut commands: Commands
){
    // Setups ortho 2D camera

    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 1000.0));
    commands.spawn_bundle(camera);
}

fn setup_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    // Setups player entity and sprite

    let red_sprite = materials.add(asset_server.load("red.png").into());
    commands.spawn_bundle(SpriteBundle {
        material: red_sprite,
        transform: Transform {
            translation: Vec3::new(
                0. - 150. + 10., 
                0. - 150. + 10., 1.),
            rotation: Quat::from_rotation_z(0.),
            scale: Vec3::splat(1.),
        },
        sprite: Sprite::new(Vec2::splat(20.)),
        ..Default::default()
    }).insert(Player).insert(Position{x: 0, y: 0});
}

fn setup_world_grid(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    // Setups world grid and draw initial sprites

    let data = vec![
        vec![0, 0, 2, 1, 1, 1, 1, 3, 0, 2, 1, 1, 3, 0, 0], 
        vec![2, 1, 1, 1, 3, 0, 0, 0, 0, 2, 1, 1, 3, 0, 0],
        vec![0, 0, 2, 1, 1, 1, 1, 3, 0, 2, 1, 1, 3, 0, 0],
        vec![2, 1, 1, 1, 3, 0, 0, 0, 0, 2, 1, 1, 3, 0, 0],
        vec![0, 0, 2, 1, 1, 1, 1, 3, 0, 2, 1, 1, 3, 0, 0],
        vec![2, 1, 1, 1, 3, 0, 0, 0, 0, 2, 1, 1, 3, 0, 0],
        vec![0, 0, 2, 1, 1, 1, 1, 3, 0, 2, 1, 1, 3, 0, 0],
        vec![2, 1, 1, 1, 3, 0, 0, 0, 0, 2, 1, 1, 3, 0, 0],
        vec![0, 0, 2, 1, 1, 1, 1, 3, 0, 2, 1, 1, 3, 0, 0]];
    
    // 0 no brick
    // 1 brick
    // 2 start brick
    // 3 end brick

    commands.spawn().insert(WorldGrid{
        data: data.clone(),
    });

    let black_sprite = materials.add(asset_server.load("black.png").into());
    let white_sprite = materials.add(asset_server.load("white.png").into());
    let blue_sprite = materials.add(asset_server.load("blue.png").into());
    let green_sprite = materials.add(asset_server.load("green.png").into());

    for (i, row) in data.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            let material = match value {
                0 => black_sprite.clone(),
                1 => white_sprite.clone(),
                2 => blue_sprite.clone(),
                3 => green_sprite.clone(),
                _ => panic!("Invalid identifier encountered.")
            };
            commands.spawn_bundle(SpriteBundle {
                material: material,
                transform: Transform {
                    translation: Vec3::new(
                        (j as f32 * 20.) - 150. + 10., 
                        (i as f32 * 40.) - 150. + 10., 0.),
                    rotation: Quat::from_rotation_z(0.),
                    scale: Vec3::splat(1.),
                },
                sprite: Sprite::new(Vec2::splat(20.)),
                ..Default::default()
            });
        }
    }
}

fn process_kb(
    mut kb: EventReader<KeyboardInput>,
    mut query: Query<(&mut Transform, &mut Position, With<Player>)>,
){
    // Processes incoming keyboard messages as vim commands and moves player/marker

    use bevy::input::ElementState;
    for ev in kb.iter() {
        if ev.state == ElementState::Pressed {
            for (mut transform, mut position, _) in query.iter_mut() {
                let p = Position{x: position.x, y: position.y};
                let p_next = match ev.key_code {
                    Some(KeyCode::H) => vim_move_h(p),
                    Some(KeyCode::J) => vim_move_j(p),
                    Some(KeyCode::K) => vim_move_k(p),
                    Some(KeyCode::L) => vim_move_l(p),
                    Some(KeyCode::W) => vim_move_w(p),
                    Some(KeyCode::B) => vim_move_b(p),
                    Some(KeyCode::E) => vim_move_e(p),
                    _ => p,
                };
                position.x = p_next.x;
                position.y = p_next.y;

                let translation = &mut transform.translation;
                translation.x = p_next.x as f32 * 20. - 150. + 10.;
                translation.y = p_next.y as f32 * 20. - 150. + 10.;
            }
        }
    }
}

// left
fn vim_move_h(p: Position) -> Position{
    return Position{x: p.x - 1, y: p.y}
}

// top
fn vim_move_j(p: Position) -> Position{
    return Position{x: p.x, y: p.y - 1}
}

// bottom
fn vim_move_k(p: Position) -> Position{
    return Position{x: p.x, y: p.y + 1}
}

// right
fn vim_move_l(p: Position) -> Position{
    return Position{x: p.x + 1, y: p.y}
}

// begin of previous word
fn vim_move_w(p: Position) -> Position{
    return Position{x: p.x, y: p.y}
}

// begin of next word
fn vim_move_b(p: Position) -> Position{
    return Position{x: p.x, y: p.y}
}

// end of next word
fn vim_move_e(p: Position) -> Position{
    return Position{x: p.x, y: p.y}
}
