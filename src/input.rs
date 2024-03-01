use std::fmt::Debug;

use bevy::core_pipeline::bloom::{BloomCompositeMode, BloomPrefilterSettings, BloomSettings};

use {bevy::window::{CursorGrabMode, PrimaryWindow},
     bevy_vox_scene::VoxelSceneBundle};

use crate::{assetstuff::AllMyAssetHandles, components::message};

use {crate::components::Player,
     bevy::prelude::{ButtonInput, KeyCode, Res, *},
     bevy_third_person_camera::ThirdPersonCamera,
     rust_utils::comment};
pub fn debug_println(t: impl core::fmt::Debug) {
  println!("{:?}", t);
}
#[derive(Event)]
pub struct MoveHorizontallyAction(pub Vec2);
#[derive(Event)]
pub struct JumpAction;
#[derive(Event)]
pub struct JumpStart;
#[derive(Event)]
pub struct JumpEnd;
// does things based on keyboard input
pub fn debugfmt(t: impl Debug) -> String { format!("{:?}", t) }
fn keyboard_input(keyboard_input: Res<ButtonInput<KeyCode>>,
                  mouse_button_input: Res<ButtonInput<MouseButton>>,
                  mut window_q: Query<&mut Window, With<PrimaryWindow>>,
                  mut cam_q: Query<(Entity,
                         &mut ThirdPersonCamera,
                         Option<&BloomSettings>)>,
                  amah: Res<AllMyAssetHandles>,
                  mut c: Commands,
                  mut playerq: Query<&Transform, With<Player>>) {
  if keyboard_input.just_pressed(KeyCode::KeyR) {
    if let Ok(mut window) = window_q.get_single_mut() {
      window.cursor.grab_mode = CursorGrabMode::None;
    }
  }
  if keyboard_input.just_pressed(KeyCode::KeyL) {
    if let Ok(&player_transform) = playerq.get_single() {
      c.spawn(message(debugfmt(player_transform), player_transform.translation));
      // c.spawn(VoxelSceneBundle { scene: amah.flashlight.clone(),
      //                              transform:
      //                                player_transform,
      //                              visibility: Visibility::Visible
      //                               },
      //           // SpinningAnimation { rotation_steps: default(),
      //           //                     up_down_steps: default(),
      //           //                     up_down_distance: 0.3 }
      //          );
      if let Ok(mut window) = window_q.get_single_mut() {
        c.spawn(message(debugfmt(window.cursor.grab_mode),
                        player_transform.translation));
      }
    }
  }
  if let Ok((cam_e, mut cam, obs)) = cam_q.get_single_mut() {
    if mouse_button_input.just_pressed(MouseButton::Left) {
      cam.cursor_lock_active = !cam.cursor_lock_active;
    }
    if keyboard_input.just_pressed(KeyCode::KeyT) {
      if obs.is_some() {
        c.entity(cam_e).remove::<BloomSettings>();
      } else {
        c.entity(cam_e).insert(BloomSettings { intensity: 0.5,
                                               low_frequency_boost: 0.0,
                                               prefilter_settings:
                                                 BloomPrefilterSettings { threshold: 2.2,
                                                                          ..default() },
                                               composite_mode:
                                                 BloomCompositeMode::Additive,
                                               ..default() });
      }
    }
  }
  // let dir =
  //   [(KeyCode::D, Vec2::X),
  //    (KeyCode::A, Vec2::NEG_X),
  //    (KeyCode::W, Vec2::Y),
  //    (KeyCode::S, Vec2::NEG_Y)].into_iter()
  //                              .filter_map(|(k, v)| keyboard_input.pressed(k).then_some(v))
  //                              .sum::<Vec2>();
  // if dir != Vec2::ZERO {
  //   movement_event_writer.send(MoveHorizontallyAction(dir.normalize()));
  // }
  // if keyboard_input.just_pressed(KeyCode::Space) {
  //   jump_event_writer.send(JumpAction);
  // }
}
pub struct MyInputPlugin;
impl Plugin for MyInputPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<MoveHorizontallyAction>()
       .add_event::<JumpAction>()
       .add_event::<JumpStart>()
       .add_event::<JumpEnd>()
       .add_systems(Update, keyboard_input);
  }
}

fn log_inputs(keys: Res<ButtonInput<KeyCode>>) {
  keys.get_just_pressed()
      .for_each(|k| println!("{:?} was pressed!", k));
}
// pub fn keylogger(app: &mut App) { app.add_systems(log_inputs); }

use {bevy::{prelude::*, utils::HashMap},
     bevy_rapier3d::prelude::Velocity};

comment! {
#[derive(Resource, Default)]
pub struct PressedKeys(pub HashSet<KeyCode>);
fn get_pressed_keys_system(mut r: ResMut<PressedKeys>, i: Res<ButtonInput<KeyCode>>) {
  *r.0 = i.get_pressed().collect();
}
pub fn get_pressed_keys_plugin(app: &mut App) {
  app.init_resource::<PressedKeys>()
     .add_system(get_pressed_keys_system);
}

  impl GameControl {
    pub fn pressed(&self, keyboard_input: &Res<ButtonInput<KeyCode>>) -> bool {
      let p = |k| keyboard_input.pressed(k);
      match self {
        GameControl::Up => p(KeyCode::W) || p(KeyCode::Up),
        GameControl::Down => p(KeyCode::S) || p(KeyCode::Down),
        GameControl::Left => p(KeyCode::A) || p(KeyCode::Left),
        GameControl::Right => p(KeyCode::D) || p(KeyCode::Right)
      }
    }
  }
}
