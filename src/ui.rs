use {crate::{assetstuff::AllMyAssetHandles, bundletree::BundleTree, update::FaceCameraDir},
     bevy::{math::vec3, prelude::*, render::render_resource::CachedRenderPipelineId,
            text::BreakLineOn},
     bevy_mod_billboard::{text::BillboardTextHandles, BillboardLockAxis,
                          BillboardLockAxisBundle, BillboardMeshHandle,
                          BillboardTextBundle, BillboardTextureBundle,
                          BillboardTextureHandle},
     bevy_vox_scene::{VoxelScene, VoxelSceneBundle},
     rust_utils::{mapv, most},
     std::{f32::consts::PI, ops::Index}};

#[derive(Component)]
pub struct UiPopup {
  pub strings: Vec<String>,
  pub foreground_child: Option<Entity>,
  pub background_child: Option<Entity>
}

impl UiPopup {
  pub fn new<T: Into<String>>(strings: impl IntoIterator<Item = T>) -> Self {
    Self { strings: mapv(Into::into, strings),
           foreground_child: None,
           background_child: None }
  }
}
#[derive(Component)]
struct UiPopupBackground;
#[derive(Component)]
struct UiPopupForeground;
pub fn ui_pop_up(mut q: Query<(Entity, &mut UiPopup)>,
                 mut c: Commands,
                 amah: Res<AllMyAssetHandles>) {
  for (e, mut p) in &mut q {
    if p.is_changed() {
      let strings: &Vec<String> = &p.strings;
      let strings_max_len = strings.iter().map(|s| s.chars().count()).max().unwrap_or(0);
      let background_width = strings_max_len as f32 * 1.0;
      let background_height = strings.len() as f32 * 1.0;
      let font_size = 16.0;
      let newline = "\n".to_string();
      let text_style = |color| TextStyle { font: amah.font(),
                                           font_size,
                                           color };
      let locked_text = |billboard_bundle| {
        BillboardLockAxisBundle { billboard_bundle,
                                  lock_axis: BillboardLockAxis { y_axis: true,
                                                                 rotation: true } }
      };
      let text =
        Text::from_sections(strings.iter().intersperse(&newline).map(|s| {
                                                                  TextSection { value: s.to_owned(),
                                                          style:
                                                            text_style(Color::WHITE) }
                                                                }));
      let foreground = (FaceCameraDir,
                        locked_text(BillboardTextBundle { text,
                                                          transform:
                                                            Transform::from_scale(vec3(-1.0,
                                                                                       1.0,
                                                                                       1.0)),
                                                          ..default() }));
      let background =
        locked_text(BillboardTextBundle {
             transform: Transform::from_scale(vec3(background_width,
                                                   background_height,
                                                   1.0)),
             text: Text::from_section("█",
                                      text_style(Color::BLACK.with_a(0.9))),
             ..default() });
      if let &UiPopup { foreground_child: Some(fe),
                        background_child: Some(be),
                        .. } = p.as_ref()
      {
        c.entity(fe).insert(foreground);
        c.entity(be).insert(background);
      } else {
        let fe = c.spawn(foreground).id();
        let be = c.spawn(background).id();
        c.entity(e).add_child(fe);
        // c.entity(e).add_child(be);
        c.entity(fe).add_child(be);
        p.foreground_child = Some(fe);
        p.background_child = Some(be);
      }
    }
  }
}
