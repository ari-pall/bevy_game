#![allow(clippy::unnecessary_cast)]
#![allow(unused_imports)]
#![allow(dead_code)]
// #![feature(const_trait_impl)]
// #![feature(type_alias_impl_trait)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]
#![feature(variant_count)]
#![feature(strict_overflow_ops)]
#![feature(iter_intersperse)]
// #![feature(int_roundings)]
// #![recursion_limit = "1024"]
// #![feature(const_fn_floating_point_arithmetic)]

use {bevy::{prelude::*, window},
     bevy_rapier3d::prelude::NoUserData,
     bevy_third_person_camera::ThirdPersonCameraPlugin,
     setup::AMBIENT_LIGHT,
     update::TimeTicks,
     voxels::{voxels_init, MyVoxelRegistry}};

pub mod assetstuff;
pub mod bundletree;
pub mod components;
pub mod ui;
pub mod voxels;
// pub mod dungeon;
pub mod input;
pub mod jumpy_penguin;
pub mod setup;
pub mod state;
pub mod update;

#[bevy_main]
pub fn main() {
  App::new()
      .add_plugins((
        // bevy::pbr::ScreenSpaceAmbientOcclusionPlugin
        DefaultPlugins
          .set(ImagePlugin::default_nearest())
          .set(WindowPlugin {
            primary_window: Some(Window {
              present_mode: window::PresentMode::AutoNoVsync,
              title: "bevy_game".to_string(),
              canvas: Some("#bevy".to_string()),
              ..default()
            }),
            ..default()
          }),
        bevy_sprite3d::Sprite3dPlugin,
        // bevy_obj::ObjPlugin,
        // bevy_vox::VoxPlugin::default(),
        bevy_vox_scene::VoxScenePlugin,
        assetstuff::AssetStuffPlugin,
        ThirdPersonCameraPlugin,
        bevy_mod_billboard::prelude::BillboardPlugin,
        bevy_rapier3d::prelude::RapierPhysicsPlugin::<NoUserData>::default(),
        input::MyInputPlugin,
      ))
      .init_resource::<TimeTicks>()
      .init_resource::<MyVoxelRegistry>()


    // .init_asset::<bevy_vox_scene::scene::VoxelScene>()
    .insert_resource(AMBIENT_LIGHT)
      .add_systems(Startup, setup::setup)
      .add_systems(Startup, voxels_init)
      .add_systems(
        Update,
        (
          // update::gib_sprite_bundle,
          update::face_camera,
          update::face_camera_dir,
          update::player_movement,
          update::item_pick_up,
          update::spawn_mushroom_man,
          update::player_follower,
          update::spinning_animation,
          update::sun_movement,
          update::show_message,
          update::crazy_cubes,
          update::gib_billboard,
          update::gib_animated_billboard,
          update::increment_time,
          update::timed_animation_system,
          bevy::window::close_on_esc,
          jumpy_penguin::segment_path_motion,
          ui::ui_pop_up
        ),
      )
      // .add_systems(
      //   Update,
      // )

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
// cargo check --target wasm32-unknown-unknown
