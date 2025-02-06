use bevy::prelude::Resource;
use bevy_egui::egui::{Color32, FontFamily, FontId, Margin, Rounding};

#[derive(Clone, Copy, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
}

#[derive(Resource)]
pub struct NotificationTheme {
    pub mode: ThemeMode,
    pub style: NotificationStyle,
    pub light_colors: NotificationColors,
    pub dark_colors: NotificationColors,
    pub fonts: NotificationFonts,
    pub text_styles: TextStyles,
}
#[derive(Clone, Copy)]
pub struct TextStyles {
    // 通用文本样式
    pub title: TextStyle,
    pub body: TextStyle,
    pub label: TextStyle,
    pub count: TextStyle,

    // 导航栏特定样式
    pub nav_icon: TextStyle,
    pub nav_label: TextStyle,
    pub nav_notification: TextStyle,

    // 侧边栏特定样式
    pub sidebar_title: TextStyle,
    pub sidebar_button: TextStyle,
    pub sidebar_item: TextStyle,

    // 聊天列表特定样式
    pub chat_title: TextStyle,
    pub chat_message: TextStyle,
    pub chat_time: TextStyle,
    pub chat_unread: TextStyle,
}

#[derive(Clone, Copy)]
pub struct TextStyle {
    pub color: Color32,
    pub selected_color: Color32,
    pub hover_color: Color32,
}

pub struct NotificationStyle {
    pub rounding: Rounding,
    pub margin: Margin,
    pub nav_rounding: Rounding, // 导航栏特定圆角
    pub sidebar_margin: Margin, // 侧边栏特定边距
}

pub struct NotificationColors {
    pub background: Color32,
    pub text: Color32,
    pub selected: Color32,
    pub selected_background: Color32,
    pub notification: Color32,
    pub hover: Color32,
    pub border: Color32,
    pub divider: Color32, // 分割线颜色
    pub accent: Color32,  // 强调色
}

pub struct NotificationFonts {
    pub icon: FontId,
    pub label: FontId,
    pub title: FontId,
    pub content: FontId,
    pub timestamp: FontId,
    pub nav_icon: FontId,  // 导航图标特定字体
    pub nav_label: FontId, // 导航标签特定字体
}

