use {crate::{assetstuff::AllMyAssetHandles,
             components::pick,
             setup::{cuboid_full_iter, level, sphere_full_iter}},
     bevy::{prelude::*, utils::HashMap},
     bevy_meshem::{prelude::{mesh_grid, VoxelMesh, *},
                   Dimensions, VoxelRegistry},
     bevy_rapier3d::geometry::AsyncCollider,
     num_enum::TryFromPrimitive,
     rust_utils::{comment, println},
     std::{fmt::Debug,
           mem::variant_count,
           ops::{IndexMut, Rem}}};

#[derive(Clone, Copy, PartialEq, Eq, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum BlockType {
  Air,
  Bricks,
  Grass,
  Rocks,
  Snow,
  Stone,
  Sand,
  Dirt
}
const NUM_BLOCK_TYPES: usize = variant_count::<BlockType>();
fn index_of_block_type(block_type: BlockType) -> u8 { block_type as u8 }
fn block_type_of_index(index: u8) -> BlockType { BlockType::try_from(index).unwrap() }
#[derive(Clone, Copy, PartialEq, Eq, Debug, TryFromPrimitive)]
#[repr(u8)]
pub enum BlockTexture {
  Bricks,
  Grass,
  Rocks,
  Snow,
  Stone,
  Sand,
  Dirt
}
const NUM_BLOCK_TEXTURES: usize = variant_count::<BlockTexture>();
fn index_of_texture(block_texture: BlockTexture) -> u8 { block_texture as u8 }
fn texture_of_index(index: u8) -> BlockTexture { BlockTexture::try_from(index).unwrap() }

pub fn voxel_mesh_all_same_texture(block_texture: BlockTexture) -> Mesh {
  let index = index_of_texture(block_texture);
  let coords = [index as u32, 0];
  generate_voxel_mesh([1.0, 1.0, 1.0],
                      [NUM_BLOCK_TEXTURES as u32, 1],
                      [(Top, coords),
                       (Bottom, coords),
                       (Right, coords),
                       (Left, coords),
                       (Back, coords),
                       (Forward, coords)],
                      [0.0, 0.0, 0.0],
                      0.0,
                      Some(0.5),
                      1.0)
}
fn array_range<const LEN: usize>() -> [usize; LEN] {
  let mut arr = [0; LEN];
  for i in 0..LEN {
    arr[i] = i;
  }
  arr
}
#[derive(Resource)]
pub struct MyVoxelRegistry {
  mesh_by_block_type_index: [Option<Mesh>; NUM_BLOCK_TYPES]
}
impl Default for MyVoxelRegistry {
  fn default() -> Self {
    Self { mesh_by_block_type_index: array_range().map(|i| {
                                                    let block_type =
                                                      block_type_of_index(i as u8);
                                                    match block_type{
                BlockType::Air => None,
                BlockType::Bricks => Some(voxel_mesh_all_same_texture(BlockTexture::Bricks)),
                BlockType::Grass => Some(voxel_mesh_all_same_texture(BlockTexture::Grass)),
                BlockType::Rocks => Some(voxel_mesh_all_same_texture(BlockTexture::Rocks)),
                BlockType::Snow => Some(voxel_mesh_all_same_texture(BlockTexture::Snow)),
                BlockType::Stone => Some(voxel_mesh_all_same_texture(BlockTexture::Stone)),
                BlockType::Sand => Some(voxel_mesh_all_same_texture(BlockTexture::Sand)),
                BlockType::Dirt => Some(voxel_mesh_all_same_texture(BlockTexture::Dirt)),
            }
                                                  }) }
  }
}
impl VoxelRegistry for MyVoxelRegistry {
  type Voxel = BlockType;

  fn get_mesh(&self, voxel: &Self::Voxel) -> VoxelMesh<&Mesh> {
    let om: Option<&Mesh> = self.mesh_by_block_type_index
                                .get(index_of_block_type(*voxel) as usize)
                                .unwrap()
                                .as_ref();
    match om {
      None => VoxelMesh::Null,
      Some(mesh) => VoxelMesh::NormalCube(mesh)
    }
  }
  fn is_covering(&self, voxel: &Self::Voxel, _side: bevy_meshem::prelude::Face) -> bool {
    match voxel {
      BlockType::Air => false,
      _ => true
    }
  }
  fn get_center(&self) -> [f32; 3] { [0.0; 3] }
  fn get_voxel_dimensions(&self) -> [f32; 3] { [1.0; 3] }
  fn all_attributes(&self) -> Vec<bevy::render::mesh::MeshVertexAttribute> {
    vec![Mesh::ATTRIBUTE_POSITION,
         Mesh::ATTRIBUTE_UV_0,
         Mesh::ATTRIBUTE_NORMAL,
         Mesh::ATTRIBUTE_COLOR]
  }
}

