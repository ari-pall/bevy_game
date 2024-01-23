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
pub fn sprites_face_camera(camq: Query<&GlobalTransform, With<Camera>>,
                           mut spriteq: Query<(&mut Transform, &GlobalTransform),
                                 (With<Sprite3dComponent>, Without<Camera>)>) {
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
    c.entity(e)
     .remove::<GibSpriteBundle>()
     .insert(Sprite3d { image: s.image.clone(),
                        ..*s }.bundle(&mut sprite_3d_params));
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
