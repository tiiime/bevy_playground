use bevy::{
    prelude::{default, App, Camera2dBundle, Color, Commands, Component, Transform, Vec3},
    sprite::{Sprite, SpriteBundle},
    DefaultPlugins,
};

#[derive(Component)]
struct SnakeHead;

const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_camera)
        .add_startup_system(spawn_snake)
        .run()
}

fn setup_camera(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn spawn_snake(mut commands: Commands) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: SNAKE_HEAD_COLOR,
                ..default()
            },
            transform: Transform {
                scale: Vec3::new(10.0, 10.0, 10.0),
                ..default()
            },
            ..default()
        })
        .insert(SnakeHead);
}
