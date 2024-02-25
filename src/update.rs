use {crate::{assetstuff::AllMyAssetHandles,
             components::{FaceCamera, IsPlayerSprite, ItemPickUp, Player, PlayerFollower,
                          SpinningAnimation, Sun, SunSprite},
             setup::{billboard, spawn_with_child}},
     bevy::prelude::*,
     bevy_rapier3d::prelude::*,
     rust_utils::vec,
     std::f32::consts::TAU};
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
                              &mut Friction,
                              &Transform,
                              &mut Player)>) {
  if let (Ok((player_entity,
              mut player_force,
              mut player_impulse,
              mut player_vel,
              mut player_friction,
              player_transform,
              mut player)),
          Ok(cam_transform)) = (playerq.get_single_mut(), camq.get_single())
  {
    let player_walk_zone =
      capsule_from_height_and_radius(PLAYER_HEIGHT * 1.02, PLAYER_RADIUS * 1.02);
    // player_friction.coefficient = if player_vel.linvel.y > 0.03 { 0.0 } else { 1.0 };
    let player_max_speed = PLAYER_MAX_SPEED + player.speed_boost;
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
                       ..(cam_globaltransform.translation()
                          - globaltransform.translation()) };
      transform.look_to(dir, Vec3::Y);
    }
  }
}

// pub fn gib_sprite_bundle(mut sprite_3d_params: Sprite3dParams,
//                          mut c: Commands,
//                          q: Query<(Entity, &GibSpriteBundle)>) {
//   for (e, GibSpriteBundle(s)) in &q {
//     if sprite_3d_params.images.contains(&s.image) {
//       c.entity(e)
//        .remove::<GibSpriteBundle>()
//        .insert(Sprite3d { image: s.image.clone(),
//                           ..*s }.bundle(&mut sprite_3d_params));
//     }
//   }
// }
pub fn spawn_mushroom_man(playerq: Query<&Transform, With<Player>>,
                          keyboard_input: Res<ButtonInput<KeyCode>>,
                          mut c: Commands,
                          amah: Res<AllMyAssetHandles>) {
  if let Ok(&player_transform) = playerq.get_single() {
    if keyboard_input.just_pressed(KeyCode::KeyZ) {
      let height = 1.3;
      spawn_with_child(&mut c,
                       (PlayerFollower,
                        Friction::new(2.9),
                        RigidBody::Dynamic,
                        Velocity::default(),
                        ExternalForce::default(),
                        ExternalImpulse::default(),
                        LockedAxes::ROTATION_LOCKED,
                        capsule_from_height_and_radius(height, 0.3),
                        // Collider::capsule_y(0.4, 0.2),
                        ColliderMassProperties::Mass(0.1),
                        SpatialBundle::from_transform(player_transform)),
                       (FaceCamera,
                        billboard(Transform::from_scale(Vec3::splat(height * 1.15)),
                                  amah.mushroom_man.clone(),
                                  &amah)));
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
const PICKUPDISTANCE: f32 = 0.7;
const SPEEDBOOSTAMOUNT: f32 = 8.0;
pub fn item_pick_up(mut playerq: Query<(&Transform, &mut Player)>,
                    itemsq: Query<(Entity, &Transform, &ItemPickUp)>,
                    mut c: Commands) {
  if let Ok((&player_transform, mut player)) = playerq.get_single_mut() {
    for (item, &item_transform, &item_pick_up) in &itemsq {
      if player_transform.translation
                         .distance(item_transform.translation)
         < PICKUPDISTANCE
      {
        c.entity(item).despawn_recursive();
        match item_pick_up {
          ItemPickUp::SpeedBoost => {
            player.speed_boost += SPEEDBOOSTAMOUNT;
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
pub fn sun_movement(mut camq: Query<&GlobalTransform, With<Camera3d>>,
                    mut sunq: Query<(&mut Sun, &mut Transform), Without<Camera3d>>,
                    mut sun_sprite_q: Query<&mut Transform,
                          (With<SunSprite>,
                           Without<Camera3d>,
                           Without<Sun>)>) {
  if let (Ok(camera_globaltransform),
          Ok((mut sun, mut sun_transform)),
          Ok(mut sun_sprite_transform)) =
    (camq.get_single(), sunq.get_single_mut(), sun_sprite_q.get_single_mut())
  {
    sun.0.next();
    let rot_radians = sun.0.fraction() * TAU;
    let cam_pos = camera_globaltransform.translation();
    let sun_pos = cam_pos
                  + Vec3 { x: rot_radians.cos() * 100.0,
                           y: 60.0,
                           z: rot_radians.sin() * 100.0 };
    sun_sprite_transform.translation = sun_pos;

    let dir = cam_pos - sun_pos;
    sun_transform.look_to(dir, Vec3::Y);
    sun_sprite_transform.look_to(dir, Vec3::Y);
  }
}
