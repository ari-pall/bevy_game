use bevy::sprite::MaterialMesh2dBundle;

use {crate::{assetstuff::AllMyAssetHandles,
             components::{GibSpriteBundle, ItemPickUp, Player, PlayerFollower},
             input::*,
             setup::spawn_with_child},
     bevy::prelude::*,
     bevy_rapier3d::prelude::*,
     bevy_sprite3d::{Sprite3d, Sprite3dComponent, Sprite3dParams},
     rust_utils::vec};
fn avg<T: std::iter::Sum + std::ops::Div<f32, Output = T>>(coll: impl IntoIterator<Item = T>)
                                                           -> Option<T> {
  let v = vec(coll);
  let n = v.len();
  let s = v.into_iter().sum::<T>();
  (n != 0).then(|| s / (n as f32))
}
const PLAYER_WALK_FORCE: f32 = 14.0;
const PLAYER_MAX_SPEED: f32 = 13.0;
const PLAYER_MIN_JUMP_IMPULSE: f32 = 1.2;
const PLAYER_MAX_JUMP_IMPULSE: f32 = 2.9;
const PLAYER_JUMP_CHARGE_LEVEL_MAX: u16 = 130;
pub fn player_movement(keyboard_input: Res<Input<KeyCode>>,
                       mut move_er: EventReader<MoveHorizontallyAction>,
                       camq: Query<&Transform, With<Camera3d>>,
                       velq: Query<&Velocity, Without<Player>>,
                       mut transformsq: Query<&mut Transform, Without<Camera3d>>,
                       mut playerq: Query<(Entity,
                              &mut ExternalForce,
                              &mut ExternalImpulse,
                              &mut Velocity,
                              &Children,
                              &mut Player)>) {
  if let (Ok((player_entity,
              mut player_force,
              mut player_impulse,
              mut player_vel,
              player_children,
              mut player)),
          Ok(transform)) = (playerq.get_single_mut(), camq.get_single())
  {
    let player_max_speed = PLAYER_MAX_SPEED + player.speed_boost;

    // let entities_colliding_with_player =
    //   player_shape_hits.iter().map(|d: &ShapeHitData| d.entity);
    // if keyboard_input.just_released(KeyCode::K) {
    //   println!("colls:");
    //   entities_colliding_with_player.clone()
    //                                 .for_each(debug_println);
    // }
    // let linvels_of_entities_colliding_with_player =
    //   velq.iter_many(entities_colliding_with_player);
    // let avg_linvel_of_entities_colliding_with_player =
    //   avg(linvels_of_entities_colliding_with_player.map(|lv| lv.0));
    let avg_vel_of_entities_colliding_with_player = Some(Vec3::ZERO);
    // let player_colls = collisions.collisions_with_entity(player_entity);
    // let entities_colliding_with_player =
    //   player_colls.flat_map(|c: &Contacts| [c.entity1, c.entity2])
    //               .filter(|&e| e != player_entity);
    // let linvels_of_entities_colliding_with_player =
    //   linvelq.iter_many(entities_colliding_with_player);
    // let avg_linvel_of_entities_colliding_with_player =
    //   avg(linvels_of_entities_colliding_with_player.map(|lv| lv.0));
    // if grounded
    // let is_grounded = avg_linvel_of_entities_colliding_with_player.is_some();
    let is_grounded = true;
    if let Some(avgvel) = avg_vel_of_entities_colliding_with_player {
      let dir =
        [(KeyCode::D, Vec2::X),
         (KeyCode::A, Vec2::NEG_X),
         (KeyCode::W, Vec2::Y),
         (KeyCode::S, Vec2::NEG_Y)].into_iter()
                                   .filter_map(|(k, v)| {
                                     keyboard_input.pressed(k).then_some(v)
                                   })
                                   .sum::<Vec2>()
                                   .normalize_or_zero();
      let Vec2 { x, y } = dir;
      let right = transform.right().normalize();
      let forward = -(right.cross(Vec3::Y).normalize_or_zero());
      let relvel = player_vel.linvel - avgvel;
      let relspeed = relvel.length();
      player_force.force = 1.0 * {
        let desired_force = (right * x + forward * y) * PLAYER_WALK_FORCE;
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
      };
    }
    let charge_fraction =
      player.jump_charge_level.unwrap_or(0) as f32 / (PLAYER_JUMP_CHARGE_LEVEL_MAX as f32);
    player.jump_charge_level = if keyboard_input.just_released(KeyCode::Space) {
      // is_grounded
      if is_grounded {
        println!("jumped");
        player_impulse.impulse = Vec3::Y
                                 * 1.0
                                 * (PLAYER_MIN_JUMP_IMPULSE
                                    + ((PLAYER_MAX_JUMP_IMPULSE - PLAYER_MIN_JUMP_IMPULSE)
                                       * charge_fraction));
        // player_force.persistent = false;
        // player_force.apply_force(Vec3::Y
        //                          * 100.0
        //                          * (PLAYER_MIN_JUMP_IMPULSE
        //                             + ((PLAYER_MAX_JUMP_IMPULSE
        //                                 - PLAYER_MIN_JUMP_IMPULSE)
        //                                * charge_fraction)));
      }
      None
    } else if keyboard_input.just_pressed(KeyCode::Space) {
      Some(0)
    } else {
      player.jump_charge_level
            .map(|n| PLAYER_JUMP_CHARGE_LEVEL_MAX.min(n + 1))
    };
    if let Some(&ce) = player_children.first() {
      if let Ok(mut t) = transformsq.get_mut(ce) {
        t.scale.y = 1.0 - (charge_fraction * 0.3);
      }
    }
  }
}
pub fn sprites_face_camera(camq: Query<&GlobalTransform, With<Camera3d>>,
                           mut spriteq: Query<(&mut Transform, &GlobalTransform),
                                 (With<Sprite3dComponent>,
                                  Without<Camera3d>)>) {
  if let Ok(cam_globaltransform) = camq.get_single() {
    for (mut sprite_transform, sprite_globaltransform) in &mut spriteq {
      let dir = Vec3 { y: 0.0,
                       ..(sprite_globaltransform.translation()
                          - cam_globaltransform.translation()) };
      sprite_transform.look_to(dir, Vec3::Y);
    }
  }
}
pub fn gib_sprite_bundle(mut sprite_3d_params: Sprite3dParams,
                         mut c: Commands,
                         q: Query<(Entity, &GibSpriteBundle)>) {
  for (e, GibSpriteBundle(s)) in &q {
    if sprite_3d_params.images.contains(&s.image) {
      c.entity(e)
       .remove::<GibSpriteBundle>()
       .insert(Sprite3d { image: s.image.clone(),
                          ..*s }.bundle(&mut sprite_3d_params));
    }
  }
}
pub fn spawn_mushroom_man(playerq: Query<&Transform, With<Player>>,
                          keyboard_input: Res<Input<KeyCode>>,
                          mut c: Commands,
                          amah: Res<AllMyAssetHandles>) {
  if let Ok(&player_transform) = playerq.get_single() {
    if keyboard_input.just_pressed(KeyCode::Z) {
      spawn_with_child(&mut c,
                       (PlayerFollower,
                        Friction::new(3.9),
                        RigidBody::Dynamic,
                        Velocity::default(),
                        ExternalForce::default(),
                        ExternalImpulse::default(),
                        LockedAxes::ROTATION_LOCKED,
                        Collider::capsule_y(0.4, 0.2),
                        ColliderMassProperties::Mass(0.2),
                        SpatialBundle::from_transform(player_transform)),
                       GibSpriteBundle(Sprite3d { image: amah.mushroom_man.clone(),
                                                  pixels_per_metre: 23.0,
                                                  ..default() }))
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
const PICKUPDISTANCE: f32 = 0.6;
const SPEEDBOOSTAMOUNT: f32 = 5.0;
pub fn item_pick_up(mut playerq: Query<(&Transform, &mut Player)>,
                    itemsq: Query<(Entity, &Transform, &ItemPickUp)>,
                    mut c: Commands) {
  if let Ok((&player_transform, mut player)) = playerq.get_single_mut() {
    for (item, &item_transform, &item_pick_up) in &itemsq {
      if player_transform.translation
                         .distance(item_transform.translation)
         < PICKUPDISTANCE
      {
        c.entity(item).despawn();
        match item_pick_up {
          ItemPickUp::SpeedBoost => {
            player.speed_boost += SPEEDBOOSTAMOUNT;
          }
          ItemPickUp::HealthBoost(_) => todo!(),
        }
      }
    }
  }
}