impl TextStyles {
    pub fn light() -> Self {
        Self {
            // 通用样式
            title: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::WHITE,
                hover_color: Color32::from_rgb(220, 220, 220),
            },
            body: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::WHITE,
                hover_color: Color32::from_rgb(220, 220, 220),
            },
            label: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::WHITE,
                hover_color: Color32::from_rgb(220, 220, 220),
            },
            count: TextStyle {
                color: Color32::from_rgb(255, 255, 255),
                selected_color: Color32::from_rgb(200, 200, 200),
                hover_color: Color32::from_rgb(180, 180, 180),
            },

            // 导航栏样式
            nav_icon: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::from_rgb(22, 119, 255),
                hover_color: Color32::from_rgb(220, 220, 220),
            },
            nav_label: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::from_rgb(22, 119, 255),
                hover_color: Color32::from_rgb(220, 220, 220),
            },
            nav_notification: TextStyle {
                color: Color32::DARK_RED,
                selected_color: Color32::RED,
                hover_color: Color32::RED,
            },

            // 侧边栏样式
            sidebar_title: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::WHITE,
                hover_color: Color32::from_rgb(220, 220, 220),
            },
            sidebar_button: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::WHITE,
                hover_color: Color32::from_rgb(180, 180, 180),
            },
            sidebar_item: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::WHITE,
                hover_color: Color32::from_rgb(220, 220, 220),
            },

            // 聊天列表样式
            chat_title: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::from_rgb(51, 51, 51),
                hover_color: Color32::from_rgb(220, 220, 220),
            },
            chat_message: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::from_rgb(220, 220, 220),
                hover_color: Color32::from_rgb(200, 200, 200),
            },
            chat_time: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::from_rgb(0, 0, 0),
                hover_color: Color32::from_rgb(170, 170, 170),
            },
            chat_unread: TextStyle {
                color: Color32::RED,
                selected_color: Color32::RED,
                hover_color: Color32::RED,
            },
        }
    }

    pub fn dark() -> Self {
        Self {
            title: TextStyle {
                color: Color32::from_rgb(230, 230, 230),
                selected_color: Color32::WHITE,
                hover_color: Color32::from_rgb(200, 200, 200),
            },
            body: TextStyle {
                color: Color32::from_rgb(230, 230, 230),
                selected_color: Color32::WHITE,
                hover_color: Color32::from_rgb(200, 200, 200),
            },
            label: TextStyle {
                color: Color32::from_rgb(230, 230, 230),
                selected_color: Color32::WHITE,
                hover_color: Color32::from_rgb(200, 200, 200),
            },
            count: TextStyle {
                color: Color32::from_rgb(255, 255, 255),
                selected_color: Color32::from_rgb(200, 200, 200),
                hover_color: Color32::from_rgb(180, 180, 180),
            },

            // 导航栏样式
            nav_icon: TextStyle {
                color: Color32::from_rgb(230, 230, 230),
                selected_color: Color32::from_rgb(22, 119, 255),
                hover_color: Color32::from_rgb(180, 180, 180),
            },
            nav_label: TextStyle {
                color: Color32::from_rgb(230, 230, 230),
                selected_color: Color32::from_rgb(22, 119, 255),
                hover_color: Color32::from_rgb(180, 180, 180),
            },
            nav_notification: TextStyle {
                color: Color32::LIGHT_RED,
                selected_color: Color32::RED,
                hover_color: Color32::RED,
            },

            // 侧边栏样式
            sidebar_title: TextStyle {
                color: Color32::from_rgb(230, 230, 230),
                selected_color: Color32::WHITE,
                hover_color: Color32::from_rgb(200, 200, 200),
            },
            sidebar_button: TextStyle {
                color: Color32::from_rgb(230, 230, 230),
                selected_color: Color32::WHITE,
                hover_color: Color32::from_rgb(180, 180, 180),
            },
            sidebar_item: TextStyle {
                color: Color32::from_rgb(230, 230, 230),
                selected_color: Color32::WHITE,
                hover_color: Color32::from_rgb(200, 200, 200),
            },

            // 聊天列表样式
            chat_title: TextStyle {
                color: Color32::from_rgb(230, 230, 230),
                selected_color: Color32::from_rgb(230, 230, 230),
                hover_color: Color32::from_rgb(200, 200, 200),
            },
            chat_message: TextStyle {
                color: Color32::from_rgb(230, 230, 230),
                selected_color: Color32::from_rgb(200, 200, 200),
                hover_color: Color32::from_rgb(180, 180, 180),
            },
            chat_time: TextStyle {
                color: Color32::from_rgb(230, 230, 230),
                selected_color: Color32::from_rgb(0, 0, 0),
                hover_color: Color32::from_rgb(170, 170, 170),
            },
            chat_unread: TextStyle {
                color: Color32::LIGHT_RED,
                selected_color: Color32::RED,
                hover_color: Color32::RED,
            },
        }
    }
}

impl Default for TextStyles {
    fn default() -> Self {
        Self {
            // 通用样式
            title: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::WHITE,
                hover_color: Color32::from_rgb(220, 220, 220),
            },
            body: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::WHITE,
                hover_color: Color32::from_rgb(220, 220, 220),
            },
            label: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::WHITE,
                hover_color: Color32::from_rgb(220, 220, 220),
            },
            count: TextStyle {
                color: Color32::from_rgb(255, 255, 255),
                selected_color: Color32::from_rgb(200, 200, 200),
                hover_color: Color32::from_rgb(180, 180, 180),
            },

            // 导航栏样式
            nav_icon: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::from_rgb(22, 119, 255),
                hover_color: Color32::from_rgb(220, 220, 220),
            },
            nav_label: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::from_rgb(22, 119, 255),
                hover_color: Color32::from_rgb(220, 220, 220),
            },
            nav_notification: TextStyle {
                color: Color32::DARK_RED,
                selected_color: Color32::RED,
                hover_color: Color32::RED,
            },

            // 侧边栏样式
            sidebar_title: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::WHITE,
                hover_color: Color32::from_rgb(220, 220, 220),
            },
            sidebar_button: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::WHITE,
                hover_color: Color32::from_rgb(180, 180, 180),
            },
            sidebar_item: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::WHITE,
                hover_color: Color32::from_rgb(220, 220, 220),
            },

            // 聊天列表样式
            chat_title: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::from_rgb(51, 51, 51),
                hover_color: Color32::from_rgb(220, 220, 220),
            },
            chat_message: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::from_rgb(220, 220, 220),
                hover_color: Color32::from_rgb(200, 200, 200),
            },
            chat_time: TextStyle {
                color: Color32::from_rgb(51, 51, 51),
                selected_color: Color32::from_rgb(0, 0, 0),
                hover_color: Color32::from_rgb(170, 170, 170),
            },
            chat_unread: TextStyle {
                color: Color32::RED,
                selected_color: Color32::RED,
                hover_color: Color32::RED,
            },
        }
    }
}

