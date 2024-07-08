use {crate::{assetstuff::AllMyAssetHandles,
             bundletree::BundleTree,
             components::{message, FaceCamera, IsPlayerSprite, ItemPickUp, Message,
                          Player, PlayerFollower, SpinningAnimation, Sun, TimedAnimation},
             setup::{billboard, flashlight, TEXT_SCALE},
             ui::{ui_pop_up, UiPopup}},
     bevy::{math::vec2,
            prelude::*,
            utils::{HashMap, HashSet}},
     bevy_mod_billboard::BillboardTextBundle,
     bevy_rapier3d::prelude::*,
     bevy_sprite3d::{Sprite3d, Sprite3dParams},
     rust_utils::{pairs, vec},
     std::{f32::consts::{PI, TAU},
           ops::Not}};
fn avg<T: std::iter::Sum + std::ops::Div<f32, Output = T>>(coll: impl IntoIterator<Item = T>)
                                                           -> Option<T> {
  let v = vec(coll);
  let n = v.len();
  let s = v.into_iter().sum::<T>();
  (n != 0).then(|| s / (n as f32))
}
pub fn capsule_from_height_and_radius(height: f32, radius: f32) -> Collider {
  Collider::capsule_y(height / 2.0 - radius, radius)
}

pub const PLAYER_HEIGHT: f32 = 1.8;
pub const PLAYER_RADIUS: f32 = 0.3;
pub const PLAYER_WALK_FORCE: f32 = 14.0;
pub const PLAYER_MAX_SPEED: f32 = 11.0;
pub const PLAYER_MIN_JUMP_IMPULSE: f32 = 1.2;
pub const PLAYER_MAX_JUMP_IMPULSE: f32 = 2.9;
pub fn player_movement(keyboard_input: Res<ButtonInput<KeyCode>>,
                       camq: Query<&Transform, With<Camera3d>>,
                       rapier_context: Res<RapierContext>,
                       velq: Query<Option<&Velocity>, Without<Player>>,
                       mut player_sprite_transform_q: Query<&mut Transform,
                             (With<IsPlayerSprite>,
                              Without<Camera3d>,
                              Without<Player>)>,
                       mut playerq: Query<(Entity,
                              &mut ExternalForce,
                              &mut ExternalImpulse,
                              &mut Velocity,
                              &Transform,
                              &mut Player)>) {
  if let (Ok((player_entity,
              mut player_force,
              mut player_impulse,
              mut player_vel,
              player_transform,
              mut player)),
          Ok(cam_transform)) = (playerq.get_single_mut(), camq.get_single())
  {
    let player_walk_zone =
      capsule_from_height_and_radius(PLAYER_HEIGHT * 1.02, PLAYER_RADIUS * 1.02);
    // let player_max_speed = PLAYER_MAX_SPEED + player.speed_boost;
    let player_max_speed = PLAYER_MAX_SPEED;
    let mut entities_colliding_with_player = Vec::new();
    rapier_context.intersections_with_shape(
      player_transform.translation,
      Quat::IDENTITY,
      &player_walk_zone,
      QueryFilter::new().exclude_collider(player_entity),
      |e| {
        entities_colliding_with_player.push(e);
        true
      },
    );
    // if keyboard_input.just_released(KeyCode::K) {
    //   println!("colls:");
    //   for coll in &entities_colliding_with_player {
    //     crate::input::debug_println(coll);
    //   }
    // }
    let vels_of_entities_colliding_with_player =
      velq.iter_many(entities_colliding_with_player)
          .map(|ov| ov.copied().unwrap_or_default().linvel);
    let avg_vel_of_entities_colliding_with_player =
      avg(vels_of_entities_colliding_with_player);
    // let avg_vel_of_entities_colliding_with_player = Some(Vec3::ZERO);
    // let is_grounded = true;
    let is_grounded = avg_vel_of_entities_colliding_with_player.is_some();
    let right = cam_transform.right().normalize();
    let forward = -(right.cross(Vec3::Y).normalize_or_zero());
    let dir = [(KeyCode::KeyD, Vec2::X),
               (KeyCode::KeyA, Vec2::NEG_X),
               (KeyCode::KeyW, Vec2::Y),
               (KeyCode::KeyS, Vec2::NEG_Y)].into_iter()
                                            .filter_map(|(k, v)| {
                                              keyboard_input.pressed(k).then_some(v)
                                            })
                                            .sum::<Vec2>()
                                            .normalize_or_zero();
    let Vec2 { x, y } = dir;
    // if grounded
    player_force.force = if let Some(avgvel) = avg_vel_of_entities_colliding_with_player {
      let relvel = player_vel.linvel - avgvel;
      let relspeed = relvel.length();
      let desired_force = Vec3 { y: 0.0,
                                 ..relvel * (-1.6) }
                          + (right * x + forward * y) * PLAYER_WALK_FORCE;
      if relspeed < 0.1 {
        desired_force
      } else {
        let desired_parallel = desired_force.project_onto(relvel);
        let desired_perpendicular = desired_force - desired_parallel;
        let desired_parallel_bounded = if desired_parallel.dot(relvel).is_sign_positive() {
          desired_parallel * (1.0 - relspeed / player_max_speed)
        } else {
          desired_parallel
        };
        desired_parallel_bounded + desired_perpendicular
      }
    } else {
      Vec3::ZERO
    };
    let charge_fraction = player.jump_charge_level.fraction();
    if is_grounded && keyboard_input.just_released(KeyCode::Space) {
      player_impulse.impulse = Vec3::Y
                               * (PLAYER_MIN_JUMP_IMPULSE
                                  + ((PLAYER_MAX_JUMP_IMPULSE - PLAYER_MIN_JUMP_IMPULSE)
                                     * charge_fraction));
    }
    if keyboard_input.pressed(KeyCode::Space) {
      player.jump_charge_level.next()
    } else {
      player.jump_charge_level = default();
    }
    if let Ok(mut player_sprite_transform) = player_sprite_transform_q.get_single_mut() {
      player_sprite_transform.translation.y = (-charge_fraction) * 0.2;
      player_sprite_transform.scale =
        (Vec3::ONE - Vec3::Y * (charge_fraction * 0.3)) * PLAYER_HEIGHT;
    }
  }
}
pub fn face_camera(camq: Query<&GlobalTransform, With<Camera3d>>,
                   mut camera_facers_q: Query<(&mut Transform, &GlobalTransform),
                         (With<FaceCamera>, Without<Camera3d>)>) {
  if let Ok(cam_globaltransform) = camq.get_single() {
    for (mut transform, globaltransform) in &mut camera_facers_q {
      let dir = Vec3 { y: 0.0,
                       ..(globaltransform.translation()
                          - cam_globaltransform.translation()) };
      transform.look_to(dir, Vec3::Y);
    }
  }
}
#[derive(Component)]
pub struct FaceCameraDir;
pub fn face_camera_dir(camq: Query<&Transform, With<Camera3d>>,
                       mut camera_facers_q: Query<&mut Transform,
                             (With<FaceCameraDir>,
                              Without<Camera3d>)>) {
  if let Ok(cam_transform) = camq.get_single() {
    for mut transform in &mut camera_facers_q {
      transform.look_to(cam_transform.forward().into(), Vec3::Y);
    }
  }
}
#[derive(Component)]
pub struct Billboard {
  pub transform: Transform,
  pub image_handle: Handle<Image>,
  pub unlit: bool
}
pub fn gib_billboard(mut sprite_3d_params: Sprite3dParams,
                     mut c: Commands,
                     q: Query<(Entity, &Billboard)>) {
  for (e,
       Billboard { transform,
                   image_handle,
                   unlit }) in &q
  {
    if let Some(image) = sprite_3d_params.images.get(image_handle.clone()) {
      c.entity(e)
       .remove::<Billboard>()
       .insert(Sprite3d { image: image_handle.clone(),
                          transform: *transform,
                          pixels_per_metre: image.height() as f32,
                          double_sided: true,
                          unlit: *unlit,
                          ..default() }.bundle(&mut sprite_3d_params));
    }
  }
}
#[derive(Component)]
pub struct AnimatedBillboard {
  pub transform: Transform,
  pub image_handle: Handle<Image>,
  pub unlit: bool,
  pub num_frames: usize
}
pub fn gib_animated_billboard(mut sprite_3d_params: Sprite3dParams,
                              mut c: Commands,
                              q: Query<(Entity, &AnimatedBillboard)>) {
  for (e, animated_billboard) in &q {
    let image_handle = animated_billboard.image_handle.clone();
    if let Some(image) = sprite_3d_params.images.get(&image_handle) {
      let &AnimatedBillboard { transform,
                               unlit,
                               num_frames,
                               .. } = animated_billboard;
      let image_width = image.width() as f32;
      let image_height = image.height() as f32;
      let frame_width = image_width / (num_frames as f32);
      let texture_atlas_layout_handle =
        sprite_3d_params.atlas_layouts
                        .add(TextureAtlasLayout::from_grid(vec2(frame_width, image_height),
                                                           num_frames,
                                                           1,
                                                           None,
                                                           None));
      let texture_atlas = TextureAtlas { layout: texture_atlas_layout_handle,
                                         index: 0 };
      c.entity(e)
       .remove::<AnimatedBillboard>()
       .insert(Sprite3d { image: image_handle,
                          transform,
                          // alpha_mode: AlphaMode::Blend,
                          pixels_per_metre: image_height,
                          double_sided: true,
                          unlit,
                          ..default() }.bundle_with_atlas(&mut sprite_3d_params,
                                                          texture_atlas));
    }
  }
}
pub fn spawn_mushroom_man(playerq: Query<&Transform, With<Player>>,
                          keyboard_input: Res<ButtonInput<KeyCode>>,
                          mut c: Commands,
                          amah: Res<AllMyAssetHandles>) {
  if let Ok(&player_transform) = playerq.get_single() {
    if keyboard_input.just_pressed(KeyCode::KeyZ) {
      let height = 1.3;
      (PlayerFollower,
       Friction::new(2.9),
       RigidBody::Dynamic,
       Velocity::default(),
       ExternalForce::default(),
       ExternalImpulse::default(),
       LockedAxes::ROTATION_LOCKED,
       capsule_from_height_and_radius(height, 0.3),
       ColliderMassProperties::Mass(0.1),
       SpatialBundle::from_transform(player_transform))
        .with_child((FaceCamera,
                     billboard(Transform::from_scale(Vec3::splat(height * 1.15)),
                               amah.mushroom_man())))
        .with_child(message("spawned a mushroom man", default()))
        // .with_child((SpatialBundle{
        //   transform: Transform::from_translation(Vec3::Y).with_scale(Vec3::splat(0.03)),
        //   ..default()
        // },
        //              UiPopup::new(["some text"]),
        // ))
        .spawn(&mut c);
    }
  }
}
pub fn player_follower(mut followerq: Query<(&mut ExternalForce, &Transform),
                             With<PlayerFollower>>,
                       mut playerq: Query<&Transform, With<Player>>) {
  if let Ok(player_transform) = playerq.get_single() {
    for (mut follower_force, follower_transform) in &mut followerq {
      follower_force.force =
        (player_transform.translation - follower_transform.translation) * 0.6;
    }
  }
}
const PICKUPDISTANCE: f32 = 1.1;
const SPEEDBOOSTAMOUNT: f32 = 8.0;
pub fn item_pick_up(mut playerq: Query<(&Transform, &mut Player)>,
                    playerspriteq: Query<Entity, With<IsPlayerSprite>>,
                    itemsq: Query<(Entity, &Transform, &ItemPickUp)>,
                    amah: Res<AllMyAssetHandles>,
                    mut c: Commands) {
  if let Ok((&player_transform, mut player)) = playerq.get_single_mut() {
    for (item, &item_transform, &item_pick_up) in &itemsq {
      if player_transform.translation
                         .distance(item_transform.translation)
         < PICKUPDISTANCE
      {
        c.entity(item).despawn_recursive();
        match item_pick_up {
          ItemPickUp::CoffeeCup => {
            // player.speed_boost += SPEEDBOOSTAMOUNT;
            player.num_coffee_cups += 1;
            c.spawn(message(format!("You have {} coffee cups", player.num_coffee_cups),
                            player_transform.translation));
          }
          ItemPickUp::GetFlashLight => {
            if let Ok(player_sprite) = playerspriteq.get_single() {
              flashlight(Transform::from_xyz(-0.49, 0.25, 0.0).with_scale(Vec3::splat(0.06))
                                                             .looking_to(Vec3::NEG_Z, Vec3::Y),
                         &amah).spawn_as_child(player_sprite, &mut c);
              c.spawn(message("you found a flashlight", player_transform.translation));
            }
          }
          ItemPickUp::HealthBoost(_) => todo!()
        }
      }
    }
  }
}
pub fn spinning_animation(mut q: Query<(&mut Transform, &mut SpinningAnimation)>) {
  for (mut t, mut sa) in &mut q {
    let rotation_angle_radians = sa.rotation_steps.fraction() * TAU;
    t.rotation = Quat::from_rotation_y(rotation_angle_radians);

    let sine_offset = (sa.up_down_steps.fraction() * TAU).sin() * sa.up_down_distance;
    t.translation.y = sine_offset;

    sa.rotation_steps.next();
    sa.up_down_steps.next();
  }
}
pub const MESSAGE_SHOW_TIME_TICKS: u32 = 230;
pub const MESSAGE_RAISE_ALT: f32 = 1.1;
// pub const MESSAGE_RAISE_TIME_TICKS: u32 = 50;
pub fn show_message(mut q: Query<(Entity, &mut Transform, &mut Message)>, mut c: Commands) {
  for (e, mut t, mut m) in &mut q {
    if m.age_ticks > MESSAGE_SHOW_TIME_TICKS {
      c.entity(e).despawn_recursive();
    } else {
      let scale =
        (((m.age_ticks as f32) / (MESSAGE_SHOW_TIME_TICKS as f32)) * PI).sin()
                                                                        .powf(0.11);
      t.translation = m.origin_pos + (Vec3::Y * MESSAGE_RAISE_ALT * scale);
      t.scale = Vec3::splat(TEXT_SCALE * scale);

      m.age_ticks += 1;
    }
  }
}
pub fn sun_movement(mut camq: Query<&GlobalTransform, With<Camera3d>>,
                    mut sunq: Query<(&mut Sun, &mut Transform)>) {
  if let (Ok(camera_globaltransform), Ok((mut sun, mut sun_transform))) =
    (camq.get_single(), sunq.get_single_mut())
  {
    sun.0.next();
    let rot_radians = sun.0.fraction() * TAU;
    let cam_pos = camera_globaltransform.translation();
    let new_sun_pos = cam_pos
                      + Vec3 { x: rot_radians.cos() * 100.0,
                               y: 60.0,
                               z: rot_radians.sin() * 100.0 };
    sun_transform.translation = new_sun_pos;

    let dir = new_sun_pos - cam_pos;
    sun_transform.look_to(dir, Vec3::Y);
  }
}

