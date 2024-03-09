use {crate::{assetstuff::{AllMyAssetHandles, GLOWY_COLOR, GLOWY_COLOR_2, GLOWY_COLOR_3},
             bundletree::BundleTree,
             components::{name, FaceCamera, IsPlayerSprite, ItemPickUp, Player,
                          SpinningAnimation, Sun, TimedAnimation},
             jumpy_penguin::SegmentPathMotion,
             ui::{ui_pop_up, UiPopup},
             update::{capsule_from_height_and_radius, AnimatedBillboard, Billboard,
                      FaceCameraDir, PLAYER_HEIGHT, PLAYER_RADIUS},
             voxels::BlockType},
     bevy::{core_pipeline::{self,
                            bloom::{BloomCompositeMode, BloomPrefilterSettings,
                                    BloomSettings}},
            math::vec3,
            pbr::{NotShadowCaster, NotShadowReceiver},
            prelude::*,
            render::camera::{Exposure, RenderTarget},
            utils::hashbrown::HashSet},
     bevy_mod_billboard::BillboardTextBundle,
     bevy_rapier3d::prelude::*,
     bevy_third_person_camera::{Offset, ThirdPersonCamera, ThirdPersonCameraTarget},
     bevy_vox_scene::VoxelSceneBundle,
     rust_utils::comment,
     std::f32::consts::PI};

pub fn translate(it: impl Iterator<Item = IVec3>,
                 translation: IVec3)
                 -> impl Iterator<Item = IVec3> {
  it.map(move |v| v + translation)
}
pub fn cuboid_full_iter(lower_corner: IVec3,
                        side_lengths: IVec3)
                        -> impl Iterator<Item = IVec3> {
  let mut v = Vec::new();
  for x in 0..side_lengths.x {
    for y in 0..side_lengths.y {
      for z in 0..side_lengths.z {
        v.push(lower_corner + IVec3 { x, y, z })
      }
    }
  }
  v.into_iter()
}
pub fn sphere_full_iter(center: IVec3, radius: i32) -> impl Iterator<Item = IVec3> {
  cuboid_full_iter(center - IVec3::splat(radius),IVec3::splat(radius * 2)).filter(move |v: &IVec3| v.distance_squared(center) <= radius.pow(2))
}
comment! {
  pub fn spheroid_full_iter(center: IVec3, xrad: i32, yrad: i32, zrad: i32) -> impl Iterator<Item = IVec3> {
    let rads =IVec3::new(xrad,yrad,zrad);
    cuboid_full_iter(-rads,2 * rads).filter(move |v: &IVec3| {
      v.as_vec3();
      v.dot_into_vec <= radius.pow(2)})
  }
  pub fn floating_island(center: IVec3) -> impl Iterator<Item = (IVec3, BlockType)> {
    let island_radius = 10;
    sphere_full_iter(center,)
    // let sphere_center = IVec3::Y * radius;
      let beach_length = 2;
    cuboid_full_iter(translation, IVec3 { x: island_radius * 2,
                                          y: todo!(),
                                          z: island_radius * 2 }).map(|v: IVec3| v);
  }
}
// pub fn cuboid_corners_and_edges_iter(lower_corner: IVec3,
//                                      side_lengths: IVec3)
//                                      -> impl Iterator<Item = IVec3> {
//   translate(
//     1..(side_lengths.x - 2)
//       ,jj
//   )
// }
pub fn cuboid_surface_iter(lower_corner: IVec3,
                           side_lengths: IVec3)
                           -> impl Iterator<Item = IVec3> {
  let full: HashSet<IVec3> = cuboid_full_iter(lower_corner, side_lengths).collect();
  let interior: HashSet<IVec3> =
    cuboid_full_iter(lower_corner + IVec3::ONE, side_lengths - IVec3::splat(2)).collect();
  let diff: Vec<_> = full.difference(&interior).copied().collect();
  diff.into_iter()
  // diff.into_iter().copied()
}

