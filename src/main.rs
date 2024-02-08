#![allow(clippy::unnecessary_cast)]
#![allow(unused_imports)]
#![allow(dead_code)]
// #![feature(const_trait_impl)]
// #![feature(type_alias_impl_trait)]
#![allow(unused_mut)]

use {bevy::{prelude::*, window},
     bevy_rapier3d::prelude::NoUserData,
     bevy_third_person_camera::ThirdPersonCameraPlugin};

pub mod assetstuff;
pub mod components;
pub mod dungeon;
pub mod input;
pub mod jumpy_penguin;
pub mod setup;
pub mod state;
pub mod update;
// pub mod game;
// pub mod gamething;
// pub mod tests;
// pub mod input;
// pub mod gol;
// pub mod game2d;
// pub mod dynamic_character_3d;
// pub mod examples_common_3d;
// pub mod camera;
// pub mod kinematic_character_3d;
// pub mod physics;
// pub mod lunarlander3d;
// pub mod tiles;
// // mod audio;
// // mod menu;
// pub mod loading;
// pub mod lunarlander3d;
// pub mod menu;
// impl<F: FnOnce(&mut App)> Plugin for F {
//   fn build(&self, app: &mut App) { self.call_once(app); }
// }
#[bevy_main]
pub fn main() {
  App::new().init_resource::<state::StateStuff>()
            .add_plugins((
    DefaultPlugins.set(ImagePlugin::default_nearest())
                  .set(WindowPlugin {
                    primary_window: Some(Window {
                      // resolution:window::WindowResolution::new(1080.0,1080.0*3./4.),
                      // cursor: window::Cursor{
                      //   // grab_mode: window::CursorGrabMode::Confined ,
                      //   visible: false,
                      //   ..default()},
                      mode: window::WindowMode::BorderlessFullscreen,
                      present_mode: window::PresentMode::AutoNoVsync,
                      title: "bevy_game".to_string(),
                      canvas: Some("#bevy".to_string()),
                      fit_canvas_to_parent: true,
                      // prevent_default_event_handling: todo!(),
                       ..default()}),
                    ..default()
                  }),
    bevy_vox::VoxPlugin::default(),
    assetstuff::AssetStuffPlugin,
    ThirdPersonCameraPlugin,
    // Aery,
    bevy_tnua_rapier3d::TnuaRapier3dPlugin,
    bevy_tnua::prelude::TnuaControllerPlugin,
    bevy_sprite3d::Sprite3dPlugin,
    bevy_rapier3d::prelude::RapierPhysicsPlugin::<NoUserData>::default(),
    input::MyInputPlugin, // game::game_plugin,
                          // input::keylogger,
                          // input::get_pressed_keys_plugin,
                          // tests::tests_plugin
  ))

            .insert_resource(AmbientLight{ color: Color::ALICE_BLUE, brightness: 0.2 })

        // .insert_resource(bevy_xpbd_3d::resources::SubstepCount(2))
            .add_systems(Startup, setup::setup)
            .add_systems(Update,
                         (update::gib_sprite_bundle,
                          update::sprites_face_camera,
                          update::player_movement,
                          update::item_pick_up,
                          update::spawn_mushroom_man,
                          update::player_follower,
                          update::spinning_animation,
                          bevy::window::close_on_esc,
                          jumpy_penguin::segment_path_motion))
            .run();
  // .insert_resource(ClearColor(Color::SALMON))
  // .insert_resource(game::generate_level())
  // .add_plugin(bevy_fps_controller::controller::FpsControllerPlugin)
  // .add_startup_system(spawn_planets_and_lunar_lander)
  // .add_system(game::ui)
  // .add_startup_system(load_lunar_lander)
}

// trunk build --release --public-url "bevy_game" --filehash false

// trunk serve
