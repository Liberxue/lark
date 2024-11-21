use bevy::prelude::{Component, Resource};
use bevy_egui::egui::Vec2;

#[derive(Resource)]
pub struct UiState {
    pub nav_width: f32,
    pub selected_nav_index: usize,
    pub show_avatar_menu: bool,
    pub show_status_menu: bool,
    pub show_siderbar: bool,
    pub selected_siderbar_button: String,
    //input

    // chat content
}

impl Default for UiState {
    fn default() -> Self {
        Self {
            nav_width: 50.0,
            selected_nav_index: 1,
            show_avatar_menu: false,
            show_status_menu: false,
            show_siderbar: false,
            selected_siderbar_button: String::new(),
        }
    }
}
// 定义导航页面枚举
#[derive(Debug, PartialEq, Clone)]
pub enum NavPage {
    Message,
    VideoMeeting,
    Calendar,
    Doc,
    Contact,
    Table,
    Search,
}
impl UiState {
    pub fn current_page(&self) -> NavPage {
        match self.selected_nav_index {
            0 => NavPage::Search,
            1 => NavPage::Message,
            2 => NavPage::Calendar,
            3 => NavPage::Doc,
            4 => NavPage::VideoMeeting,
            5 => NavPage::Table,
            6 => NavPage::Contact,
            _ => NavPage::Message,
        }
    }
}

#[derive(Default, Resource)]
pub struct OccupiedScreenSpace {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

#[derive(Resource, Default, Debug, PartialEq, Eq, Clone)]
pub enum AppState {
    #[default]
    SplashStart,
    UiSetup,
    SplashAnimate,
    Running,
}
#[derive(Component)]
pub struct SplashCamera;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct SplashScreen {
    pub is_animating: bool,
}

#[derive(Component)]
pub struct SplashAnimation {
    pub start_pos: Vec2,
    pub end_pos: Vec2,
    pub progress: f32,
    pub duration: f32,
}

mod setup;

pub use setup::*;
