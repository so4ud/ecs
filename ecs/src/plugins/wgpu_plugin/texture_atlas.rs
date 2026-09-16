use std::ops::Index;

use crate::components::Component;
use crate::plugins::wgpu_plugin::state::State;
use json;
use macros::Component;

pub type TextureID = usize;

#[derive(Debug, Clone, Copy, Component)]
pub struct Texture {
    /// index into the `TextureAtlas` recource
    pub texture_id: TextureID,
}

pub struct TextureAtlas {
    pub(crate) atlas: wgpu::Texture,
    pub(crate) texture_views: wgpu::TextureView,
    // ! have to use this
    pub(crate) texture_info: Vec<TextureInfo>,
}
impl TextureAtlas {
    pub(crate) fn new(state: &mut State) -> Self {
        let texture = state.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("main texture atlas"),
            size: wgpu::Extent3d {
                width: 4096,
                height: 4096,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb, // Standard for sRGB PNGs
            usage: wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut pih = Self {
            atlas: texture,
            texture_views: texture_view,
            texture_info: vec![],
        };
        pih.load_texture(
            state,
            "assets/textures/texture_atlas_0.png",
            Some("texture atlas 0".to_string()),
        );
        return pih;
    }
    pub(crate) fn read_texture_data(&mut self) {
        let file = std::fs::read("assets/textures/texture_infos.json")
            .expect("no texture info file present");
        let texture_data_json =
            json::parse(&String::from_utf8(file).expect("texture data file not valid utf-8"))
                .expect("texture data file not valid json");

        for texture_data in texture_data_json.entries() {
            let name = texture_data.0.to_string();
            let atlas_id = texture_data
                .1
                .index("atlas_id")
                .as_u32()
                .expect(&format!("unexpected token at: {}", &name));
            let origin0 = texture_data
                .1
                .index("origin")
                .index(0)
                .as_u32()
                .expect(&format!("unexpected token at: {}", &name));
            let origin1 = texture_data
                .1
                .index("origin")
                .index(1)
                .as_u32()
                .expect(&format!("unexpected token at: {}", &name));
            let origin = (origin0, origin1);

            let size0 = texture_data
                .1
                .index("size")
                .index(0)
                .as_u32()
                .expect(&format!("unexpected token at: {}", &name));
            let size1 = texture_data
                .1
                .index("size")
                .index(1)
                .as_u32()
                .expect(&format!("unexpected token at: {}", &name));
            let size = (size0, size1);
            let texture_info = TextureInfo {
                name: Some(name),
                size,
                origin,
                atlas_id,
            };
            self.texture_info.push(texture_info);
        }
    }
    /// png only
    pub(crate) fn load_texture(
        &mut self,
        state: &mut State,
        file_path: impl AsRef<std::path::Path>,
        name: Option<String>,
    ) -> TextureID {
        let img_bytes = std::fs::read(file_path).unwrap();
        let img = image::load_from_memory(&img_bytes[..]).unwrap().to_rgba8();
        let dimensions = img.dimensions();

        // 1. Define texture size and descriptor
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        // 2. Write the image data to the queue
        state.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &self.atlas,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &img,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        // self.texture_info.push(TextureInfo {
        //     name,
        //     size: dimensions,
        //     origin: (0, 0),
        //     atlas_id: 0,
        // });
        return self.texture_info.len() - 1;
    }
    pub fn get_texture(&self, texture_id: TextureID) -> (&wgpu::TextureView, &TextureInfo) {
        if texture_id > self.texture_info.len() - 1 {
            return (&self.texture_views, &self.texture_info[0]);
        }
        return (&self.texture_views, &self.texture_info[texture_id]);
    }
}
// ! encoder.copy_texture_to_texture(source, destination, Extent3d {depth_or_array_layers});

#[derive(Debug)]
pub struct TextureInfo {
    pub name: Option<String>,
    /// size in pixels
    pub size: (u32, u32),
    /// start position in the `TextureAtlas`, top-left corner
    pub origin: (u32, u32),
    /// index into `TextureAtlas.atlas`
    pub atlas_id: u32,
    // format: wgpu::TextureFormat,
}
