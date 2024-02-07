use {bevy::{asset::embedded_asset,
            gltf::{Gltf, GltfMesh},
            prelude::*,
            render::render_resource::{Extent3d, TextureDimension, TextureFormat}},
     rust_utils::{comment, map}};
// what is a scene? how do I generate a scene?
// fn generate_level() -> Scene { Scene }
/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
  const TEXTURE_SIZE: usize = 8;
  let mut palette: [u8; 32] = [255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255,
                               121, 255, 102, 255, 102, 255, 198, 255, 102, 198, 255, 255,
                               121, 102, 255, 255, 236, 102, 255, 255];
  let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
  for y in 0..TEXTURE_SIZE {
    let offset = TEXTURE_SIZE * y * 4;
    texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
    palette.rotate_right(4);
  }
  Image::new_fill(Extent3d { width: TEXTURE_SIZE as u32,
                             height: TEXTURE_SIZE as u32,
                             depth_or_array_layers: 1 },
                  TextureDimension::D2,
                  &texture_data,
                  TextureFormat::Rgba8UnormSrgb)
}
fn colorful_texture() -> Image {
  let texture_size = 8;
  Image::new_fill(Extent3d { width: texture_size,
                             height: texture_size,
                             depth_or_array_layers: 1 },
                  TextureDimension::D2,
                  map(|i| [255, 121, 236, 102, 159, 198][i % 6],
                      0..((texture_size * texture_size * 4) as usize)).collect::<Vec<u8>>()
                                                                      .as_slice(),
                  TextureFormat::Rgba8UnormSrgb)
}
#[derive(Resource, Default)]
pub struct AllMyAssetHandles {
  pub cube: Handle<Mesh>,
  pub unitcube: Handle<Mesh>,
  pub boxmesh: Handle<Mesh>,
  pub flatbox: Handle<Mesh>,
  pub capsule: Handle<Mesh>,
  pub torus: Handle<Mesh>,
  pub cylinder: Handle<Mesh>,
  pub icosphere: Handle<Mesh>,
  pub uvsphere: Handle<Mesh>,
  pub planesize50: Handle<Mesh>,
  pub particle_mesh: Handle<Mesh>,
  pub penguin_material: Handle<StandardMaterial>,
  pub particle_material: Handle<StandardMaterial>,
  pub funky_material: Handle<StandardMaterial>,
  pub colorful_material: Handle<StandardMaterial>,
  pub character_controller_demo_scene_gltf: Handle<Gltf>,
  pub wat: Handle<Gltf>,
  pub lunarlander: Handle<Scene>,
  pub character_controller_demo_scene: Handle<Scene>,
  pub level_scene: Handle<Scene>,
  pub island_level_scene: Handle<Scene>,
  pub some_sketch_level: Handle<Scene>,
  pub snowman: Handle<Scene>,
  pub funky_image: Handle<Image>,
  pub colorful_image: Handle<Image>,
  pub penguin_image: Handle<Image>,
  pub mushroom_man: Handle<Image>,
  pub tree: Handle<Image>,
  pub iceberg: Handle<Image>,
  pub stickman: Handle<Image>,
  pub coffee: Handle<Image>,
  pub snow_image: Handle<Image>,
  pub snow_material: Handle<StandardMaterial>,
  pub grass: Handle<Image>,
  pub grass_material: Handle<StandardMaterial>,
  pub stone: Handle<Image>,
  pub stone_material: Handle<StandardMaterial>,
  pub water: Handle<Image>,
  pub water_material: Handle<StandardMaterial>,
}
pub struct AssetStuffPlugin;
impl Plugin for AssetStuffPlugin {
  fn build(&self, app: &mut App) {
    let mut amah = AllMyAssetHandles::default();
    macro_rules! asset_paths {
      {$($name: ident, $path: expr)*} => {
        $(embedded_asset!(app, "src/", concat!("../assets/", $path));
          let $name = app.world.get_resource::<AssetServer>().unwrap()
                         .load(format!("embedded://bevy_game/../assets/{}",
                                                $path));
          amah.$name = $name.clone();)*}}
    macro_rules! glb_paths {
      {$($name: ident, $glb_path: expr, $path_within: expr)*} => {
        $(embedded_asset!(app, "src/", concat!("../assets/", $glb_path));
          let $name = app.world.get_resource::<AssetServer>().unwrap()
          .load(format!("embedded://bevy_game/../assets/{}#{}",
                                 $glb_path, $path_within));
          amah.$name = $name.clone();)*}}
    macro_rules! assets {
      {$($name: ident, $value: expr)*} => {
        $(let $name = app.world.get_resource_mut::<Assets<_>>().unwrap().add($value);
          amah.$name = $name.clone();)*}}
    glb_paths! {
      lunarlander, "lunarlander.glb", "Scene0"
      character_controller_demo_scene, "character_controller_demo.glb", "Scene0"
      level_scene, "level.glb", "Scene0"
      island_level_scene, "this_here_level.glb", "Scene0"
      some_sketch_level, "somesketchlevel.glb", "Scene0"
      snowman, "snowman.glb", "Scene0"
    }
    asset_paths! {
      stone, "stone.png"
      iceberg, "iceberg.png"
      coffee, "coffee.png"
      stickman, "stickman.png"
      grass, "grass.png"
      water, "water.png"
      tree, "tree.png"
      snow_image, "snow.png"
      penguin_image, "penguin.png"
      mushroom_man, "mushroom_man.png"
      wat, "wat.glb"
      character_controller_demo_scene_gltf, "character_controller_demo.glb"
    }
    assets! {
      unitcube, shape::Cube { size: 1.0 }.into()
      cube, shape::Cube { size: 0.7 }.into()
      boxmesh, shape::Box::default().into()
      flatbox, shape::Box::new(2.1,0.3,2.1).into()
      capsule, shape::Capsule::default().into()
      torus, shape::Torus::default().into()
      cylinder, shape::Cylinder::default().into()
      icosphere, shape::Icosphere::default().try_into().unwrap()
      uvsphere, shape::UVSphere::default().into()
      planesize50, shape::Plane::from_size(50.0).into()
      colorful_image, colorful_texture()
      colorful_material, StandardMaterial::from(colorful_image.clone())
      funky_image, uv_debug_texture()
      funky_material, StandardMaterial::from(funky_image.clone())
      water_material, StandardMaterial { perceptual_roughness:0.3,
                                         base_color: Color::SEA_GREEN,
                                         metallic:0.0,
                                         reflectance:0.5,
                                         ..water.clone().into()}
      snow_material, StandardMaterial { perceptual_roughness:0.4,
                                        base_color: Color::ALICE_BLUE,
                                        metallic:0.0,
                                        reflectance:0.5,
                                        ior: 1.31,
                                        ..StandardMaterial::from(snow_image.clone())}
      stone_material, StandardMaterial { perceptual_roughness:0.8,
                                         base_color: Color::GRAY,
                                         metallic:0.0,
                                         reflectance:0.3,
                                         ..StandardMaterial::from(stone.clone())}
      grass_material, StandardMaterial { perceptual_roughness:0.8,
                                         base_color: Color::GREEN,
                                         metallic:0.0,
                                         reflectance:0.2,
                                         ..StandardMaterial::from(grass.clone())}
      penguin_material, StandardMaterial::from(penguin_image.clone())
      particle_material,StandardMaterial::from(Color::rgb(0.2, 0.7, 0.9))
      particle_mesh, Mesh::try_from(shape::Icosphere { radius: 0.06 as f32,
                                                      ..default() }).unwrap()
    }
    app.insert_resource(amah)
       // .add_systems(Update, set_asset_handles)
      ;
  }
}
comment! {

  /// Helper resource for tracking our asset
  #[derive(Resource)]
  struct MyAssetPack(Handle<Gltf>);

  fn load_gltf(mut commands: Commands, ass: Res<AssetServer>) {
    let gltf = ass.load("lunarlander.gltf");
    commands.insert_resource(MyAssetPack(gltf));
  }
  fn spawn_gltf_objects(mut c: Commands, my: Res<MyAssetPack>, assets_gltf: Res<Assets<Gltf>>) {
    // if the GLTF has loaded, we can navigate its contents
    if let Some(gltf) = assets_gltf.get(&my.0) {
      // spawn the first scene in the file
      c.spawn(SceneBundle { scene: gltf.scenes[0].clone(),
                            ..default() });

      // spawn the scene named "YellowCar"
      c.spawn(SceneBundle { scene: gltf.named_scenes["YellowCar"].clone(),
                            transform: Transform::from_xyz(1.0, 2.0, 3.0),
                            ..default() });

      // PERF: the `.clone()`s are just for asset handles, don't worry :)
    }
  }
}
// #[derive(AssetCollection, Resource)]
// pub struct FontAssets {
//   #[asset(path = "fonts/FiraSans-Bold.ttf")]
//   pub fira_sans: Handle<Font>,
// }

// #[derive(AssetCollection, Resource)]
// pub struct AudioAssets {
//   #[asset(path = "audio/flying.ogg")]
//   pub flying: Handle<AudioSource>,
// }
