use {crate::{assetstuff::AllMyAssetHandles,
             components::{GibSpriteBundle, Player}},
     bevy::{math::vec3, prelude::*},
     bevy_rapier3d::prelude::*,
     bevy_sprite3d::Sprite3d,
     bevy_third_person_camera::{ThirdPersonCamera, ThirdPersonCameraTarget},
     rust_utils::{comment, map, mapv, vec},
     std::iter::{Cycle, Peekable}};

pub struct IceBerg;
pub struct Water;
#[derive(Component)]
pub struct SegmentPathMotion {
  pub destinations: Peekable<Cycle<std::vec::IntoIter<Vec3>>>,
  pub speed: f32
}
impl SegmentPathMotion {
  pub fn new(destinations: impl IntoIterator<Item = Vec3>, speed: f32) -> Self {
    Self { destinations: vec(destinations).into_iter().cycle().peekable(),
           speed }
  }
  pub fn circular_motion(center: Vec3, radius: f32, speed: f32) -> SegmentPathMotion {
    let num_points = 10;
    let destinations = map(|i| {
                             let angle =
                               2.0 * std::f32::consts::PI * (i as f32) / (num_points as f32);
                             let x = center.x + radius * angle.cos();
                             let z = center.z + radius * angle.sin();
                             vec3(x, center.y, z)
                           },
                           0..num_points);
    Self::new(destinations, speed)
  }
  pub fn dest(&mut self) -> Vec3 { *(self.destinations.peek().unwrap()) }
}
pub fn segment_path_motion(mut q: Query<(&mut Velocity,
                                  &Transform,
                                  &mut SegmentPathMotion)>) {
  for (mut vel, t, mut spm) in q.iter_mut() {
    if spm.dest().distance(t.translation) < spm.speed {
      spm.destinations.next();
    }
    vel.linvel = (spm.dest() - t.translation).normalize_or_zero() * spm.speed;
  }
}
