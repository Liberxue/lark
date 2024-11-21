use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::{CompositeAlphaMode, WindowMode, WindowTheme},
    winit::WinitSettings,
};
use lark::UiPlugin;

fn main() {
    let mut app = App::new();
    // lark plugin
    let lark_plugins = WindowPlugin {
        primary_window: Some(Window {
            mode: WindowMode::Windowed,
            decorations: false,
            resolution: (1280., 720.).into(),
            skip_taskbar: true,
            transparent: true,
            #[cfg(target_os = "macos")]
            composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
            #[cfg(target_os = "linux")]
            composite_alpha_mode: CompositeAlphaMode::PreMultiplied,
            window_theme: Some(WindowTheme::Dark),
            position: WindowPosition::At(IVec2::new(1900, 100)), //TODO: set last position
            ..default()
        }),
        ..default()
    };
    app.add_plugins(DefaultPlugins.set(lark_plugins));
    app.insert_resource(WinitSettings::desktop_app());
    app.add_plugins(bevy_svg::prelude::SvgPlugin); // set svg
    app.insert_resource(ClearColor(Color::NONE));
    // 多重采样抗锯齿
    app.insert_resource(Msaa::Sample4);
    app.add_plugins(UiPlugin);
    app.add_plugins(LogDiagnosticsPlugin::default());
    app.add_plugins(FrameTimeDiagnosticsPlugin);
    app.run();
}
