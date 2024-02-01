use {crate::components::Player,
     bevy::prelude::{Input, KeyCode, Res, *},
     bevy_third_person_camera::ThirdPersonCamera,
     bevy_xpbd_3d::prelude::{ExternalForce, ExternalImpulse, Rotation, ShapeCaster,
                             ShapeHits},
     rust_utils::comment};
fn debug_println(t: impl core::fmt::Debug) {
  println!("{:?}", t);
}
#[derive(Event)]
pub struct MoveHorizontallyAction(pub Vector2);
#[derive(Event)]
pub struct JumpAction;
#[derive(Event)]
pub struct JumpStart;
#[derive(Event)]
pub struct JumpEnd;
// does things based on keyboard input
fn keyboard_input(mut movement_event_writer: EventWriter<MoveHorizontallyAction>,
                  mut jump_event_writer: EventWriter<JumpAction>,
                  // mut jump_start_event_writer: EventWriter<JumpStart>,
                  // mut jump_end_event_writer: EventWriter<JumpEnd>,
                  keyboard_input: Res<Input<KeyCode>>,
                  mouse_button_input: Res<Input<MouseButton>>,
                  mut cam_q: Query<&mut ThirdPersonCamera>,
                  // _gltfs: Res<Assets<Gltf>>,
                  // _amah: Res<AllMyAssetHandles>,
                  playerq: Query<&Transform, With<Player>>,
                  q: Query<(Has<Rotation>,
                         Has<ShapeCaster>,
                         Has<ShapeHits>,
                         // Has<Grounded>,
                         Has<ExternalForce>,
                         Has<ExternalImpulse>),
                        With<Player>>) {
  // if keyboard_input.just_pressed(KeyCode::Space) {
  //   jump_start_event_writer.send(JumpStart);
  // }
  if keyboard_input.just_pressed(KeyCode::L) {
    playerq.for_each(debug_println);
  }
  if let Ok(mut cam) = cam_q.get_single_mut() {
    if mouse_button_input.just_pressed(MouseButton::Left) {
      cam.cursor_lock_active = !cam.cursor_lock_active;
    }
  }
  if keyboard_input.just_pressed(KeyCode::G) {
    debug_println(&q);
    // debug_println(gltfs.get(amah.character_controller_demo_scene_gltf.clone())
    //                    .unwrap());
  }
  if keyboard_input.just_pressed(KeyCode::P) {
    let tup = q.single();
    println!("Rotation: {}", tup.0);
    println!("ShapeCaster: {}", tup.1);
    println!("ShapeHits: {}", tup.1);
    println!("Grounded: {}", tup.3);
    println!("Externalforce: {}", tup.4);
    println!("ExternalImpulse: {}", tup.4);
  }
  let dir =
    [(KeyCode::D, Vec2::X),
     (KeyCode::A, Vec2::NEG_X),
     (KeyCode::W, Vec2::Y),
     (KeyCode::S, Vec2::NEG_Y)].into_iter()
                               .filter_map(|(k, v)| keyboard_input.pressed(k).then_some(v))
                               .sum::<Vec2>()
                               .normalize_or_zero();
  if dir != Vector2::ZERO {
    movement_event_writer.send(MoveHorizontallyAction(dir.normalize()));
  }
  if keyboard_input.just_pressed(KeyCode::Space) {
    jump_event_writer.send(JumpAction);
  }
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

fn log_inputs(keys: Res<Input<KeyCode>>) {
  keys.get_just_pressed()
      .for_each(|k| println!("{:?} was pressed!", k));
}
// pub fn keylogger(app: &mut App) { app.add_systems(log_inputs); }

use {bevy::{prelude::*, utils::HashMap},
     bevy_xpbd_3d::math::Vector2};

comment! {
#[derive(Resource, Default)]
pub struct PressedKeys(pub HashSet<KeyCode>);
fn get_pressed_keys_system(mut r: ResMut<PressedKeys>, i: Res<Input<KeyCode>>) {
  *r.0 = i.get_pressed().collect();
}
pub fn get_pressed_keys_plugin(app: &mut App) {
  app.init_resource::<PressedKeys>()
     .add_system(get_pressed_keys_system);
}

  impl GameControl {
    pub fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
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
