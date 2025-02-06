use std::time::Duration;

use bevy::{
    app::App,
    color::Color,
    prelude::{default, ClearColor, Msaa, PluginGroup, Window},
    window::{CompositeAlphaMode, WindowMode, WindowPlugin},
    winit::{UpdateMode, WinitSettings},
    DefaultPlugins,
};

use lark::UiPlugin;
fn main() {
    let mut app = App::new();
    let window_plugin = WindowPlugin {
        primary_window: Some(Window {
            mode: WindowMode::Windowed,
            decorations: false,
            resolution: (1024., 820.).into(),
            focused: true,
            transparent: true,
            visible: true,
            #[cfg(target_os = "macos")]
            composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
            #[cfg(target_os = "linux")]
            composite_alpha_mode: CompositeAlphaMode::PreMultiplied,
            // window_theme: Some(WindowTheme::Dark),
            ..default()
        }),
        ..default()
    };

    app.add_plugins(DefaultPlugins.set(window_plugin));

    app.insert_resource(WinitSettings::desktop_app());
    app.insert_resource(WinitSettings {
        focused_mode: UpdateMode::reactive(Duration::from_secs(500)),
        unfocused_mode: UpdateMode::reactive_low_power(Duration::from_secs(600)),
    });

    app.insert_resource(ClearColor(Color::NONE));
    app.add_plugins(bevy_svg::prelude::SvgPlugin);
    app.insert_resource(Msaa::Sample4);
    app.add_plugins(UiPlugin);
    app.run();
}
