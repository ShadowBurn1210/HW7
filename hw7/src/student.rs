use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crate::asset_loader::SceneAssets;

use crate::movement::*;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATIONAL_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;


#[derive(Bundle)]

pub struct Astronaut;


pub struct StudentBundle {
    velocity: Velocity,
    model: SceneBundle,
}

pub struct StudentPlugin;

impl Plugin for StudentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship);
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((MovingObejectBundle {
        velocity: Velocity::new(Vec3::ZERO),
        acceleration: Acceleration::new(Vec3::ZERO),
        model: SceneBundle {
            scene: scene_assets.spaceship.clone(),
            transform: Transform::from_translation(STARTING_TRANSLATION),
            ..default()
        },
    }, Astronaut
    ));
}

fn astornaut_movement_controls(
    mut query: Query<(&mut Transform, &mut Velocity), With<Astronaut>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
){

}