use bevy::{
    prelude::*,
    render::render_resource::{Extent3d, Texture, TextureDimension, TextureFormat},
};

use crate::{Render, H_SIZE, W_SIZE};

pub struct SimPlugin;

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(W_SIZE as f32, H_SIZE as f32)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
        ..default()
    });
}

fn render_loop(
    mut commands: Commands,
    mut render: Local<Render>,
    mut query: Query<&mut Sprite>,
) {
    let data = render.data;

    for mut sprite in query.iter_mut() {
        println!("sprite: {:?}", sprite);
    }
}

impl Plugin for SimPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, render_loop);
    }
}
