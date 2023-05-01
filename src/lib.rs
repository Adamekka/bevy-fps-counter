//! FPS counter for Bevy game engine

use crate::config::{STRING_FORMAT, UPDATE_INTERVAL};
use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use config::{STRING_INITIAL, STRING_MISSING};
use std::fmt::Write;

mod config {
    use bevy::prelude::*;
    use std::time::Duration;

    pub const FONT_SIZE: f32 = 32.;
    pub const FONT_COLOR: Color = Color::WHITE;
    pub const UPDATE_INTERVAL: Duration = Duration::from_secs(1);

    pub const STRING_FORMAT: &str = "FPS: ";
    pub const STRING_INITIAL: &str = "FPS: ...";
    pub const STRING_MISSING: &str = "FPS: ???";
}

/// FPS counter plugin
pub struct FpsCounterPlugin;

impl Plugin for FpsCounterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(spawn_text)
            .add_system(update)
            .init_resource::<FpsCounter>();
    }
}

#[derive(Resource)]
pub struct FpsCounter {
    pub timer: Timer,
    pub update_now: bool,
}

impl Default for FpsCounter {
    fn default() -> Self {
        Self {
            timer: Timer::new(UPDATE_INTERVAL, TimerMode::Repeating),
            update_now: true,
        }
    }
}

impl FpsCounter {
    /// Enable FPS counter
    pub fn enable(&mut self) {
        self.timer.unpause();
        self.update_now = true;
    }

    /// Disable FPS counter
    pub fn disable(&mut self) {
        self.timer.pause();
        self.update_now = true;
    }

    /// Check if FPS counter is enabled
    pub fn is_enabled(&self) -> bool {
        !self.timer.paused()
    }
}

/// The marker on the text to be updated
#[derive(Component)]
pub struct FpsCounterText;

fn update(
    time: Res<Time>,
    diagnostics: Res<Diagnostics>,
    state_resources: Option<ResMut<FpsCounter>>,
    mut text_query: Query<&mut Text, With<FpsCounterText>>,
) {
    if let Some(mut state) = state_resources {
        if state.update_now || state.timer.tick(time.delta()).just_finished() {
            if state.timer.paused() {
                for mut text in text_query.iter_mut() {
                    let value: &mut String = &mut text.sections[0].value;
                    value.clear();
                }
            } else {
                let fps_dialog: Option<f64> = extract_fps(&diagnostics);

                for mut text in text_query.iter_mut() {
                    let value: &mut String = &mut text.sections[0].value;
                    value.clear();

                    if let Some(fps) = fps_dialog {
                        write!(value, "{}{:.0}", STRING_FORMAT, fps).expect("Failed to write");
                    } else {
                        value.clear();
                        write!(value, "{}", STRING_MISSING).expect("Failed to write");
                    }
                }
            }
        }
    }
}

fn extract_fps(diagnostics: &Res<Diagnostics>) -> Option<f64> {
    diagnostics
        .get(bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average())
}

fn spawn_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/UbuntuMonoNerdFontCompleteMono.ttf");

    commands
        .spawn(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: STRING_INITIAL.to_string(),
                    style: TextStyle {
                        font,
                        font_size: config::FONT_SIZE,
                        color: config::FONT_COLOR,
                    },
                }],
                ..default()
            },
            ..default()
        })
        .insert(FpsCounterText);
}
