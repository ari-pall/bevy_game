use {crate::{assetstuff::AllMyAssetHandles,
             components::{GibSpriteBundle, ItemPickUp, Player},
             jumpy_penguin::SegmentPathMotion},
     bevy::{core_pipeline, math::vec3, prelude::*},
     bevy_rapier3d::prelude::*,
     bevy_sprite3d::Sprite3d,
     bevy_third_person_camera::{ThirdPersonCamera, ThirdPersonCameraTarget},
     rust_utils::comment};

// // Environment (see `async_colliders` example for creating colliders from scenes)
// c.spawn((
//   SceneBundle {
//     scene: amah.island_level_scene.clone(),
//     transform: Transform::from_scale(Vec3::ONE * 20.0)
//       .with_translation(Vec3::NEG_ONE * 20.0),
//     ..default()
//   },
//   AsyncSceneCollider::new(Some(ComputedCollider::ConvexDecomposition(
//     VHACDParameters { ..default() },
//   ))),
//   RigidBody::Static,
// ));
// pub fn iceberg(center: Vec3, speed: f32, amah: &AllMyAssetHandles) -> impl Bundle {
//   (RigidBody::Kinematic,
//    Friction::default(),
//    SegmentPathMotion::circular_motion(center, 15.0, speed),
//    AsyncCollider(ComputedCollider::ConvexHull),
//    PbrBundle { mesh: amah.flatbox.clone(),
//                material: amah.snow_material.clone(),
//                transform: Transform::from_translation(center),
//                ..default() })
// }
pub fn spawn_with_child(c: &mut Commands, a: impl Bundle, b: impl Bundle) {
  c.spawn(a).with_children(|x| {
              x.spawn(b);
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

  let iceberg = |mut spm: SegmentPathMotion| {
    (RigidBody::KinematicVelocityBased,
     Velocity::default(),
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
  spawn!(circle_iceberg(vec3(0.0, -6.0, 0.0), 0.0, 1.3));

  let up_down_iceberg = |origin: Vec3, height: f32, speed: f32| {
    iceberg(SegmentPathMotion::new([origin, origin + Vec3::Y * height], speed))
  };
  spawn!(up_down_iceberg(vec3(5.343936, -3.0, -2.4758048), 4.0, 0.5));
  spawn!(up_down_iceberg(vec3(9.069067, -4.0, -5.0675673), 4.0, 0.4));
  spawn!(up_down_iceberg(vec3(12.84221, -6.0, -4.947112), 4.0, 0.3));
  spawn!(GibSpriteBundle(Sprite3d { image: amah.iceberg.clone(),
                                    transform: Transform::from_xyz(-30.0, 0.0, -40.0),
                                    pixels_per_metre: 1.5,
                                    ..default() }));
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
  // ScreenSpaceAmbientOcclusionPlugin
  // Camera
  spawn!((Camera3dBundle { camera: Camera { hdr: true,
                                            ..default() },
                           tonemapping:
                             core_pipeline::tonemapping::Tonemapping::Reinhard,
                           ..default() },
          UiCameraConfig { show_ui: true },
          ThirdPersonCamera { cursor_lock_key: KeyCode::Tab,
                              cursor_lock_toggle_enabled: true,
                              cursor_lock_active: false,
                              mouse_sensitivity: 1.7,
                              zoom: bevy_third_person_camera::Zoom::new(1.2, 13.0),
                              zoom_sensitivity: 0.2,
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
       AsyncCollider(ComputedColliderShape::ConvexHull),
       MaterialMeshBundle { mesh: amah.unitcube.clone(),
                            material,
                            transform,
                            ..default() })
    };
    // Collider::from_bevy_mesh
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
      'p' => {
        let player_height = 0.75;
        let player_radius = 0.3;
        let player_friction = 4.0;
        let player_collider = Collider::capsule_y(player_height / 2.0, player_radius);
        // let player_density = 1.0;
        let player_mass = 0.3;
        // let player_shape_caster = ShapeCaster::new(player_collider.clone(),
        //                                            Vec3::ZERO,
        //                                            Quat::IDENTITY,
        //                                            Vec3::NEG_Y).with_max_time_of_impact(0.3)
        //                                                        .with_max_hits(5);
        spawn_with_child(&mut c,
                         (Player { speed_boost: 0.0,
                                   jump_charge_level: None },
                          ColliderMassProperties::Mass(player_mass),
                          Friction { combine_rule: CoefficientCombineRule::Multiply,
                                     coefficient: player_friction },
                          Restitution { coefficient: 0.0,
                                        combine_rule: CoefficientCombineRule::Multiply },
                          ExternalForce::default(),
                          ExternalImpulse::default(),
                          Velocity::default(),
                          RigidBody::Dynamic,
                          ThirdPersonCameraTarget,
                          LockedAxes::ROTATION_LOCKED,
                          SpatialBundle::from_transform(transform),
                          player_collider,
                          // player_shape_caster
                         ),
                         GibSpriteBundle(Sprite3d { image: amah.penguin_image.clone(),
                                                    pixels_per_metre: 19.0,
                                                    ..default() }))
      }
      't' => spawn!((RigidBody::Fixed,
                     Collider::capsule_y(0.6, 0.2),
                     GibSpriteBundle(Sprite3d { image: amah.tree.clone(),
                                                transform,
                                                pixels_per_metre: 12.0,
                                                ..default() }))),
      'C' => spawn!((ItemPickUp::SpeedBoost,
                     GibSpriteBundle(Sprite3d { image: amah.coffee.clone(),
                                                transform,
                                                pixels_per_metre: 18.0,
                                                ..default() }))),
      'L' => spawn!((RigidBody::Dynamic,
                     ColliderMassProperties::Density(1.0),
                     // MassPropertiesBundle::default(),
                     AsyncSceneCollider { shape:
                                            Some(ComputedColliderShape::ConvexHull),
                                          named_shapes: default() },
                     SceneBundle { scene: amah.lunarlander.clone(),
                                   transform,
                                   ..default() })),
      'l' => spawn!(PointLightBundle { transform,
                                       point_light: PointLight { intensity: 1500.0,
                                                                 radius: 0.7,
                                                                 range: 50.0,
                                                                 shadows_enabled:
                                                                   true,
                                                                 ..default() },
                                       ..default() }),
      _ => (),
      // _ => panic!("{:?}, {tile}", coords),
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
