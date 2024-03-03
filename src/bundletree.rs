use bevy::prelude::*;

pub struct BundleTreeStruct<F: FnOnce(&mut Commands) -> Entity>(F);

pub trait BundleTree: Sized {
  fn spawn(self, c: &mut Commands) -> Entity;
  fn spawn_as_child(self, parent: Entity, c: &mut Commands) {
    let childe = self.spawn(c);
    c.entity(parent).add_child(childe);
  }
  fn with_child(self, child: impl BundleTree) -> impl BundleTree {
    // let k: dyn BundleTree;
    // Box::new()
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
impl<F: FnOnce(&mut Commands) -> Entity> BundleTree for BundleTreeStruct<F> {
  fn spawn(self, c: &mut Commands) -> Entity { self.0(c) }
}

pub struct BundleTreeBox(Box<dyn FnOnce(&mut Commands) -> Entity>);
impl BundleTree for BundleTreeBox {
  fn spawn(self, c: &mut Commands) -> Entity { self.0(c) }
}
