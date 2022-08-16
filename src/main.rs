use bevy::{
    input::{keyboard::KeyCode, Input},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(keyboard_input_system)
        .run();

        /* // For when we want to add more systems 
                .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(check_for_collisions)
                .with_system(move_paddle.before(check_for_collisions))
                .with_system(apply_velocity.before(check_for_collisions))
                .with_system(play_collision_sound.after(check_for_collisions)),
        )
        */
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("boss_bee.png"),
        ..default()
    });
}

/// This system prints 'A' key state
fn keyboard_input_system(keyboard_input: Res<Input<KeyCode>>) {

    if keyboard_input.just_released(KeyCode::W) {
        info!("'A' just released");//up
    }
    if keyboard_input.just_released(KeyCode::S) {
        info!("'A' just released");//down
    }
    if keyboard_input.just_released(KeyCode::A) {
        info!("'A' just released");//left
    }
    if keyboard_input.just_released(KeyCode::D) {
        info!("'A' just released");//righ
    }
}

