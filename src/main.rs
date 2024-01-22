#![allow(clippy::unnecessary_cast)]
#![allow(unused_imports)]
#![allow(dead_code)]
// #![feature(const_trait_impl)]
// #![feature(type_alias_impl_trait)]
#![allow(unused_mut)]

use {bevy::{prelude::*, window},
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
  // Mesh::
  // let b: Box<dyn Bundle> = Box::new(components::Player);
  // dungeon::main();
  App::new().init_resource::<state::StateStuff>()
            .add_plugins((
    assetstuff::AssetStuffPlugin,
    DefaultPlugins.set(ImagePlugin::default_nearest()),
    ThirdPersonCameraPlugin,
    // Aery,
    bevy_sprite3d::Sprite3dPlugin,
    bevy_xpbd_3d::prelude::PhysicsPlugins::default(),
    input::MyInputPlugin, // game::game_plugin,
                          // input::keylogger,
                          // input::get_pressed_keys_plugin,
                          // tests::tests_plugin
  ))
            .add_systems(Startup, setup::setup)
            .add_systems(Update,
                         (update::gib_sprite_bundle,
                          update::sprites_face_camera,
                          update::player_movement,
                          update::item_pick_up,
                          bevy::window::close_on_esc,
                          jumpy_penguin::segment_path_motion))
            .run();
  // .insert_resource(ClearColor(Color::SALMON))
  // .insert_resource(game::generate_level())
  // .add_plugin(bevy_fps_controller::controller::FpsControllerPlugin)
  // .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
  // .add_startup_system(spawn_planets_and_lunar_lander)
  // .add_system(game::ui)
  // .add_startup_system(load_lunar_lander)
}

// trunk build --release --public-url "bevy_game"
