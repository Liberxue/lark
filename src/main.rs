use bevy::{
    app::App,
    color::Color,
    prelude::{default, ClearColor, Msaa, PluginGroup, Window},
    window::{CompositeAlphaMode, WindowMode, WindowPlugin, WindowTheme},
    winit::WinitSettings,
    DefaultPlugins,
};

use lark::UiPlugin;
fn main() {
    let mut app = App::new();
    // lark plugin
    let lark_plugins = WindowPlugin {
        primary_window: Some(Window {
            mode: WindowMode::Windowed,
            decorations: false,
            resolution: (1024., 720.).into(),
            focused: true,
            transparent: true,
            visible: true,
            #[cfg(target_os = "macos")]
            composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
            #[cfg(target_os = "linux")]
            composite_alpha_mode: CompositeAlphaMode::PreMultiplied,
            window_theme: Some(WindowTheme::Dark),
            ..default()
        }),
        ..default()
    };
    app.add_plugins(DefaultPlugins.set(lark_plugins));
    app.insert_resource(WinitSettings::desktop_app());
    app.add_plugins(bevy_svg::prelude::SvgPlugin); // set svg
    app.insert_resource(ClearColor(Color::NONE));
    app.insert_resource(Msaa::Sample4);
    app.add_plugins(UiPlugin);
    app.run();
}