const CHUNK_SIDE_LENGTH: usize = 16;
const CHUNK_VOLUME: usize = CHUNK_SIDE_LENGTH.pow(3);
const MESHING_ALGORITHM: MeshingAlgorithm = bevy_meshem::prelude::MeshingAlgorithm::Culling;

#[derive(Component)]
struct Meshy {
  metadata: MeshMD<BlockType>,
  grid: [BlockType; CHUNK_VOLUME]
}

// fn floating_island()
// fn city_block([x,y,z]:[usize;3])->BlockType{

// }
fn prob(p: f32) -> bool { p > rand::random::<f32>() }
#[derive(Component)]
struct MeshInfo;
type Chunk = [BlockType; CHUNK_VOLUME];
const AIR_CHUNK: Chunk = [BlockType::Air; CHUNK_VOLUME];
fn spawn_blocks(chunks: &mut HashMap<IVec3, Chunk>,
                level: impl Iterator<Item = (IVec3, BlockType)>) {
  for (IVec3 { x, y, z }, block_type) in level {
    if block_type != BlockType::Air {
      let rem_euclid = |n: i32| n.rem_euclid(CHUNK_SIDE_LENGTH as i32) as usize;
      let div_euclid = |n: i32| n.div_euclid(CHUNK_SIDE_LENGTH as i32);
      let chunk_id = IVec3::new(div_euclid(x), div_euclid(y), div_euclid(z));
      let x_within = rem_euclid(x);
      let y_within = rem_euclid(y);
      let z_within = rem_euclid(z);
      let index_within =
        x_within + z_within * CHUNK_SIDE_LENGTH + y_within * (CHUNK_SIDE_LENGTH).pow(2);
      if chunks.get(&chunk_id) == None {
        chunks.insert(chunk_id, AIR_CHUNK);
      }
      chunks.get_mut(&chunk_id).unwrap()[index_within] = block_type;
    }
  }
}
pub fn voxels_init(mvr: Res<MyVoxelRegistry>,
                   mut c: Commands,
                   amah: Res<AllMyAssetHandles>,
                   mut meshes: ResMut<Assets<Mesh>>) {
  let mut chunks: HashMap<IVec3, Chunk> = HashMap::new();
  spawn_blocks(&mut chunks,
               level().map(|([x, y, z], tile)| {
                        (IVec3 { x: x as i32,
                                 y: y as i32,
                                 z: z as i32 },
                         match tile {
                           'g' => BlockType::Grass,
                           's' => BlockType::Snow,
                           'S' => BlockType::Bricks,
                           'k' => BlockType::Rocks,
                           'j' => BlockType::Stone,
                           _ => BlockType::Air
                         })
                      }));
  spawn_blocks(&mut chunks,
               sphere_full_iter(IVec3 { x: -50,
                                        y: 4,
                                        z: 50 },
                                30).map(|pos| {
                                     (pos,
                                      pick([BlockType::Grass,
                                            BlockType::Snow,
                                            BlockType::Bricks,
                                            BlockType::Rocks,
                                            BlockType::Stone]))
                                   }));
  let dims: Dimensions = (CHUNK_SIDE_LENGTH, CHUNK_SIDE_LENGTH, CHUNK_SIDE_LENGTH);
  // let smooth_lighting_params = Some(SmoothLightingParameters { intensity: 0.3,
  //                                                              max: 0.8,
  //                                                              smoothing: 1.1,
  //                                                              apply_at_gen: true });
  let smooth_lighting_params = None;

  for (chunk_id, chunk) in chunks {
    let chunk_translation = (chunk_id * (CHUNK_SIDE_LENGTH as i32)).as_vec3();
    let (culled_mesh, metadata) = mesh_grid(dims,
                                            &[],
                                            &chunk,
                                            mvr.as_ref(),
                                            MESHING_ALGORITHM,
                                            smooth_lighting_params).unwrap();
    let culled_mesh_handle: Handle<Mesh> = meshes.add(culled_mesh.clone());
    let meshy = Meshy { grid: chunk,
                        metadata };
    c.spawn((
      PbrBundle { mesh: culled_mesh_handle,
                  material: amah.blocks_material(),
                  transform: Transform::from_translation(chunk_translation),
                  // visibility: Visibility::Hidden,
                  ..default() },
      AsyncCollider(bevy_rapier3d::geometry::ComputedColliderShape::TriMesh) // meshy
    ));
  }
}