pub fn level() -> impl Iterator<Item = ([usize; 3], char)> {
  [include_str!("level1.txt"),
   include_str!("level2.txt"),
   include_str!("level3.txt"),
   include_str!("level4.txt")].into_iter()
                              .enumerate()
                              .flat_map(|(y, floor)| {
                                floor.lines().enumerate().flat_map(move |(z, line)| {
                                                           line.char_indices()
                                                               .map(move |(x, tile)| {
                                                                 ([x, y, z], tile)
                                                               })
                                                         })
                              })
}
pub const ENABLE_SHADOWS_OTHER_THAN_SUN: bool = false;
pub const AMBIENT_LIGHT: AmbientLight = AmbientLight { color: Color::WHITE,
                                                       brightness: 300.0 };
pub const BLOOM_SETTINGS: BloomSettings =
  BloomSettings { intensity: 0.5,
                  low_frequency_boost: 0.0,
                  prefilter_settings: BloomPrefilterSettings { threshold: 2.2,
                                                               threshold_softness: 0.0 },
                  composite_mode: BloomCompositeMode::Additive,
                  ..BloomSettings::NATURAL };
pub enum Intensity {
  Low,
  Med,
  High
}
pub fn point_light(translation: Vec3,
                   radius: f32,
                   intensity: Intensity,
                   color: Color)
                   -> PointLightBundle {
  PointLightBundle { transform: Transform::from_translation(translation),
                     point_light: PointLight { intensity: match intensity {
                                                 Intensity::Low => 50_000.0,
                                                 Intensity::Med => 1_00_000.0,
                                                 Intensity::High => 4_00_000.0
                                               },
                                               radius,
                                               range: 5.0,
                                               shadows_enabled:
                                                 ENABLE_SHADOWS_OTHER_THAN_SUN,
                                               color,
                                               ..default() },
                     ..default() }
}
pub fn billboard(transform: Transform, image_handle: Handle<Image>) -> Billboard {
  Billboard { transform,
              image_handle,
              unlit: false }
}

