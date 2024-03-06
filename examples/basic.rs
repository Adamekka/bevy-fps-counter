use bevy::prelude::*;

use bevy_fps_counter::{FpsCounter, FpsCounterPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FpsCounterPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, mouse_handler)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn mouse_handler(
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut diags_state: ResMut<FpsCounter>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        if diags_state.is_enabled() {
            diags_state.disable();
        } else {
            diags_state.enable();
        }
    }
}
