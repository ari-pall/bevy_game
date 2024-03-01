use {crate::assetstuff::AllMyAssetHandles,
     bevy::{pbr::{NotShadowCaster, NotShadowReceiver},
            prelude::*},
     bevy_vox_scene::VoxelSceneBundle,
     rust_utils::comment,
     std::f32::consts::PI};

pub trait BundleTree: Sized {
  fn spawn(self, c: &mut Commands) -> Entity;
  fn spawn_as_child(self, parent: Entity, c: &mut Commands) {
    let childe = self.spawn(c);
    c.entity(parent).add_child(childe);
  }
  fn with_child(self, child: impl BundleTree) -> impl BundleTree
    where Self: Sized {
    BundleTreeStruct(|c: &mut Commands| {
      let parente = self.spawn(c);
      let childe = child.spawn(c);
      c.entity(parente).add_child(childe);
      parente
    })
  }
}
impl<B: Bundle> BundleTree for B {
  fn spawn(self, c: &mut Commands) -> Entity { c.spawn(self).id() }
}
pub struct BundleTreeStruct<F: FnOnce(&mut Commands) -> Entity>(F);
impl<F: FnOnce(&mut Commands) -> Entity> BundleTree for BundleTreeStruct<F> {
  fn spawn(self, c: &mut Commands) -> Entity { self.0(c) }
}
