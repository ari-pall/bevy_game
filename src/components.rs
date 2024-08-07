use std::time::Duration;

use crate::ui::UiPopup;
pub use bevy::prelude::Name;

use {bevy::{ecs::system::{SystemParam, SystemState},
            prelude::*,
            text::{Text, TextStyle},
            utils::{HashMap, HashSet}},
     bevy_mod_billboard::BillboardTextureBundle,
     rust_utils::{comment, inc, MutateTrait}};
#[derive(Component, Clone)]
pub struct Crafter;
#[derive(Component, Clone)]
pub struct Conveyor;
#[derive(Component, Clone)]
pub struct Char(pub char);
#[derive(Component, Clone)]
pub struct AttackPlayer;
#[derive(Component, Clone)]
pub struct DragonAttack;
#[derive(Component, Clone)]
pub struct RandomMovement;
#[derive(Component, Clone)]
pub struct EnemyMovement;
#[derive(Default, Clone)]
pub struct u32_cycle<const STEPS: u32>(u32);
impl<const STEPS: u32> u32_cycle<STEPS> {
  pub fn next(&mut self) { self.0 = (self.0 + 1) % STEPS; }
  pub fn fraction(&self) -> f32 { (self.0 as f32) / (STEPS as f32) }
}
#[derive(Default, Clone, Debug)]
pub struct u32_bounded<const MAX: u32>(u32);
impl<const MAX: u32> u32_bounded<MAX> {
  pub fn next(&mut self) { self.0 = (self.0 + 1).min(MAX); }
  pub fn fraction(&self) -> f32 { (self.0 as f32) / (MAX as f32) }
}
#[derive(Component, Clone)]
pub struct SpinningAnimation {
  pub rotation_steps: u32_cycle<430>,
  pub up_down_steps: u32_cycle<520>,
  pub up_down_distance: f32
}
#[derive(Component)]
pub struct FaceCamera;
const SUN_CYCLE_STEPS: u32 = 8000;
#[derive(Component, Default)]
pub struct Sun(pub u32_cycle<SUN_CYCLE_STEPS>);
#[derive(Component)]
pub struct SunSprite;
#[derive(Component, Clone)]
pub struct IsPlayerSprite;
pub const PLAYER_JUMP_CHARGE_LEVEL_MAX: u32 = 130;
#[derive(Component, Clone, Debug, Default)]
pub struct Player {
  // pub speed_boost: f32,
  pub num_coffee_cups: i32,
  // pub jump_charge_level: u16
  pub jump_charge_level: u32_bounded<PLAYER_JUMP_CHARGE_LEVEL_MAX>
}

#[derive(Component, Clone)]
pub struct Message {
  pub age_ticks: u32,
  pub origin_pos: Vec3
}
// pub fn message(text: impl Into<String>, origin_pos: Vec3) -> impl Bundle {
//   (Message{ age_ticks: 0, origin_pos },
//    BillboardTextBundle {
//      transform: Transform::from_scale(Vec3::splat(0.0)),
//      text: Text::from_section(text, TextStyle { font_size: 30.0,
//                                                 color: Color::YELLOW,
//                                                 ..default() }).with_justify(JustifyText::Center),
//      ..default()
//    })
// }
pub fn message(text: impl Into<String>, origin_pos: Vec3) -> impl Bundle {
  (SpatialBundle { transform: Transform::from_scale(Vec3::splat(0.0)),
                   ..default() },
   Message { age_ticks: 0,
             origin_pos },
   // FaceCamera,
   UiPopup::new([text]))
}
#[derive(Component, Clone, Copy)]
pub enum ItemPickUp {
  CoffeeCup,
  GetFlashLight,
  HealthBoost(u32)
}
#[derive(Component, Clone)]
pub enum Interact {
  GiveItem(Entity),
  AddMessage(String)
}
#[derive(Component)]
pub struct Combat {
  pub hp: u32,
  pub damage: u32
}

