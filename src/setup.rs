use {crate::{assetstuff::{AllMyAssetHandles, GLOWY_COLOR, GLOWY_COLOR_2, GLOWY_COLOR_3},
             components::{name, FaceCamera, IsPlayerSprite, ItemPickUp, Player,
                          SpinningAnimation, Sun},
             jumpy_penguin::SegmentPathMotion,
             update::{capsule_from_height_and_radius, PLAYER_HEIGHT, PLAYER_RADIUS}},
     bevy::{core_pipeline::{self,
                            bloom::{BloomCompositeMode, BloomPrefilterSettings,
                                    BloomSettings}},
            math::vec3,
            pbr::NotShadowCaster,
            prelude::*,
            render::camera::Exposure},
     bevy_mod_billboard::{BillboardDepth, BillboardLockAxis, BillboardLockAxisBundle,
                          BillboardMeshHandle, BillboardTextBundle,
                          BillboardTextureBundle, BillboardTextureHandle},
     bevy_rapier3d::prelude::*,
     bevy_third_person_camera::{ThirdPersonCamera, ThirdPersonCameraTarget},
     rust_utils::comment};

pub fn billboard(transform: Transform,
                 image_handle: Handle<Image>,
                 amah: &Res<AllMyAssetHandles>)
                 -> impl Bundle {
  BillboardLockAxisBundle { billboard_bundle:
                              BillboardTextureBundle { transform,
                                                       texture:
                                                         BillboardTextureHandle(image_handle),
                                                       mesh:
                                                         BillboardMeshHandle(amah.unitsquare
                                                                                 .clone()),
                                                       billboard_depth:
                                                         BillboardDepth(true),
                                                       ..default() },
                            lock_axis: BillboardLockAxis { y_axis: true,
                                                           rotation: true } }
}
pub fn spawn_with_child(c: &mut Commands, a: impl Bundle, b: impl Bundle) {
  c.spawn(a).with_children(|x| {
              x.spawn(b);
            });
}
pub fn spawn_with_2_children(commands: &mut Commands,
                             a: impl Bundle,
                             b: impl Bundle,
                             c: impl Bundle) {
  commands.spawn(a).with_children(|x| {
                     x.spawn(b);
                     x.spawn(c);
                   });
}
pub fn setup(mut c: Commands, amah: Res<AllMyAssetHandles>) {
  macro_rules! spawn {
    ($bundle:expr) => {{
      c.spawn($bundle);
    }};
    ($bundle1:expr,$bundle2:expr) => {{
      c.spawn($bundle1).with_children(|x| {
                         x.spawn($bundle2);
                       });
    }};
  }
  let text_style = TextStyle { font_size: 30.0,
                               ..default() };
  spawn!(TextBundle::from(TextSection::new("z: ".to_string(), text_style.clone())));
  spawn!(ImageBundle { style: Style { width: Val::Percent(5.0),
                                      height: Val::Percent(7.0),
                                      ..default() },
                       image: UiImage::from(amah.mushroom_man.clone()),
                       ..default() });
  spawn!(PointLightBundle { transform: Transform::from_xyz(0.0, -4.0, 0.0),
                            point_light: PointLight { intensity: 2300.0,
                                                      range: 100.0,
                                                      shadows_enabled: true,
                                                      ..default() },
                            ..default() });
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
                    amah.iceberg.clone(),
                    &amah)));
  // GibSpriteBundle(Sprite3d { image: amah.iceberg.clone(),
  //          transform: Transform::from_xyz(-30.0, 0.0, -40.0),
  //          pixels_per_metre: 1.5,
  //          ..default() }));
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
  let glowy_sphere = |transform| {
    (PointLightBundle { transform,
                        point_light: PointLight { intensity: 400.0,
                                                  radius: 1.0,
                                                  // range: 100.0,
                                                  shadows_enabled: true,
                                                  color: GLOWY_COLOR,
                                                  ..default() },
                        ..default() },
     (PbrBundle { mesh: amah.sphere.clone(),
                  material: amah.glowy_material.clone(),
                  ..default() },
      NotShadowCaster,
      RigidBody::Fixed,
      Friction::default(),
      Velocity::default(),
      AsyncCollider(ComputedColliderShape::ConvexHull)))
  };
  let (light, sphere) = glowy_sphere(Transform::from_xyz(22.709263, -26.007673, 72.32278));
  spawn!(light, sphere);
  spawn!((RigidBody::Fixed,
          // Friction::new(0.1),
          AsyncSceneCollider { shape: Some(ComputedColliderShape::TriMesh),
                               named_shapes: default() },
          SceneBundle { scene: amah.alevel.clone(),
                        transform: Transform::from_xyz(40.0, -30.0, 60.0),
                        ..default() }));
  spawn!((Sun::default(),
          billboard(Transform::from_scale(Vec3::splat(20.0)),
                    amah.sun.clone(),
                    &amah)),
         DirectionalLightBundle { directional_light:
                                    DirectionalLight { color: Color::WHITE,
                                                       illuminance: 11000.0,
                                                       shadows_enabled: true,
                                                       ..default()
                                                       // shadow_depth_bias: todo!(),
                                                       // shadow_normal_bias: todo!()
                                    },
                                  ..default() });

  // ScreenSpaceAmbientOcclusionPlugin
  // Camera
  spawn!((Camera3dBundle { camera: Camera { hdr: true,

                                            ..default() },
                           exposure: Exposure { ev100: 10.0 },
                           tonemapping:
                             core_pipeline::tonemapping::Tonemapping::Reinhard,
                           ..default() },
          // FogSettings { color: Color::rgb(0.2, 0.2, 0.4),
          //               falloff: FogFalloff::ExponentialSquared { density: 0.01 },
          //               ..default() },
          // UiCameraConfig { show_ui: true },
          // Exposure::OVERCAST,
          BloomSettings { intensity: 0.5,
                          low_frequency_boost: 0.0,
                          prefilter_settings: BloomPrefilterSettings { threshold:
                                                                         2.2,
                                                                       ..default() },
                          composite_mode: BloomCompositeMode::Additive,
                          ..default() },
          // Skybox(amah.skybox.clone()),
          ThirdPersonCamera { cursor_lock_key: KeyCode::Tab,
                              cursor_lock_toggle_enabled: true,
                              cursor_lock_active: false,
                              mouse_sensitivity: 1.7,
                              zoom: bevy_third_person_camera::Zoom::new(1.2, 13.0),
                              zoom_sensitivity: 0.1,
                              ..default() }));
  // .insert(bevy::pbr::ScreenSpaceAmbientOcclusionBundle::default())
  // .insert(bevy::core_pipeline::experimental::taa::TemporalAntiAliasBundle::default())
  let level =
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
                                });
  for ([x, y, z], tile) in level {
    let transform =
      Transform::from_translation(Vec3::from_slice(&[x, y, z].map(|n| n as f32)));
    let block = |material: Handle<StandardMaterial>| {
      (RigidBody::Fixed,
       Friction::default(),
       Velocity::default(),
       AsyncCollider(ComputedColliderShape::ConvexHull),
       PbrBundle { mesh: amah.unitcube.clone(),
                   material,
                   transform,
                   ..default() })
    };
    match tile {
      'w' => spawn!(block(amah.funky_material.clone())),
      'g' => spawn!(block(amah.grass_material.clone())),
      's' => spawn!(block(amah.snow_material.clone())),
      'S' => spawn!(block(amah.stone_material.clone())),
      'o' | ' ' => (),
      'c' => spawn!((RigidBody::Dynamic,
                     ColliderMassProperties::Density(1.0),
                     // MassPropertiesBundle::default(),
                     AsyncCollider(ComputedColliderShape::ConvexHull),
                     PbrBundle { mesh: amah.cube.clone(),
                                 material: amah.colorful_material.clone(),
                                 transform,
                                 ..default() })),
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
               PointLightBundle { point_light: PointLight { intensity: 300.0,
                                                            radius: 0.4,
                                                            shadows_enabled: true,
                                                            color: GLOWY_COLOR_2,
                                                            ..default() },
                                  ..default() })
      }
      'p' => {
        let player_friction = 1.0;
        let player_collider = capsule_from_height_and_radius(PLAYER_HEIGHT, PLAYER_RADIUS);
        let player_mass = 0.3;
        spawn!(SpatialBundle::from_transform(transform),
               (FaceCamera,
                billboard(Transform::from_scale(Vec3::splat(2.0)),
                          amah.stickman.clone(),
                          &amah)));
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
                          amah.stickman.clone(),
                          &amah)))
      }
      't' => spawn!((RigidBody::Fixed,
                     capsule_from_height_and_radius(0.8, 0.2),
                     FaceCamera,
                     billboard(transform, amah.tree.clone(), &amah))),
      'C' => {
        spawn!((ItemPickUp::SpeedBoost,
                SceneBundle { transform,
                              ..default() }),
               (SceneBundle { scene: amah.coffee_scene.clone(),
                              transform: Transform::default().with_scale(Vec3::ONE
                                                                         * 0.1),
                              ..default() },
                SpinningAnimation { rotation_steps: default(),
                                    up_down_steps: default(),
                                    up_down_distance: 0.3 }))
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
      'l' => {
        let (light, sphere) = glowy_sphere(transform);
        spawn!(light, sphere);
        // spawn!(PointLightBundle { transform,
        //                           point_light: PointLight { intensity: 400.0,
        //                                                     radius: 1.0,
        //                                                     // range: 100.0,
        //                                                     shadows_enabled: true,
        //                                                     color: GLOWY_COLOR,
        //                                                     ..default() },
        //                           ..default() },
        //        (PbrBundle { mesh: amah.uvsphere.clone(),
        //                     material: amah.glowy_material.clone(),
        //                     ..default() },
        //         NotShadowCaster,
        //         RigidBody::Fixed,
        //         Friction::default(),
        //         Velocity::default(),
        //         AsyncCollider(ComputedColliderShape::ConvexHull)))
      }
      'd' => {
        spawn!((RigidBody::Dynamic,
                ColliderMassProperties::Density(1.0),
                NotShadowCaster,
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
               PointLightBundle { point_light: PointLight { intensity: 300.0,
                                                            radius: 0.4,
                                                            shadows_enabled: true,
                                                            color: GLOWY_COLOR_3,
                                                            ..default() },
                                  ..default() })
      }
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
