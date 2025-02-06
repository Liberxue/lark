use bevy::prelude::NonSend;
use bevy::prelude::Res;
use bevy::prelude::{Commands, Entity, Query, Resource};
use bevy::window::Window;
use bevy_egui::egui;
use bevy_egui::EguiContexts;

pub fn windows_button(
    ui: &mut egui::Ui,
    window_entity: Entity,
    winit_windows: &bevy::winit::WinitWindows,
) -> egui::InnerResponse<()> {
    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        ui.spacing_mut().item_spacing.x = 4.0;
        ui.add_space(5.0);

        let draw_circle_button = |ui: &mut egui::Ui,
                                  color: egui::Color32,
                                  text: &str,
                                  mut on_click: Box<dyn FnMut()>| {
            let circle_size = 13.0;
            let (rect, response) =
                ui.allocate_exact_size(egui::vec2(circle_size, circle_size), egui::Sense::click());
            if response.clicked() {
                on_click();
            }
            let center = rect.center();
            ui.painter().circle_filled(center, circle_size / 2.0, color);
            if response.hovered() {
                ui.painter().text(
                    center,
                    egui::Align2::CENTER_CENTER,
                    text,
                    egui::FontId::proportional(15.0),
                    egui::Color32::WHITE,
                );
            }
        };

        // Close button
        draw_circle_button(
            ui,
            egui::Color32::from_rgb(255, 92, 92),
            "x",
            Box::new(|| {
                std::process::exit(0);
            }),
        );

        let window_id = window_entity;
        draw_circle_button(
            ui,
            egui::Color32::from_rgb(255, 189, 46),
            "-",
            Box::new(move || {
                if let Some(window) = winit_windows.get_window(window_id) {
                    window.set_minimized(true);
                }
            }),
        );

        // Maximize button
        let window_id = window_entity;
        draw_circle_button(
            ui,
            egui::Color32::from_rgb(39, 201, 63),
            "+",
            Box::new(move || {
                if let Some(window) = winit_windows.get_window(window_id) {
                    let is_maximized = window.is_maximized();
                    window.set_maximized(!is_maximized);
                }
            }),
        );
    })
}

#[derive(Resource)]
pub struct WindowControlsState {
    pub show_controls: bool,
}

fn setup_window_controls(mut commands: Commands) {
    commands.insert_resource(WindowControlsState {
        show_controls: true,
    });
}

pub fn window_controls_system(
    mut egui_contexts: EguiContexts,
    window_query: Query<(Entity, &Window)>,
    winit_windows: NonSend<bevy::winit::WinitWindows>,
    window_controls_state: Res<WindowControlsState>,
) {
    if let Ok((window_entity, _window)) = window_query.get_single() {
        if window_controls_state.show_controls {
            egui::Window::new("controls")
                .frame(egui::Frame::none())
                .title_bar(false)
                .fixed_pos((0.0, 0.0))
                .show(egui_contexts.ctx_mut(), |ui| {
                    windows_button(ui, window_entity, &winit_windows);
                });
        }
    }
}