use {bevy_mod_billboard::BillboardTextBundle, rand::thread_rng};
#[derive(Component, Default)]
pub struct Container(pub HashSet<Entity>);
impl Container {
  pub fn empty() -> Container { Container::default() }
}

#[derive(Component, Hash, Eq, PartialEq, Default, Copy, Clone)]
pub struct Coord(pub [i32; 2]);
impl From<(i32, i32)> for Coord {
  fn from((a, b): (i32, i32)) -> Self { Self([a, b]) }
}
fn coord(x: i32, y: i32) -> Coord { Coord::from((x, y)) }

const ORIGIN: Coord = Coord([0, 0]);
// impl std::ops::Add<Dir> for Coord {
//   type Output = Self;
//   fn add(self, [a, b]: Dir) -> Self {
//     let x = Coord
//     Coord(match self.0 {
//             [x, y] => [a + x, b + y]
//           })
//   }
// }
pub fn name(s: &'static str) -> Name { Name::new(s) }
// #[derive(Component, Default)]
// pub struct Name(String);
#[derive(Component, Default)]
pub struct Tile {
  pub walkable: bool,
  pub color: Color
}
#[derive(Component)]
pub struct Fire {
  pub dir: (i8, i8)
}
#[derive(Component)]
pub struct TimedAnimation {
  pub num_frames: usize,
  pub time_per_frame_in_ticks: usize
}
#[derive(Component)]
pub struct PlayerFollower;

// use crate::gamething::Dir;

pub fn pick<T>(coll: impl IntoIterator<Item = T>) -> T {
  rand::seq::IteratorRandom::choose(coll.into_iter(), &mut thread_rng()).unwrap()
}
pub fn pick_multiple<T>(coll: impl IntoIterator<Item = T>, n: usize) -> Vec<T> {
  rand::seq::IteratorRandom::choose_multiple(coll.into_iter(), &mut thread_rng(), n)
}
pub enum Dir {
  NORTH,
  NORTHEAST,
  EAST,
  SOUTHEAST,
  SOUTH,
  SOUTHWEST,
  WEST,
  NORTHWEST,
  HERE
}
const NORTH: Dir = Dir::NORTH;
const NORTHEAST: Dir = Dir::NORTHEAST;
const EAST: Dir = Dir::EAST;
const SOUTHEAST: Dir = Dir::SOUTHEAST;
const SOUTH: Dir = Dir::SOUTH;
const SOUTHWEST: Dir = Dir::SOUTHWEST;
const WEST: Dir = Dir::WEST;
const NORTHWEST: Dir = Dir::NORTHWEST;
const HERE: Dir = Dir::HERE;
impl Dir {
  fn is_diagonal(&self) -> bool {
    matches!(self,
             Dir::NORTHEAST | Dir::SOUTHEAST | Dir::SOUTHWEST | Dir::NORTHWEST)
  }
  // fn is_diagonal(&self) -> bool {}
  fn random_lateral() -> Dir { pick([Dir::EAST, Dir::NORTH, Dir::WEST, Dir::SOUTH]) }
  fn random() -> Dir {
    pick([NORTH, NORTHEAST, EAST, SOUTHEAST, SOUTH, SOUTHWEST, WEST, NORTHWEST, HERE])
  }
  fn from_to(orig: &Coord, dest: &Coord) -> Self {
    let Coord([ox, _oy]) = orig;
    let Coord([dx, dy]) = dest;
    type O = std::cmp::Ordering;
    match (dx.cmp(ox), dy.cmp(ox)) {
      (O::Less, O::Less) => Dir::SOUTHWEST,
      (O::Less, O::Equal) => Dir::SOUTH,
      (O::Less, O::Greater) => Dir::NORTHEAST,
      (O::Equal, O::Less) => Dir::SOUTH,
      (O::Equal, O::Equal) => Dir::HERE,
      (O::Equal, O::Greater) => Dir::NORTH,
      (O::Greater, O::Less) => Dir::SOUTHEAST,
      (O::Greater, O::Equal) => Dir::EAST,
      (O::Greater, O::Greater) => Dir::NORTHEAST
    }
  }
}
impl From<Dir> for [i32; 2] {
  fn from(value: Dir) -> Self {
    match value {
      Dir::NORTH => [0, 1],
      Dir::NORTHEAST => [1, 1],
      Dir::EAST => [1, 0],
      Dir::SOUTHEAST => [1, -1],
      Dir::SOUTH => [0, -1],
      Dir::SOUTHWEST => [-1, -1],
      Dir::WEST => [-1, 0],
      Dir::NORTHWEST => [-1, 1],
      Dir::HERE => [0, 0]
    }
  }
}

impl std::ops::Add<Dir> for Coord {
  type Output = Self;
  fn add(self, rhs: Dir) -> Self::Output {
    let Self([x, y]) = self;
    let [rx, ry] = rhs.into();
    Self([x + rx, y + ry])
  }
}
impl std::ops::Sub<Dir> for Coord {
  type Output = Self;
  fn sub(self, rhs: Dir) -> Self::Output {
    let Self([x, y]) = self;
    let [rx, ry] = rhs.into();
    Self([x - rx, y - ry])
  }
}

pub struct MovingPlatform {
  pos1: Vec3,
  pos2: Vec3
}

comment! {
  #[derive(PartialEq, Eq)]
  enum Item {
    Loot,
    Wood,
    Fish,
    Coal,
    IronOre,
    IronIngot,
    Glass,
    Sand
  }
  #[derive(Component)]
  pub struct ItemStack(Item, u32);
  pub enum CraftingRecipe {
    SmeltIron,
    SmeltStone,
    SmeltSand
  }
  type Duration = u16;
  type I = Item;
  type D = Duration;
  type IN = (I, u16);
  struct CraftingRecipeProperties {
    inputs: Vec<IN>,
    outputs: Vec<IN>,
    duration: Duration
  }
  impl CraftingRecipe {
    fn properties(&self) -> CraftingRecipeProperties {
      type CR = CraftingRecipe;
      type CRP = CraftingRecipeProperties;
      let _one_to_one = |input: IN, output: IN, duration: D| CRP {
        inputs: vec![input],
        outputs: vec![output],
        duration
      };
      let two_to_one = |input1: IN, input2: IN, output: IN, duration: D| CRP {
        inputs: vec![input1, input2],
        outputs: vec![output],
        duration
      };
      match self {
        CR::SmeltIron => two_to_one((I::IronOre, 1), (I::Coal, 1), (I::IronIngot, 1), 10),
        CR::SmeltSand => two_to_one((I::Sand, 1), (I::Coal, 1), (I::Glass, 1), 10),
        CR::SmeltIron => todo!(),
        CR::SmeltStone => todo!(),
        CR::SmeltSand => todo!()
      }
    }
    fn duration(&self) -> Duration { self.properties().duration }
    fn inputs(&self) -> Vec<IN> { self.properties().inputs }
    fn outputs(&self) -> Vec<IN> { self.properties().outputs }
  }
  #[derive(Component)]
  pub struct CraftingMachine {
    active_recipe: CraftingRecipe,
    progress: u16
  }
  type CRM = CraftingMachine;
  fn simulate_crafting_machine(
    CRM {
      active_recipe,
      progress
    }: CRM
  ) -> CRM {
    if progress < active_recipe.duration() {
      CRM {
        active_recipe,
        progress: inc(progress)
      }
    } else {
      CRM {
        active_recipe,
        progress: 0
      }
    }
  }
  fn crafting_machines_system(mut q: Query<&mut CRM>) {
    for mut m in q.iter_mut() {
      m.as_mut().update(simulate_crafting_machine);
    }
  }
}
