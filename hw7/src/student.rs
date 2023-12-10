use std::process::Command;
use bevy::input::keyboard::KeyboardInput;
use bevy::prelude::*;
use crate::asset_loader::SceneAssets;
use crate::asteroids::Asteroid;
use crate::collision_detection::Collider;
use crate::{db};

use crate::movement::*;
use crate::plant::Plant;
use crate::score::Score;

const STARTING_TRANSLATION: Vec3 = Vec3::new(0.0, 0.0, -20.0);
const SPACESHIP_SPEED: f32 = 25.0;
const SPACESHIP_ROTATIONAL_SPEED: f32 = 2.5;
const SPACESHIP_ROLL_SPEED: f32 = 2.5;
const MISSILE_SPEED: f32 = 50.0;
const MISSILE_FORWARD_SPAWN_SCALAR: f32 = 1.5;
const SPACESHIP_RADIUS: f32 = 2.5;
const MISSILE_RADIUS: f32 = 1.0;
#[derive(Component, Debug)]
pub struct Astronaut;

#[derive(Component, Debug)]
pub struct Bullet;


pub struct StudentBundle {
    velocity: Velocity,
    model: SceneBundle,
}

pub struct StudentPlugin;

impl Plugin for StudentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_spaceship)
            .add_systems(Update, (astornaut_movement_controls, spaceship_weapon_controls, handle_astronaut_collisions));
    }
}

fn spawn_spaceship(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((MovingObjectBundle {
        velocity: Velocity::new(Vec3::ZERO),
        acceleration: Acceleration::new(Vec3::ZERO),
        collider: Collider::new(SPACESHIP_RADIUS),
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
    let (mut transform, mut velocity) = query.single_mut();
    let mut rotation = 0.0;
    let mut roll = 0.0;
    let mut movement = 0.0;

    if keyboard_input.pressed(KeyCode::D) {
        rotation = -SPACESHIP_ROTATIONAL_SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::A) {
        rotation = SPACESHIP_ROTATIONAL_SPEED * time.delta_seconds();
    }

    if keyboard_input.pressed(KeyCode::S) {
        movement = -SPACESHIP_SPEED;
    }
    if keyboard_input.pressed(KeyCode::W) {
        movement = SPACESHIP_SPEED;
    }

    if keyboard_input.pressed(KeyCode::ShiftLeft) {
        roll = -SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::ControlLeft) {
        roll = SPACESHIP_ROLL_SPEED * time.delta_seconds();
    }

    transform.rotate_y(rotation);
    transform.rotate_local_z(roll);

    velocity.value = -transform.forward() * movement;

}

fn spaceship_weapon_controls(
    mut commands: Commands,
    query: Query<&Transform, With<Astronaut>>,
    keyboard_input: Res<Input<KeyCode>>,
    scene_assets: Res<SceneAssets>,
) {
    let transform = query.single();
    if keyboard_input.pressed(KeyCode::Space) {
        commands.spawn((
            MovingObjectBundle {
                velocity: Velocity::new(-transform.forward() * MISSILE_SPEED),
                acceleration: Acceleration::new(Vec3::ZERO),
                collider: Collider::new(MISSILE_RADIUS),
                model: SceneBundle {
                    scene: scene_assets.bullet.clone(),
                    transform: Transform::from_translation(
                        transform.translation + -transform.forward() * MISSILE_FORWARD_SPAWN_SCALAR,
                    ),
                    ..default()
                },
            },
            Bullet,
        ));
    }
}

fn handle_astronaut_collisions(
    mut commands: Commands,
    astronaut_query: Query<(Entity, &Collider), With<Astronaut>>,
    plant_query: Query<&Plant>,
    asteroid_query: Query<&Asteroid>,
    mut score: ResMut<Score>,
) {
    for (astronaut_entity, astronaut_collider) in astronaut_query.iter() {
        for &collided_entity in astronaut_collider.colliding_entities.iter() {
            // Check if the collided entity is a plant.
            if let Ok(_) = plant_query.get(collided_entity) {
                println!("Astronaut collided with plant! Plant collision");
                score.value += 1;
                println!("Score: {}", score.value);

            } else if let Ok(_) = asteroid_query.get(collided_entity) {
                println!("Astronaut collided with asteroid! Asteroid collision");
                score.lives -= 1;
                println!("Lives: {}", score.lives);
                if score.lives <= 0 {
                    println!("Game Over, you got {} points", score.value);

                    Command::new("sh")
                        .arg("-c")
                        .arg("echo 'Game Over' | festival --tts")
                        .spawn()
                        .expect("failed to execute process");
                    std::process::exit(0);

                }

            }
        }
    }
}



















