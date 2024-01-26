use bevy_xpbd_3d::parry::{math::AngularInertia, na::givens::GivensRotation};

use crate::{assetstuff::AllMyAssetHandles, components::PlayerFollower};

use {crate::{components::{GibSpriteBundle, ItemPickUp, Player},
             input::*},
     bevy::prelude::*,
     bevy_sprite3d::{Sprite3d, Sprite3dComponent, Sprite3dParams},
     bevy_xpbd_3d::{math::*, prelude::*},
     rust_utils::{inc, vec, MutateTrait}};
fn avg<T: std::iter::Sum + std::ops::Div<f32, Output = T>>(coll: impl IntoIterator<Item = T>)
                                                           -> Option<T> {
  let v = vec(coll);
  let n = v.len();
  let s = v.into_iter().sum::<T>();
  (n != 0).then(|| s / (n as f32))
}
const PLAYER_WALK_FORCE: f32 = 20.0;
const PLAYER_MAX_SPEED: f32 = 9.0;
const PLAYER_JUMP_IMPULSE: f32 = 3.0;
pub fn player_movement(collisions: Res<Collisions>,
                       mut move_er: EventReader<MoveHorizontallyAction>,
                       mut jump_er: EventReader<JumpAction>,
                       camq: Query<&Transform, With<Camera3d>>,
                       linvelq: Query<&LinearVelocity>,
                       mut playerq: Query<(Entity,
                              &mut ExternalForce,
                              &mut ExternalImpulse,
                              &LinearVelocity,
                              &Player)>) {
  if let (Ok((player_entity,
              mut force,
              mut impulse,
              &LinearVelocity(linvel),
              Player { speed_boost })),
          Ok(transform)) = (playerq.get_single_mut(), camq.get_single())
  {
    let player_max_speed = PLAYER_MAX_SPEED + speed_boost;
    let player_colls = collisions.collisions_with_entity(player_entity);
    let entities_colliding_with_player =
      player_colls.flat_map(|c: &Contacts| [c.entity1, c.entity2])
                  .filter(|&e| e != player_entity);
    let linvels_of_entities_colliding_with_player =
      linvelq.iter_many(entities_colliding_with_player);
    let avg_linvel_of_entities_colliding_with_player =
      avg(linvels_of_entities_colliding_with_player.map(|lv| lv.0));
    // if grounded
    if let Some(avglv) = avg_linvel_of_entities_colliding_with_player {
      if jump_er.read().next().is_some() {
        impulse.apply_impulse(Vector::Y * PLAYER_JUMP_IMPULSE);
      }
      let right = transform.right().normalize();
      let forward = -(right.cross(Vec3::Y).normalize_or_zero());
      // let speed = linvel.length();
      let relvel = linvel - avglv;
      let relspeed = relvel.length();
      force.persistent = false;
      force.apply_force({
             if let Some(&MoveHorizontallyAction(Vec2 { x, y })) = move_er.read().next() {
               let desired_force = (right * x + forward * y) * PLAYER_WALK_FORCE;
               if relspeed < 0.1 {
                 desired_force
               } else {
                 let desired_parallel = desired_force.project_onto(relvel);
                 let desired_perpendicular = desired_force - desired_parallel;
                 let desired_parallel_bounded =
                   if desired_parallel.dot(relvel).is_sign_positive() {
                     desired_parallel * (1.0 - relspeed / player_max_speed)
                   } else {
                     desired_parallel
                   };
                 desired_parallel_bounded + desired_perpendicular

                 // desired_force
               }
             } else {
               Vec3::ZERO
               // -linvel * 6.7
             }
           });
    }
  }
}
pub fn sprites_face_camera(camq: Query<&GlobalTransform, With<Camera3d>>,
                           mut spriteq: Query<(&mut Transform, &GlobalTransform),
                                 (With<Sprite3dComponent>,
                                  Without<Camera3d>)>) {
  // AngularInertia
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
  if let Ok(player_transform) = playerq.get_single() {
    if keyboard_input.just_pressed(KeyCode::Z) {
      let collider = Collider::capsule(0.8, 0.2);
      c.spawn((RigidBody::Dynamic,
               PlayerFollower,
               MassPropertiesBundle::new_computed(&collider, 3.4),
               Friction::new(3.9),
               LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
               SpatialBundle::from_transform(*player_transform),
               collider,
               GibSpriteBundle(Sprite3d { image: amah.mushroom_man.clone(),
                                          transform: *player_transform,
                                          pixels_per_metre: 23.0,
                                          ..default() })));

      // spawn_with_child(&mut c,
      //                  (RigidBody::Dynamic,
      //                   MassPropertiesBundle::new_computed(&collider, 3.4),
      //                   collider),
      //                  GibSpriteBundle(Sprite3d { image: amah.mushroom_man.clone(),
      //                                             transform: *player_transform,
      //                                             pixels_per_metre: 12.0,
      //                                             ..default() }));
    }
  }
}
pub fn player_follower(mut followerq: Query<(&mut ExternalForce,
                              &Transform,
                              &LinearVelocity),
                             With<PlayerFollower>>,
                       mut playerq: Query<&Transform, With<Player>>) {
  if let Ok(player_transform) = playerq.get_single() {
    for (mut follower_force, follower_transform, follower_linvel) in &mut followerq {
      // let relpos = (player_transform.translation - follower_transform.translation);
      // let dist = relpos.length();
      follower_force.persistent = false;
      follower_force.apply_force((player_transform.translation
                                  - follower_transform.translation)
                                 * 0.8);
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
