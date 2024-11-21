use bevy::prelude::*;
use bevy_egui::{egui::Vec2, EguiContexts};
use bevy_svg::prelude::{Origin, Svg2dBundle};
use std::f32::consts::PI;

use crate::resources::{
    setup_ui, AppState, MainCamera, SplashAnimation, SplashCamera, SplashScreen,
};

pub fn splash_start(
    mut cmd: Commands,
    assert_server: Res<AssetServer>,
    mut state: ResMut<AppState>,
    time: Res<Time>,
) {
    cmd.spawn((Camera2dBundle::default(), SplashCamera));
    cmd.spawn(Svg2dBundle {
        svg: assert_server.load("logo/lark.svg"),
        origin: Origin::Center,
        transform: Transform::from_xyz(0., 0., 0.),
        ..Default::default()
    })
    .insert(SplashScreen {
        is_animating: false,
    });
    // sleep 1s
    if time.elapsed_seconds() > 1. {
        *state = AppState::UiSetup;
    }
}

// add splash_to_ui add camera
pub fn splash_to_ui(
    mut cmd: Commands,
    mut state: ResMut<AppState>,
    contexts: EguiContexts,
    mut splash_query: Query<(Entity, &mut Transform, &mut SplashScreen)>,
    splash_camera_query: Query<Entity, With<SplashCamera>>,
) {
    for camera_entity in splash_camera_query.iter() {
        cmd.entity(camera_entity).despawn();
    }
    // add camera
    cmd.spawn((Camera2dBundle::default(), MainCamera)); //
    setup_ui(contexts);

    for (entity, _, mut splash) in splash_query.iter_mut() {
        if !splash.is_animating {
            cmd.entity(entity).insert(SplashAnimation {
                start_pos: Vec2::ZERO,
                end_pos: Vec2::new(-550., 230.), //   ___|(0.0)___
                progress: 0.,
                duration: 2.,
            });
            splash.is_animating = true;
        }
    }
    *state = AppState::SplashAnimate;
}

pub fn ease_in_out(t: f32) -> f32 {
    if t < 0.5 {
        4.0 * t * t * t
    } else {
        let f = 2.0 * t - 2.0;
        0.5 * f * f * f + 1.
    }
}
pub fn animate_splash(
    mut cmd: Commands,
    time: Res<Time>,
    mut splash_query: Query<(Entity, &mut Transform, &SplashScreen, &mut SplashAnimation)>,
    mut state: ResMut<AppState>,
) {
    for (entity, mut transform, _, mut splash) in splash_query.iter_mut() {
        splash.progress += time.delta_seconds();
        let t = (splash.progress / splash.duration).min(1.);
        let eased_t = ease_in_out(t);

        let control_point = Vec2::new(splash.start_pos.x, splash.end_pos.y + 60.);

        let p0 = splash.start_pos;
        let p1 = control_point;
        let p2 = splash.end_pos;

        let one_minus_t = 1.0 - eased_t;

        let pos = p0 * one_minus_t * one_minus_t
            + p1 * 2.0 * one_minus_t * eased_t
            + p2 * eased_t * eased_t;

        transform.translation = Vec3::new(pos.x, pos.y, 0.);
        transform.scale = Vec3::splat(1. - (eased_t * 0.7));
        transform.rotation = Quat::from_rotation_z(eased_t * PI);

        if t >= 1.0 {
            cmd.entity(entity).despawn();
            *state = AppState::Running;
        }
    }
}
