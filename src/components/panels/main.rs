use super::{chat_main_ui, left_chat_list_ui, left_nav_ui, left_sidebar_ui};
use crate::resources::{NavPage, OccupiedScreenSpace, UiState};
use bevy::prelude::ResMut;
use bevy_egui::{egui, egui::CentralPanel, EguiContexts};

pub fn main_ui_system(
    mut contexts: EguiContexts,
    mut ui_state: ResMut<UiState>,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {
    let ctx = contexts.ctx_mut();

    let left = left_nav_ui(ctx, &mut ui_state).response.rect.width();
    occupied_screen_space.left = left;
    match ui_state.current_page() {
        NavPage::Message => {
            if ui_state.show_siderbar {
                let sidebar_width = left_sidebar_ui(ctx, &mut ui_state).response.rect.width();
                occupied_screen_space.left += sidebar_width;
            }
            let chat_list_width = left_chat_list_ui(ctx, &mut ui_state).response.rect.width();
            let chat_main_width = chat_main_ui(ctx, &mut ui_state).response.rect.width();
            occupied_screen_space.left += chat_list_width;
            occupied_screen_space.right = chat_main_width;
        }
        NavPage::Calendar => {
            show_calendar_ui(ctx);
        }
        NavPage::Doc => {
            show_doc_ui(ctx);
        }
        NavPage::Table => {
            show_table_ui(ctx);
        }
        NavPage::Search => {
            show_search_ui(ctx);
        }
        NavPage::Contact => {
            show_contact_ui(ctx);
        }
        NavPage::VideoMeeting => {
            show_video_meeting_ui(ctx);
        }
    }
}

fn show_calendar_ui(ctx: &egui::Context) {
    CentralPanel::default()
        .frame(egui::Frame {
            fill: egui::Color32::from_rgb(0, 0, 0),
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.heading("calendar");
        });
}
fn show_search_ui(ctx: &egui::Context) {
    CentralPanel::default()
        .frame(egui::Frame {
            fill: egui::Color32::from_rgb(0, 0, 0),
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.heading("search");
        });
}
fn show_doc_ui(ctx: &egui::Context) {
    CentralPanel::default()
        .frame(egui::Frame {
            fill: egui::Color32::from_rgb(0, 0, 0),
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.heading("doc");
        });
}

fn show_message_ui(ctx: &egui::Context) {
    CentralPanel::default()
        .frame(egui::Frame {
            fill: egui::Color32::from_rgb(0, 0, 0),
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.heading("message");
        });
}

fn show_contact_ui(ctx: &egui::Context) {
    CentralPanel::default()
        .frame(egui::Frame {
            fill: egui::Color32::from_rgb(0, 0, 0),
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.heading("contact");
        });
}

fn show_table_ui(ctx: &egui::Context) {
    CentralPanel::default()
        .frame(egui::Frame {
            fill: egui::Color32::from_rgb(0, 0, 0),
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.heading("table");
        });
}

fn show_video_ui(ctx: &egui::Context) {
    CentralPanel::default()
        .frame(egui::Frame {
            fill: egui::Color32::from_rgb(0, 0, 0),
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.heading("video");
        });
}
fn show_video_meeting_ui(ctx: &egui::Context) {
    CentralPanel::default()
        .frame(egui::Frame {
            fill: egui::Color32::from_rgb(0, 0, 0),
            ..Default::default()
        })
        .show(ctx, |ui| {
            ui.heading("video_meeting");
        });
}
