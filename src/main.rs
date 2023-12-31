//! Shows how to render simple primitive shapes with a single color.

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, WorldInspectorPlugin::new()))
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
        .run();
}

#[derive(Component)]
enum Direction {
    Up,
    Down,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Circle
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
            ..default()
        },
        Direction::Up,
    ));

    // Rectangle
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.25, 0.25, 0.75),
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
            ..default()
        },
        Direction::Down,
    ));

    // Quad
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Quad::new(Vec2::new(50., 100.)).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
            transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
            ..default()
        },
        Direction::Up,
    ));

    // Hexagon
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
            material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
            transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
            ..default()
        },
        Direction::Down,
    ));
}

fn sprite_movement(
    time: Res<Time>,
    mut sprite_position: Query<(&mut Direction, &mut Transform)>,
    window: Query<&Window>,
) {
    for (mut direction, mut transform) in &mut sprite_position {
        const SPEED: f32 = 150.0;
        let y = &mut transform.translation.y;

        // move the sprite
        let dy = SPEED * time.delta_seconds();
        match *direction {
            Direction::Up => *y += dy,
            Direction::Down => *y -= dy,
        }

        // get window properties
        let window = window.single();
        // let width = window.resolution.width();
        let half_height = window.resolution.height() / 2.0;

        // bounce at the edges
        const HALF_HEIGHT_OF_SHAPE: f32 = 50.0; // TODO: this should not be static, but read from the shape
        if *y > (half_height - HALF_HEIGHT_OF_SHAPE) {
            *direction = Direction::Down;
        } else if *y < (-half_height + HALF_HEIGHT_OF_SHAPE) {
            *direction = Direction::Up;
        }
    }
}
