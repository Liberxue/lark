use bevy_egui::egui::{self, Ui};

pub fn windows_button(ui: &mut Ui) -> egui::InnerResponse<()> {
    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
        ui.spacing_mut().item_spacing.x = 4.0; // 设置按钮之间的间距
        ui.add_space(5.0);
        let draw_circle_button = |ui: &mut egui::Ui,
                                  color: egui::Color32,
                                  text: &str,
                                  on_click: fn()| {
            let circle_size = 13.0;
            let (rect, response) =
                ui.allocate_exact_size(egui::vec2(circle_size, circle_size), egui::Sense::click());
            if response.clicked() {
                on_click(); // Trigger the provided click function
            }
            let center = rect.center();

            ui.painter().circle_filled(center, circle_size / 2.0, color);

            if response.hovered() {
                ui.painter().text(
                    center,
                    egui::Align2::CENTER_CENTER,
                    text,
                    egui::FontId::proportional(15.0), // Bold font style
                    egui::Color32::WHITE,
                );
            }
        };

        draw_circle_button(ui, egui::Color32::from_rgb(255, 92, 92), "x", || {
            std::process::exit(0); // Close action
        });

        draw_circle_button(ui, egui::Color32::from_rgb(255, 189, 46), "-", || {
            println!("Minimize button clicked");
        });

        draw_circle_button(ui, egui::Color32::from_rgb(39, 201, 63), "+", || {
            println!("Maximize button clicked");
        });
    })
}
