use bevy::prelude::*;

use bevy_fps_counter::{FpsCounter, FpsCounterPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(FpsCounterPlugin)
        .add_startup_system(setup)
        .add_system(mouse_handler)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn mouse_handler(mouse_button_input: Res<Input<MouseButton>>, mut diags_state: ResMut<FpsCounter>) {
    if mouse_button_input.pressed(MouseButton::Left) {
        if diags_state.is_enabled() {
            diags_state.disable();
        } else {
            diags_state.enable();
        }
    }
}
