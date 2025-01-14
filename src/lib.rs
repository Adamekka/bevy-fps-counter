//! FPS counter for Bevy game engine

use crate::config::{STRING_FORMAT, UPDATE_INTERVAL};
use bevy::{
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use config::{STRING_INITIAL, STRING_MISSING};

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
        app.add_plugins(FrameTimeDiagnosticsPlugin)
            .add_systems(Startup, spawn_text)
            .add_systems(Update, update)
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
    diagnostics: Res<DiagnosticsStore>,
    state_resources: Option<ResMut<FpsCounter>>,
    mut query: Query<Entity, With<FpsCounterText>>,
    mut writer: TextUiWriter,
) {
    let Some(mut state) = state_resources else {
        return;
    };
    if !(state.update_now || state.timer.tick(time.delta()).just_finished()) {
        return;
    }
    if state.timer.paused() {
        for entity in query.iter_mut() {
            writer.text(entity, 0).clear();
        }
    } else {
        let fps_dialog: Option<f64> = extract_fps(&diagnostics);

        for entity in query.iter_mut() {
            if let Some(fps) = fps_dialog {
                *writer.text(entity, 0) = format!("{}{:.0}", STRING_FORMAT, fps);
            } else {
                *writer.text(entity, 0) = STRING_MISSING.to_string();
            }
        }
    }
}

fn extract_fps(diagnostics: &Res<DiagnosticsStore>) -> Option<f64> {
    diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average())
}

fn spawn_text(mut commands: Commands) {
    commands
        .spawn((
            Text::new(STRING_INITIAL),
            TextFont {
                font_size: config::FONT_SIZE,
                ..Default::default()
            },
            TextColor(config::FONT_COLOR),
        ))
        .insert(FpsCounterText);
}