pub fn flashlight(transform: Transform, amah: &Res<AllMyAssetHandles>) -> impl BundleTree {
  (VoxelSceneBundle { scene: amah.flashlight.clone(),
                      transform,
                      ..default() },
   NotShadowCaster,
   NotShadowReceiver)
                     .with_child(SpotLightBundle { spot_light:
                                                     SpotLight { shadows_enabled: ENABLE_SHADOWS_OTHER_THAN_SUN,
                                                                 intensity: 2_000_000.0,
                                                                 range: 5.0,
                                                                 outer_angle: PI * 0.2,
                                                                 ..default() },
                                                   transform:
                                                     Transform::from_translation(Vec3::NEG_Z
                                                                                 * 3.0),
                                                   ..default() })
}
pub const FIRE_ANIM_LENGTH: usize = 4;
pub const TORCH_ANIM_LENGTH: usize = 4;
pub fn setup(mut c: Commands, amah: Res<AllMyAssetHandles>) {
  macro_rules! spawn {
    ($bundle:expr) => {{
      c.spawn($bundle);
    }};
    ($bundle1:expr,$bundle2:expr) => {{
      $bundle1.with_child($bundle2).spawn(&mut c);
    }};
    ($bundle1:expr,$bundle2:expr,$bundle3:expr) => {{
      $bundle1.with_child($bundle2)
              .with_child($bundle3)
              .spawn(&mut c);
    }};
  }

  spawn!((FaceCamera,
          SpatialBundle { transform:
                            Transform::from_xyz(13.0, 2.0, 5.0).with_scale(Vec3::splat(0.03)),
                          ..default() },
          UiPopup::new(["some", "text"])));
  spawn!((FaceCameraDir,
          SpatialBundle { transform:
                            Transform::from_xyz(17.0, 2.5, 5.0).with_scale(Vec3::splat(0.02)),
                          ..default() },
          UiPopup::new(["╔════╗",
                        "║some║",
                        "║    ║",
                        "║téxt║",
                        "║    ║",
                        "║here║",
                        "╚════╝",])));
  let text_style = TextStyle { font_size: 30.0,
                               ..default() };
  spawn!(TextBundle::from(TextSection::new("z: ".to_string(), text_style.clone())));
  // DefaultUiCamera
  spawn!(ImageBundle { style: Style { width: Val::Percent(5.0),
                                      height: Val::Percent(7.0),
                                      ..default() },
                       image: UiImage::from(amah.mushroom_man.clone()),
                       ..default() });
  // RenderTarget
  // TargetCamera
  let col_text = |color: Color, text: &str| TextSection { value: text.to_string(),
                                                          style:
                                                            TextStyle { font_size: 30.0,
                                                                        color,
                                                                        ..default() } };
  spawn!(BillboardTextBundle {
    transform: Transform::from_xyz(23.128942, 3.8398309, 3.602163)
      .with_scale(Vec3::splat(0.0085)),
    text: Text::from_sections([
      col_text(Color::RED,"IMPORTANT "),
      col_text(Color::BLUE,"text")
    ]).with_justify(JustifyText::Center),
    ..default()
  });
  let iceberg = |mut spm: SegmentPathMotion| {
    (RigidBody::KinematicVelocityBased,
     Velocity::default(),
     // Velocity::angular(Vec3::Y * ((rand::random::<f32>() - 0.5) * 0.1)),
     Friction::default(),
     AsyncCollider(ComputedColliderShape::ConvexHull),
     PbrBundle { mesh: amah.flatbox.clone(),
                 material: amah.snow_material.clone(),
                 transform: Transform::from_translation(spm.dest()),
                 ..default() },
     spm)
  };
  spawn!(iceberg(SegmentPathMotion::new([vec3(0.0, -6.0, 0.0)], 1.3)));
  let circle_iceberg = |center: Vec3, radius: f32, speed: f32| {
    iceberg(SegmentPathMotion::circular_motion(center, radius, speed))
  };
  // spawn!(iceberg(vec3(0.0, -6.0, 0.0), 1.3, amah.as_ref()));
  spawn!(circle_iceberg(vec3(0.0, -6.0, 0.0), 12.0, 1.3));
  spawn!(circle_iceberg(vec3(0.0, -6.0, 0.0), 9.0, 1.3));
  spawn!(circle_iceberg(vec3(0.0, -6.0, 0.0), 6.0, 1.3));
  // spawn!(circle_iceberg(vec3(0.0, -6.0, 0.0), 0.0, 1.3));

  let up_down_iceberg = |origin: Vec3, height: f32, speed: f32| {
    iceberg(SegmentPathMotion::new([origin, origin + Vec3::Y * height], speed))
  };
  spawn!(up_down_iceberg(vec3(5.343936, -3.0, -2.4758048), 4.0, 0.5));
  spawn!(up_down_iceberg(vec3(9.069067, -4.0, -5.0675673), 4.0, 0.4));
  spawn!(up_down_iceberg(vec3(12.84221, -6.0, -4.947112), 4.0, 0.3));
  spawn!((FaceCamera,
          billboard(Transform::from_xyz(-30.0, 0.0, -40.0).with_scale(Vec3::splat(7.0)),
                    amah.iceberg.clone())));
  spawn!((RigidBody::Fixed,
          AsyncCollider(ComputedColliderShape::ConvexHull),
          PbrBundle { mesh: amah.planesize50.clone(),
                      material: amah.water_material.clone(),
                      transform: Transform::from_xyz(0.0, -6.0, 0.0),
                      ..default() }));
  spawn!((RigidBody::Fixed,
          AsyncSceneCollider { shape: Some(ComputedColliderShape::TriMesh),
                               named_shapes: default() },
          SceneBundle { scene: amah.island_level_scene.clone(),
                        transform:
                          Transform::from_xyz(10.0, -30.0, -10.0).with_scale(Vec3::ONE
                                                                             * 20.0),
                        ..default() }));
  spawn!((RigidBody::Fixed,
          Friction::new(0.1),
          AsyncSceneCollider { shape: Some(ComputedColliderShape::TriMesh),
                               named_shapes: default() },
          SceneBundle { scene: amah.some_sketch_level.clone(),
                        transform:
                          Transform::from_xyz(-30.0, -30.0, 30.0).with_scale(Vec3::ONE
                                                                             * 20.0),
                        ..default() }));
  spawn!((RigidBody::Fixed,
          Friction::new(0.1),
          AsyncSceneCollider { shape: Some(ComputedColliderShape::TriMesh),
                               named_shapes: default() },
          SceneBundle { scene: amah.turtle_level.clone(),
                        transform: Transform::from_xyz(40.0, -10.0, -40.0),
                        ..default() }));
  let glowy_sphere = |translation| {
    (PbrBundle { mesh: amah.sphere.clone(),
                 material: amah.glowy_material.clone(),
                 transform: Transform::from_translation(translation),
                 ..default() },
                 NotShadowCaster,
                 NotShadowReceiver,
                 RigidBody::Fixed,
                 Friction::default(),
                 Velocity::default(),
                 AsyncCollider(ComputedColliderShape::ConvexHull))
    .with_child(point_light(Vec3::ZERO,1.0,Intensity::High,GLOWY_COLOR))
  };
  glowy_sphere(vec3(22.709263, -26.007673, 72.32278)).spawn(&mut c);
  spawn!((RigidBody::Fixed,
          // Friction::new(0.1),
          AsyncSceneCollider { shape: Some(ComputedColliderShape::TriMesh),
                               named_shapes: default() },
          SceneBundle { scene: amah.alevel.clone(),
                        transform: Transform::from_xyz(40.0, -30.0, 60.0),
                        ..default() }));

  spawn!((Sun::default(),
          Billboard { transform: Transform::from_scale(Vec3::splat(20.0)),
                      image_handle: amah.sun.clone(),
                      unlit: true },
          NotShadowCaster,
          NotShadowReceiver),
         DirectionalLightBundle { directional_light:
                                    DirectionalLight { color: Color::WHITE,
                                                       illuminance: 11000.0,
                                                       shadows_enabled: true,
                                                       ..default()
                                                       // shadow_depth_bias: todo!(),
                                                       // shadow_normal_bias: todo!()
                                    },

                                  // Self {
                                  //     num_cascades: 1,
                                  //     minimum_distance: 0.1,
                                  //     maximum_distance: 100.0,
                                  //     first_cascade_far_bound: 5.0,
                                  //     overlap_proportion: 0.2,
                                  // }
                                  // cascade_shadow_config: CascadeShadowConfig
                                  transform: Transform::from_scale(Vec3::NEG_ONE),
                                  ..default() });

  // ScreenSpaceAmbientOcclusionPlugin
  // Camera
  spawn!((Camera3dBundle { camera: Camera { hdr: true,
                                            ..default() },
                           exposure: Exposure { ev100: 10.0 },
                           tonemapping:
                             core_pipeline::tonemapping::Tonemapping::Reinhard,
                           ..default() },
          IsDefaultUiCamera,
          BLOOM_SETTINGS,
          // Skybox(amah.skybox.clone()),
          ThirdPersonCamera { cursor_lock_key: KeyCode::Tab,
                              cursor_lock_toggle_enabled: true,
                              cursor_lock_active: false,
                              offset_enabled: true,
                              offset: Offset::new(0.0, 0.5),
                              mouse_sensitivity: 1.7,
                              zoom: bevy_third_person_camera::Zoom::new(1.2, 13.0),
                              zoom_sensitivity: 0.1,
                              ..default() }));
  for ([x, y, z], tile) in level() {
    let translation = Vec3::from_slice(&[x, y, z].map(|n| n as f32));
    let transform = Transform::from_translation(translation);
    match tile {
      't' => {
        let height = 4.0;
        let radius = 0.5;

        spawn!((RigidBody::Fixed,
                capsule_from_height_and_radius(height, radius),
                SpatialBundle::from_transform(Transform::from_translation(translation
                                                                          - 0.5
                                                                          + (height
                                                                             / 2.0))),
                FaceCamera),
               billboard(Transform::from_scale(Vec3::splat(height)),
                         amah.tree.clone()))
      }
      'f' => {
        spawn!((FaceCamera,
                NotShadowCaster,
                NotShadowReceiver,
                TimedAnimation { num_frames: FIRE_ANIM_LENGTH,
                                 time_per_frame_in_ticks: 20 },
                AnimatedBillboard { transform,
                                    image_handle: amah.fire.clone(),
                                    unlit: true,
                                    num_frames: FIRE_ANIM_LENGTH }),
               point_light(Vec3::ZERO, 0.4, Intensity::Med, GLOWY_COLOR_3))
      }
      'T' => {
        spawn!((FaceCamera,
                NotShadowCaster,
                NotShadowReceiver,
                TimedAnimation { num_frames: TORCH_ANIM_LENGTH,
                                 time_per_frame_in_ticks: 20 },
                AnimatedBillboard { transform,
                                    image_handle: amah.torch.clone(),
                                    unlit: true,
                                    num_frames: TORCH_ANIM_LENGTH }),
               point_light(Vec3::ZERO, 0.4, Intensity::Med, GLOWY_COLOR_2))
      }
      'C' => spawn!((ItemPickUp::SpeedBoost,
                     SceneBundle { transform,
                                   ..default() }),
                    (SceneBundle { scene: amah.coffee_scene.clone(),
                                   transform: Transform::from_scale(Vec3::splat(0.1)),
                                   ..default() },
                     SpinningAnimation { rotation_steps: default(),
                                         up_down_steps: default(),
                                         up_down_distance: 0.3 })),
      'F' => {
        (ItemPickUp::GetFlashLight,
         SceneBundle { transform,
                       ..default() })
          .with_child((SceneBundle::default(),
                       SpinningAnimation { rotation_steps: default(),
                                           up_down_steps: default(),
                                           up_down_distance: 0.2 })
                      .with_child(flashlight(Transform::from_scale(Vec3::splat(0.08)),
                                             &amah)))

          .spawn(&mut c);
      }
      'H' => {
        SceneBundle { transform,
                      ..default() }
        .with_child((VoxelSceneBundle { scene: amah.glowtest.clone(),
                                        transform: Transform::from_scale(Vec3::splat(0.1)),
                                        ..default() },
                     SpinningAnimation { rotation_steps: default(),
                                         up_down_steps: default(),
                                         up_down_distance: 0.2 }))
        .spawn(&mut c);
      }
      'L' => spawn!((RigidBody::Dynamic,
                     ColliderMassProperties::Density(1.0),
                     // MassPropertiesBundle::default(),
                     AsyncSceneCollider { shape:
                                            Some(ComputedColliderShape::ConvexHull),
                                          named_shapes: default() },
                     SceneBundle { scene: amah.lunarlander.clone(),
                                   transform,
                                   ..default() })),
      'c' => spawn!((RigidBody::Dynamic,
                     ColliderMassProperties::Density(1.0),
                     // MassPropertiesBundle::default(),
                     AsyncCollider(ComputedColliderShape::ConvexHull),
                     PbrBundle { mesh: amah.cube.clone(),
                                 material: amah.colorful_material.clone(),
                                 transform,
                                 ..default() })),
      'y' => {
        spawn!((RigidBody::Dynamic,
                ColliderMassProperties::Density(1.0),
                // NotShadowCaster,
                // MassPropertiesBundle::default(),
                AsyncCollider(ComputedColliderShape::ConvexHull),
                Restitution { coefficient: 0.0,
                              combine_rule: CoefficientCombineRule::Multiply },
                // Friction { coefficient: 0.2,
                //            combine_rule: CoefficientCombineRule::Multiply },
                PbrBundle { mesh: amah.sphere.clone(),
                            material: amah.snow_material.clone(),
                            transform: transform.with_scale(Vec3::splat(0.4)),
                            ..default() }))
      }
      'b' => {
        spawn!((RigidBody::Dynamic,
                Restitution { coefficient: 0.97,
                              combine_rule: CoefficientCombineRule::Max },
                ColliderMassProperties::Density(1.0),
                NotShadowCaster,
                // MassPropertiesBundle::default(),
                AsyncCollider(ComputedColliderShape::ConvexHull),
                PbrBundle { mesh: amah.sphere.clone(),
                            material: amah.glowy_material_2.clone(),
                            transform: transform.with_scale(Vec3::ONE * 0.4),
                            ..default() }),
               point_light(Vec3::ZERO, 0.4, Intensity::Med, GLOWY_COLOR_2))
      }
      'd' => {
        spawn!((RigidBody::Dynamic,
                ColliderMassProperties::Density(1.0),
                NotShadowCaster,
                NotShadowReceiver,
                // MassPropertiesBundle::default(),
                AsyncCollider(ComputedColliderShape::ConvexHull),
                name("uranium cube"),
                Restitution { coefficient: 0.9,
                              combine_rule: CoefficientCombineRule::Max },
                Friction { coefficient: 0.2,
                           combine_rule: CoefficientCombineRule::Multiply },
                PbrBundle { mesh: amah.cube.clone(),
                            material: amah.glowy_material_3.clone(),
                            transform,
                            ..default() }),
               point_light(Vec3::ZERO, 0.4, Intensity::Med, GLOWY_COLOR_3))
      }
      'l' => {
        glowy_sphere(translation).spawn(&mut c);
      }
      'p' => {
        let player_friction = 1.0;
        let player_collider = capsule_from_height_and_radius(PLAYER_HEIGHT, PLAYER_RADIUS);
        let player_mass = 0.3;
        spawn!(SpatialBundle::from_transform(transform),
               (FaceCamera,
                billboard(Transform::from_scale(Vec3::splat(2.0)),
                          amah.stickman.clone(),)));
        spawn!((Player::default(),
                ColliderMassProperties::Mass(player_mass),
                Friction { combine_rule: CoefficientCombineRule::Multiply,
                           coefficient: player_friction },
                Restitution { coefficient: 0.0,

                              combine_rule: CoefficientCombineRule::Multiply },
                ExternalImpulse::default(),
                ExternalForce::default(),
                Velocity::default(),
                RigidBody::Dynamic,
                ThirdPersonCameraTarget,
                LockedAxes::ROTATION_LOCKED,
                SpatialBundle::from_transform(transform),
                player_collider),
               (IsPlayerSprite,
                FaceCamera,
                billboard(Transform::from_scale(Vec3::splat(PLAYER_HEIGHT)),
                          amah.stickman.clone(),)))
      }
      'o' | ' ' => (),
      _ => () // _ => panic!("{:?}, {tile}", coords),
    }
  }
}
comment! {

/// The acceleration used for movement.
#[derive(Component)]
pub struct MovementAcceleration(Scalar);
pub fn ui(mut c: Commands) {
  c.spawn(Camera2dBundle { camera: Camera { order: -1,
                                            ..default() },
                           ..default() });
  c.spawn(NodeBundle { style: Style { width: Val::Percent(100.),
                                      ..default() },
                       background_color: Color::rgba(0.15, 0.15, 0.15, 0.0).into(),
                       ..default() })
   .with_children(|parent| {
     // text
     parent.spawn((
                TextBundle::from_section(
                    "Use Arrow Keys or WASD to Move The Chain",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::WHITE,
                        ..default()
                    },
                )
                .with_style(Style {
                    margin: UiRect {
                        left: Val::Px(5.0),
                        top: Val::Px(30.0),
                        ..default()
                    },
                    ..default()
                }),
                // Because this is a distinct label widget and
                // not button/list item text, this is necessary
                // for accessibility to treat the text accordingly.
                Label,
            ));
   });
}

}