// pub fn camera(mut camq: Query<(&mut Transform), With<Camera3d>>,
//               keys: Res<ButtonInput<KeyCode>>,
//               window_q: Query<&Window, With<PrimaryWindow>>,
//               mouse: Res<ButtonInput<MouseButton>>,
//               mut mouse_evr: EventReader<MouseMotion>,
//               mut scroll_evr: EventReader<MouseWheel>,
//               mut playerq: Query<&Transform, With<Player>>) {
//               raycast...
//   if let Ok(window) = window_q.get_single() {
//     window.cursor.grab_mode
// }
//   for (e, mut t, mut m) in &mut q {
//     if m.age_ticks > MESSAGE_SHOW_TIME_TICKS {
//       c.entity(e).despawn();
//     } else {
//       t.translation.y = m.origin_pos.y
//                         + MESSAGE_RAISE_ALT
//                           * if m.age_ticks > MESSAGE_RAISE_TIME_TICKS {
//                             1.0
//                           } else {
//                             (m.age_ticks as f32) / (MESSAGE_RAISE_TIME_TICKS as f32)
//                           };
//       m.age_ticks += 1;
//     }
//   }
// }

const CRAZY_CUBES_DIST: i32 = 8;
#[derive(Default, Resource)]
pub struct CrazyCubes(pub HashMap<IVec3, Entity>);
pub fn crazy_cubes(mut c: Commands,
                   amah: Res<AllMyAssetHandles>,
                   playerq: Query<&Transform, With<Player>>,
                   mut cubes: Local<CrazyCubes>) {
  if let Ok(&Transform { translation: playerpos,
                         .. }) = playerq.get_single()
  {
    let center_cube_pos = IVec3 { x: playerpos.x.round() as i32,
                                  y: -70,
                                  z: playerpos.z.round() as i32 };
    let desired_cube_poses: HashSet<IVec3> =
      pairs(-CRAZY_CUBES_DIST..=CRAZY_CUBES_DIST,
            -CRAZY_CUBES_DIST..=CRAZY_CUBES_DIST).filter_map(|(relx, relz)| {
        let rel_pos = IVec3 { x: relx,
                              y: 0,
                              z: relz };
        let cube_pos = center_cube_pos + rel_pos;
        (rel_pos.length_squared() <= CRAZY_CUBES_DIST.pow(2)).then_some(cube_pos)
      })
      .collect();
    let to_remove =
      vec(cubes.0
               .keys()
               .filter_map(|&pos| desired_cube_poses.contains(&pos).not().then_some(pos)));
    let to_add =
      vec(desired_cube_poses.iter().filter_map(|&pos| {
                                     cubes.0.contains_key(&pos).not().then_some(pos)
                                   }));
    let vec3_from_ivec3 = |IVec3 { x, y, z }| Vec3 { x: x as f32,
                                                     y: y as f32,
                                                     z: z as f32 };
    for pos in to_remove {
      let e = cubes.0.remove(&pos).unwrap();
      c.entity(e).despawn();
    }
    for pos in to_add {
      let e = c.spawn((RigidBody::Fixed,
                       Friction::default(),
                       Velocity::default(),
                       AsyncCollider(ComputedColliderShape::ConvexHull),
                       PbrBundle { mesh: amah.unitcube(),
                                   material: amah.grass_material(),
                                   transform:
                                     Transform::from_translation(vec3_from_ivec3(pos)),
                                   ..default() }))
               .id();
      cubes.0.insert(pos, e);
    }
  }
}
#[derive(Default, Resource)]
pub struct TimeTicks(pub u32);
pub fn increment_time(mut time: ResMut<TimeTicks>) { time.0 += 1; }
pub fn timed_animation_system(time_ticks: Res<TimeTicks>,
                              mut q: Query<(&TimedAnimation, &mut TextureAtlas)>) {
  for (&TimedAnimation { num_frames,
                         time_per_frame_in_ticks },
       mut atlas) in &mut q
  {
    let time = time_ticks.0 as usize;
    let index = |time| (time / time_per_frame_in_ticks) % num_frames;
    let old_index = index(time.saturating_sub(1));
    let new_index = index(time);
    if new_index != old_index {
      atlas.index = new_index;
    }
  }
}

#[derive(Component, Default)]
pub struct NumberThing(pub u8);
pub fn number_thing(mut number_thing_q: Query<(&mut NumberThing,
                           &mut UiPopup,
                           &GlobalTransform)>,
                    mut playerq: Query<&Transform, With<Player>>,
                    keyboard_input: Res<ButtonInput<KeyCode>>) {
  if let Ok(&player_transform) = playerq.get_single() {
    for (mut n, mut p, &t) in &mut number_thing_q {
      p.strings = vec![" ArrowUp / ArrowDown ".to_string(), n.0.to_string()];
      if player_transform.translation.distance(t.translation()) < 5.0 {
        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
          n.0 = n.0.wrapping_add(1);
        }
        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
          n.0 = n.0.wrapping_sub(1);
        }
      }
    }
  }
}
