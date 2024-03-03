use {bevy::{asset::embedded_asset,
            gltf::Gltf,
            math::primitives,
            prelude::*,
            render::{render_asset::RenderAssetUsages,
                     render_resource::{Extent3d, TextureDimension, TextureFormat}}},
     bevy_vox_scene::VoxelScene,
     rust_utils::{comment, map}};
/// Creates a colorful test pattern
fn uv_debug_texture() -> Image {
  // DynamicSceneBuilder
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
                  TextureFormat::Rgba8UnormSrgb,
                  RenderAssetUsages::RENDER_WORLD)
}
fn colorful_texture() -> Image {
  let texture_size = 8;
  Image::new_fill(Extent3d { width: texture_size,
                             height: texture_size,
                             depth_or_array_layers: 1 },
                  TextureDimension::D2,
                  map(|_| rand::random(),
                      0..((texture_size * texture_size * 4) as usize)).collect::<Vec<u8>>()
                                                                      .as_slice(),
                  TextureFormat::Rgba8UnormSrgb,
                  RenderAssetUsages::RENDER_WORLD)
}

// pub const UNIT_SQUARE: OnceCell<Handle<Mesh>> = OnceCell::new();

pub const GLOWY_COLOR: Color = Color::rgb_linear(13.99, 11.32, 50.0);
pub const GLOWY_COLOR_2: Color = Color::rgb_linear(10.0, 0.3, 0.0);
pub const GLOWY_COLOR_3: Color = Color::rgb_linear(0.0, 30.0, 0.0);
#[derive(Resource, Default)]
pub struct AllMyAssetHandles {
  pub cube: Handle<Mesh>,
  pub unitcube: Handle<Mesh>,
  pub boxmesh: Handle<Mesh>,
  pub flatbox: Handle<Mesh>,
  pub capsule: Handle<Mesh>,
  pub unitsquare: Handle<Mesh>,
  pub torus: Handle<Mesh>,
  pub cylinder: Handle<Mesh>,
  pub sphere: Handle<Mesh>,
  pub planesize50: Handle<Mesh>,
  pub penguin_material: Handle<StandardMaterial>,
  pub particle_material: Handle<StandardMaterial>,
  pub funky_material: Handle<StandardMaterial>,
  pub glowy_material: Handle<StandardMaterial>,
  pub glowy_material_2: Handle<StandardMaterial>,
  pub glowy_material_3: Handle<StandardMaterial>,
  pub colorful_material: Handle<StandardMaterial>,
  pub character_controller_demo_scene_gltf: Handle<Gltf>,
  pub wat: Handle<Gltf>,
  pub lunarlander: Handle<Scene>,
  pub character_controller_demo_scene: Handle<Scene>,
  pub level_scene: Handle<Scene>,
  pub island_level_scene: Handle<Scene>,
  pub some_sketch_level: Handle<Scene>,
  pub alevel: Handle<Scene>,
  pub goxel_level: Handle<Scene>,
  pub turtle_level: Handle<Scene>,
  pub snowman: Handle<Scene>,
  pub funky_image: Handle<Image>,
  pub colorful_image: Handle<Image>,
  pub penguin_image: Handle<Image>,
  pub mushroom_man: Handle<Image>,
  pub tree: Handle<Image>,
  pub iceberg: Handle<Image>,
  pub stickman: Handle<Image>,
  pub skybox: Handle<Image>,
  pub sun: Handle<Image>,
  pub fire: Handle<Image>,
  pub coffee: Handle<Image>,
  pub coffee_scene: Handle<Scene>,
  pub flashlight: Handle<VoxelScene>,
  pub glowtest: Handle<VoxelScene>,
  pub flower: Handle<VoxelScene>,
  pub snow_image: Handle<Image>,
  pub snow_material: Handle<StandardMaterial>,
  pub grass: Handle<Image>,
  pub grass_material: Handle<StandardMaterial>,
  pub stone: Handle<Image>,
  pub stone_material: Handle<StandardMaterial>,
  pub water: Handle<Image>,
  pub water_material: Handle<StandardMaterial>
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
      alevel, "alevel.gltf", "Scene0"
      island_level_scene, "this_here_level.glb", "Scene0"
      some_sketch_level, "somesketchlevel.glb", "Scene0"
      snowman, "snowman.glb", "Scene0"
      coffee_scene, "coffee.glb", "Scene0"
      goxel_level, "goxel_level.glb", "Scene0"
      turtle_level, "turtle level.gltf", "Scene0"
      flashlight, "flashlight.vox", "flashlight"
    }
    asset_paths! {
      stone, "stone.png"
      skybox, "skybox.png"
      sun, "sun.png"
      fire, "fire.png"
      iceberg, "iceberg.png"
      coffee, "coffee.png"
      stickman, "stickman.png"
      flower, "flower.vox"
      glowtest, "glowtest.vox"
      grass, "grass.png"
      water, "water.png"
      tree, "tree.png"
      snow_image, "snow.png"
      penguin_image, "penguin.png"
      mushroom_man, "mushroom_man.png"
      wat, "wat.glb"
      character_controller_demo_scene_gltf, "character_controller_demo.glb"
    }

    StandardMaterial { unlit: true,
                       alpha_mode: AlphaMode::Mask(0.0),
                       ..GLOWY_COLOR_3.into() };
    let imgmat = |h: Handle<Image>| StandardMaterial { base_color_texture: Some(h),
                                                       ..default() };
    // let colmat = |color: Color| StandardMaterial::from(color);
    let colmat = StandardMaterial::from;

    // bevy::math::Rect{ min: vec2(-0.5,-0.5) , max: vec2(0.5,0.5) }
    // bevy::math::primitives::Rectangle::new(1.0,1.0)
    assets! {
      unitcube, primitives::Cuboid::new(1.0,1.0,1.0).mesh()
      cube, primitives::Cuboid::new(0.7,0.7,0.7).mesh()
      boxmesh, primitives::Cuboid::new(2.0,1.0,1.0).mesh()
      flatbox, primitives::Cuboid::new(2.1,0.3,2.1).mesh()
      capsule, primitives::Capsule3d::default().mesh()
      torus, primitives::Torus::default().mesh()
      sphere, primitives::Sphere{radius:1.0}.mesh()
      planesize50, primitives::Cuboid::new(25.0,0.1,25.0).mesh()
      unitsquare, primitives::Rectangle::new(1.0,1.0).mesh()
      colorful_image, colorful_texture()
      colorful_material, StandardMaterial::from(colorful_image.clone())
      funky_image, uv_debug_texture()
      funky_material, imgmat(funky_image.clone())
      glowy_material, StandardMaterial { unlit: true,
                                         alpha_mode: AlphaMode::Mask(0.0),
                                         ..GLOWY_COLOR.into() }
      glowy_material_2, StandardMaterial { unlit: true,
                                           alpha_mode: AlphaMode::Mask(0.0),
                                           ..colmat(GLOWY_COLOR_2).into() }
      glowy_material_3, StandardMaterial { unlit: true,
                                           alpha_mode: AlphaMode::Mask(0.0),
                                           ..colmat(GLOWY_COLOR_3).into() }
      water_material, StandardMaterial { perceptual_roughness:0.3,
                                         base_color: Color::SEA_GREEN,
                                         metallic:0.0,
                                         reflectance:0.5,
                                         ..imgmat(water.clone())}
      snow_material, StandardMaterial { perceptual_roughness:0.4,
                                        base_color: Color::ALICE_BLUE,
                                        metallic:0.0,
                                        reflectance:0.5,
                                        ior: 1.31,
                                        ..imgmat(snow_image.clone())}
      stone_material, StandardMaterial { perceptual_roughness:0.8,
                                         base_color: Color::GRAY,
                                         metallic:0.0,
                                         reflectance:0.3,
                                         ..imgmat(stone.clone())}
      grass_material, StandardMaterial { perceptual_roughness:0.8,
                                         base_color: Color::GREEN,
                                         metallic:0.0,
                                         reflectance:0.2,
                                         ..imgmat(grass.clone())}
      penguin_material, imgmat(penguin_image.clone())
      particle_material, colmat(Color::rgb(0.2, 0.7, 0.9))

    }

    // Sprite3d::bundle_with_atlas()
    // AtlasSprite3dBundle
    // let texture = fire.clone();
    // let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 12.0), 4, 1, None, None);
    // let atlas = TextureAtlas::texture_rect
    // let mut texture_atlas_layouts = app.world.resource_mut::<Assets<TextureAtlasLayout>>();
    // let layout_handle = texture_atlas_layouts.add(layout);
    // UNIT_SQUARE.set(unitsquare).unwrap();
    app.insert_resource(amah);
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
