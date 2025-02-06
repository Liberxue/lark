use bevy::{prelude::*, winit::WinitSettings};
use bevy_egui::EguiPlugin;
use components::*;
use resources::{setup_ui, AppState, NotificationTheme, OccupiedScreenSpace, UiState};

mod components;
mod resources;
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WinitSettings::default())
            .add_plugins(EguiPlugin)
            .init_resource::<UiState>()
            .init_resource::<NotificationTheme>()
            .init_resource::<OccupiedScreenSpace>()
            .init_resource::<AppState>()
            .add_systems(Startup, setup_ui)
            .add_systems(
                Update,
                (
                    splash_start.run_if(resource_equals(AppState::SplashStart)),
                    splash_to_ui.run_if(resource_equals(AppState::UiSetup)),
                    animate_splash.run_if(resource_equals(AppState::SplashAnimate)),
                    (main_ui_system,).run_if(resource_equals(AppState::Running)),
                ),
            );
    }
}