// ------------------------------------------------------------------------------------------------

comment! {
  #[allow(unused_imports, dead_code)]
  use bevy::pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin};
  use {bevy::prelude::*, bevy_meshem::prelude::*, rand::prelude::*, rust_utils::comment};

  /// Constants for us to use.
  const FACTOR: usize = 8;
  const CHUNK_LEN: usize = FACTOR * FACTOR * FACTOR;
  const SPEED: f32 = FACTOR as f32 * 2.0;

  fn main() {
    let mesh = voxel_mesh();
    App::new().add_plugins((DefaultPlugins,
                            WireframePlugin,
                            bevy_vox_scene::VoxScenePlugin,
                            assetstuff::AssetStuffPlugin))
              .insert_resource(BlockRegistry { grass: mesh.clone(),
                                               dirt: mesh })
              .insert_resource(AmbientLight { brightness: 400.0,
                                              color: Color::WHITE })
              .add_systems(Startup, setup)
    // .add_systems(Update,
    //              (input_handler, toggle_wireframe, input_handler_rotation, mesh_update))
              .add_event::<ToggleWireframe>()
      .add_event::<RegenerateMesh>()
      .run();
  }

  #[derive(Event, Default)]
  struct ToggleWireframe;

  #[derive(Event, Default)]
  struct RegenerateMesh;

  #[derive(Resource)]
  struct BlockRegistry {
    grass: Mesh,
    dirt: Mesh
  }

  /// The important part! Without implementing a [`VoxelRegistry`], you can't use the function.
  impl VoxelRegistry for BlockRegistry {
    /// The type of our Voxel, the example uses u16 for Simplicity but you may have a struct
    /// Block { Name: ..., etc ...}, and you'll define that as the type, but encoding the block
    /// data onto simple type like u16 or u64 is probably prefferable.
    type Voxel = u16;
    /// The get_mesh function, probably the most important function in the
    /// [`VoxelRegistry`], it is what allows us to  quickly access the Mesh of each Voxel.
    fn get_mesh(&self, voxel: &Self::Voxel) -> VoxelMesh<&Mesh> {
      match *voxel {
        0 => VoxelMesh::Null,
        1 => VoxelMesh::NormalCube(&self.dirt),
        2 => VoxelMesh::NormalCube(&self.grass),
        _ => VoxelMesh::Null
      }
    }
    /// Important function that tells our Algorithm if the Voxel is "full", for example, the Air
    /// in minecraft is not "full", but it is still on the chunk data, to singal there is nothing.
    fn is_covering(&self, voxel: &u16, _side: Face) -> bool { *voxel != 0 }
    /// The center of the Mesh, out mesh is defined in src/voxel_mesh.rs, just a constant.
    fn get_center(&self) -> [f32; 3] { [0.0, 0.0, 0.0] }
    /// The dimensions of the Mesh, out mesh is defined in src/voxel_mesh.rs, just a constant.
    fn get_voxel_dimensions(&self) -> [f32; 3] { [1.0, 1.0, 1.0] }
    /// The attributes we want to take from out voxels, note that using a lot of different
    /// attributes will likely lead to performance problems and unpredictible behaviour.
    /// We chose these 3 because they are very common, the algorithm does preserve UV data.
    fn all_attributes(&self) -> Vec<bevy::render::mesh::MeshVertexAttribute> {
      vec![Mesh::ATTRIBUTE_POSITION,
           Mesh::ATTRIBUTE_UV_0,
           Mesh::ATTRIBUTE_NORMAL]
    }
  }

  /// Setting up everything to showcase the mesh.
  fn setup(breg: Res<BlockRegistry>,
           mut c: Commands,
           mut materials: ResMut<Assets<StandardMaterial>>,
           // wireframe_config: ResMut<WireframeConfig>,
           mut meshes: ResMut<Assets<Mesh>>,
           amah: Res<AllMyAssetHandles>) {
    let gridthing: Vec<_> =
      vec![1u16; CHUNK_LEN].iter()
                           .enumerate()
                           .map(|(i, x)| {
                             if i >= FACTOR * FACTOR * FACTOR - FACTOR * FACTOR {
                               2
                             } else {
                               *x
                             }
                           })
                           .collect();
    let grid: [u16; CHUNK_LEN] = gridthing.try_into().unwrap();
    let dims: Dimensions = (FACTOR, FACTOR, FACTOR);
    let texture_handle = amah.penguin_image.clone();

    let (culled_mesh, metadata) = mesh_grid(dims,
                                            &[],
                                            &grid,
                                            breg.into_inner(),
                                            MeshingAlgorithm::Culling,
                                            None).unwrap();
    let culled_mesh_handle: Handle<Mesh> = meshes.add(culled_mesh.clone());
    c.spawn((
      PbrBundle {
        mesh: culled_mesh_handle,
        material: materials.add(StandardMaterial {
          // base_color: Color::LIME_GREEN,
          // alpha_mode: AlphaMode::Mask(0.5),
          base_color_texture: Some(texture_handle),
          ..default()
        }),
        ..default()
      },
      Meshy {
        meta: metadata,
        grid,
      },
    ));

    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_and_light_transform =
      Transform::from_xyz(FACTOR as f32 * 1.7,
                          FACTOR as f32 * 1.7,
                          FACTOR as f32 * 1.7).looking_at(Vec3::new(FACTOR as f32 * 0.5,
                                                                    FACTOR as f32 * 0.5,
                                                                    FACTOR as f32 * 0.5),
                                                          Vec3::Y);

    // Camera in 3D space.
    c.spawn(Camera3dBundle { transform: camera_and_light_transform,
                             ..default() });

    // Light up the scene.
    c.spawn(PointLightBundle { point_light: PointLight { intensity: 7000.0,
                                                         range: 1000.0,
                                                         ..default() },
                               transform: camera_and_light_transform,
                               ..default() });
    // for (att, _val) in culled_mesh.attributes() {
    //     // dbg!(att);
    //     if att == Mesh::ATTRIBUTE_POSITION.id {}
    // }
    c.spawn(
      TextBundle::from_section(
        format!(
          "X/Y/Z: Rotate\nR: Reset orientation\nMove Camera: W/A/S/D/Left-Shift/Space\nToggle Wireframe: T\n"),
        TextStyle {
          font_size: 26.0,
          color: Color::LIME_GREEN,
          ..default()
        },
      )
        .with_style(Style {
          position_type: PositionType::Absolute,
          top: Val::Px(12.0),
          left: Val::Px(12.0),
          ..default()
        }),
    );
    c.spawn((
      MeshInfo,
      TextBundle::from_section(
        format!("Press -C- To Break / Add a random voxel\n",),
        TextStyle {
          font_size: 26.0,
          color: Color::LIME_GREEN,
          ..default()
        },
      )
        .with_style(Style {
          position_type: PositionType::Absolute,
          bottom: Val::Px(12.0),
          left: Val::Px(12.0),
          ..default()
        }),
    ));
  }

  /// System to add or break random voxels.
  fn mesh_update(mut meshy: Query<&mut Meshy>,
                 breg: Res<BlockRegistry>,
                 mut meshes: ResMut<Assets<Mesh>>,
                 mesh_query: Query<&Handle<Mesh>>,
                 mut event_reader: EventReader<RegenerateMesh>) {
    for _ in event_reader.read() {
      let mesh = meshes.get_mut(mesh_query.get_single().unwrap())
                       .expect("Couldn't get a mut ref to the mesh");

      let m = meshy.get_single_mut().unwrap().into_inner();
      let mut rng = rand::thread_rng();
      let choice = m.grid.iter().enumerate().choose(&mut rng).unwrap();
      let neighbors: [Option<u16>; 6] = {
        let mut r = [None; 6];
        for i in 0..6 {
          match get_neighbor(choice.0, Face::from(i), m.meta.dims) {
            None => {}
            Some(j) => r[i] = Some(m.grid[j])
          }
        }
        r
      };
      match choice {
        (i, 1) => {
          m.meta.log(VoxelChange::Broken, i, 1, neighbors);
          update_mesh(mesh, &mut m.meta, breg.into_inner());
          m.grid[i] = 0;
        }
        (i, 0) => {
          m.meta.log(VoxelChange::Added, i, 1, neighbors);
          update_mesh(mesh, &mut m.meta, breg.into_inner());
          m.grid[i] = 1;
        }
        _ => {}
      }
      break;
    }
  }

}