impl NotificationTheme {
    pub fn current_colors(&self) -> &NotificationColors {
        match self.mode {
            ThemeMode::Light => &self.light_colors,
            ThemeMode::Dark => &self.dark_colors,
        }
    }

    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::Light,
        };
        self.text_styles = match self.mode {
            ThemeMode::Light => TextStyles::light(),
            ThemeMode::Dark => TextStyles::dark(),
        };
    }

    pub fn set_mode(&mut self, mode: ThemeMode) {
        self.mode = mode;
        self.text_styles = match mode {
            ThemeMode::Light => TextStyles::light(),
            ThemeMode::Dark => TextStyles::dark(),
        };
    }
}

impl Default for NotificationStyle {
    fn default() -> Self {
        Self {
            rounding: Rounding {
                nw: 0.0,
                ne: -12.0,
                sw: 0.0,
                se: 12.0,
            },
            margin: Margin {
                left: 0.,
                right: 12.,
                top: 12.,
                bottom: 12.,
            },
            nav_rounding: Rounding {
                nw: 12.0,
                ne: 0.0,
                sw: 12.0,
                se: 0.0,
            },
            sidebar_margin: Margin {
                left: 12.,
                right: 8.,
                top: 8.,
                bottom: 8.,
            },
        }
    }
}

impl NotificationColors {
    pub fn light() -> Self {
        Self {
            background: Color32::from_rgb(245, 245, 245),
            text: Color32::from_rgb(51, 51, 51),
            selected: Color32::from_rgb(0, 120, 212),
            selected_background: Color32::from_rgb(219, 234, 254),
            notification: Color32::LIGHT_RED,
            hover: Color32::from_rgb(229, 229, 229),
            border: Color32::from_rgb(218, 218, 218),
            divider: Color32::from_rgb(51, 51, 51),
            accent: Color32::from_rgb(0, 120, 212),
        }
    }

    pub fn dark() -> Self {
        Self {
            background: Color32::from_rgb(32, 32, 32),
            text: Color32::from_rgb(255, 255, 255),
            selected: Color32::from_rgb(0, 95, 184),
            selected_background: Color32::from_rgb(64, 64, 64),
            notification: Color32::DARK_RED,
            hover: Color32::from_rgb(44, 44, 44),
            border: Color32::from_rgb(70, 70, 70),
            divider: Color32::from_rgb(255, 255, 255),
            accent: Color32::from_rgb(0, 95, 184),
        }
    }
}

impl Default for NotificationFonts {
    fn default() -> Self {
        Self {
            icon: FontId::new(25.0, FontFamily::Proportional),
            label: FontId::new(12.0, FontFamily::Proportional),
            title: FontId::new(16.0, FontFamily::default()),
            content: FontId::new(14.0, FontFamily::default()),
            timestamp: FontId::new(12.0, FontFamily::default()),
            nav_icon: FontId::new(20.0, FontFamily::Proportional),
            nav_label: FontId::new(10.0, FontFamily::Proportional),
        }
    }
}

impl Default for NotificationTheme {
    fn default() -> Self {
        Self {
            mode: ThemeMode::Light,
            style: NotificationStyle::default(),
            light_colors: NotificationColors::light(),
            dark_colors: NotificationColors::dark(),
            fonts: NotificationFonts::default(),
            text_styles: TextStyles::default(),
        }
    }
}
