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
fn keyboard_input(// mut movement_event_writer: EventWriter<MoveHorizontallyAction>,
                  keyboard_input: Res<ButtonInput<KeyCode>>,
                  mouse_button_input: Res<ButtonInput<MouseButton>>,
                  mut cam_q: Query<&mut ThirdPersonCamera>,
                  mut playerq: Query<&Transform, With<Player>>) {
  if keyboard_input.just_pressed(KeyCode::KeyL) {
    playerq.for_each(debug_println);
  }
  if let Ok(mut cam) = cam_q.get_single_mut() {
    if mouse_button_input.just_pressed(MouseButton::Left) {
      cam.cursor_lock_active = !cam.cursor_lock_active;
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
