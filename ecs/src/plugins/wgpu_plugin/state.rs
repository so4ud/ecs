use std::collections::HashMap;
use std::num::NonZero;
use std::sync::Arc;

use wgpu::util::DeviceExt;
use wgpu::{
    BindGroupLayout, Buffer, Extent3d, RenderPipeline, RenderPipelineDescriptor, Texture,
    TextureView,
};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop, OwnedDisplayHandle};
use winit::window::{Window, WindowId};

use crate::app::App;
use crate::ecs::ECS;
use crate::plugins::wgpu_plugin::vertex::Vertex;
use crate::plugins::wgpu_plugin::{RenderingPipelinesAndBinds, TextureInfo, Uniforms};
use crate::wgpu;

pub struct State {
    pub instance: wgpu::Instance,
    pub window: Arc<Window>,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub surface: wgpu::Surface<'static>,
    pub surface_format: wgpu::TextureFormat,
}

impl State {
    pub async fn new(display: OwnedDisplayHandle, window: Arc<Window>) -> State {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor::new_with_display_handle(
            Box::new(display),
        ));
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (mut device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .unwrap();

        let size = window.inner_size();

        let surface = instance.create_surface(window.clone()).unwrap();
        let cap = surface.get_capabilities(&adapter);
        let surface_format = cap.formats[0];

        let state = State {
            instance,
            window,
            device,
            queue,
            size,
            surface,
            surface_format,
        };
        state.configure_surface();

        state
    }

    pub(crate) fn init_rendering_pipeline(&self, ecs: &mut ECS) {
        let shader = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            });
        let bind_group_layout: BindGroupLayout =
            self.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some(" Bind Group Layout"),
                    entries: &[
                        wgpu::BindGroupLayoutEntry {
                            binding: 0,
                            visibility: wgpu::ShaderStages::all(),
                            ty: wgpu::BindingType::Buffer {
                                ty: wgpu::BufferBindingType::Uniform,
                                has_dynamic_offset: false,
                                min_binding_size: Some(
                                    NonZero::new(std::mem::size_of::<Uniforms>() as u64).unwrap(),
                                ),
                            },
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 1,
                            visibility: wgpu::ShaderStages::all(),
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                            count: None,
                        },
                        wgpu::BindGroupLayoutEntry {
                            binding: 2,
                            visibility: wgpu::ShaderStages::all(),
                            ty: wgpu::BindingType::Texture {
                                sample_type: wgpu::TextureSampleType::Float { filterable: false },
                                view_dimension: wgpu::TextureViewDimension::D2,
                                multisampled: false,
                            },
                            count: None,
                        },
                    ],
                });
        // 2. Create the pipeline layout (defines uniform/bind group layouts)
        let render_pipeline_layout =
            self.device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Render Pipeline Layout"),
                    bind_group_layouts: &[Some(&bind_group_layout)], // Add your &BindGroupLayouts here if passing uniforms
                    immediate_size: 0,
                });

        // 3. Instantiate the Render Pipeline
        let render_pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Render Pipeline"),
                layout: Some(&render_pipeline_layout),

                // Vertex shader stage configuration
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"), // Name of function in WGSL
                    buffers: &[crate::plugins::wgpu_plugin::vertex::Vertex::desc()], // Array of VertexBufferLayouts (if passing vertex data)
                    // buffers: &[wgpu::VertexBufferLayout {array_stride: 128, step_mode: wgpu::VertexStepMode::Vertex, attributes}],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                },

                // Fragment shader stage configuration (optional, but required for drawing colors)
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: wgpu::TextureFormat::Bgra8UnormSrgb, // Match your surface/texture format
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),

                // How to interpret vertices into geometric shapes
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList, // Draw triangles
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw, // Counter-clockwise is front facing
                    cull_mode: Some(wgpu::Face::Back), // Skip rendering hidden back faces
                    polygon_mode: wgpu::PolygonMode::Fill,
                    unclipped_depth: false,
                    conservative: false,
                },

                depth_stencil: None, // Set up if you need a depth buffer for 3D depth-testing

                multisample: wgpu::MultisampleState {
                    count: 1, // Standard 1x sampling (Anti-aliasing config)
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
                multiview_mask: None,
                cache: None,
            });
        let mut ses = RenderingPipelinesAndBinds {
            rendering_infos: HashMap::new(),
        };
        ses.rendering_infos.insert(
            "3d".to_string(),
            super::RenderingPipelineAndBind {
                bind_group_layout,
                render_pipeline,
            },
        );
        ecs.insert_recource(ses);
    }

    pub fn get_window(&self) -> &Window {
        &self.window
    }

    pub fn configure_surface(&self) {
        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: self.surface_format,
            view_formats: vec![self.surface_format.add_srgb_suffix()],
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            width: self.size.width,
            height: self.size.height,
            desired_maximum_frame_latency: 2,
            present_mode: wgpu::PresentMode::AutoVsync,
        };
        self.surface.configure(&self.device, &surface_config);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        // dbg!(&new_size);
        self.size = new_size;

        if self.size.width != 0 && self.size.height != 0 {
            self.configure_surface();
        }
    }

    pub fn render(
        &mut self,
        vertex_buffer: Option<(&Buffer, u32)>,
        texture: Option<&TextureView>,
        texture_info: Option<&TextureInfo>,
        m: [[f32; 4]; 4],
        v: [[f32; 4]; 4],
        p: [[f32; 4]; 4],
        render_pipeline: &RenderPipeline,
        bind_group_layout: &BindGroupLayout,
    ) {
        let surface_texture = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(texture) => texture,
            wgpu::CurrentSurfaceTexture::Occluded | wgpu::CurrentSurfaceTexture::Timeout => return,
            wgpu::CurrentSurfaceTexture::Suboptimal(texture) => {
                drop(texture);
                self.configure_surface();
                return;
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.configure_surface();
                return;
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                unreachable!("No error scope registered, so validation errors will panic")
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                self.surface = self.instance.create_surface(self.window.clone()).unwrap();
                self.configure_surface();
                return;
            }
        };
        let texture_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                // Without add_srgb_suffix() the image we will be working with
                // might not be "gamma correct".
                format: Some(self.surface_format.add_srgb_suffix()),
                // format: Some(wgpu::TextureFormat::Rgba16Float),
                ..Default::default()
            });

        // Renders a GREEN screen
        let mut encoder = self.device.create_command_encoder(&Default::default());
        // Create the renderpass which will clear the screen.
        let size = self.size.clone();
        // dbg!(&size);

        if size.width != 0 && size.height != 0 {
            let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &texture_view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.15,
                            g: 0.15,
                            b: 0.15,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            let uniform_buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("MVP Uniform Buffer"),
                size: std::mem::size_of::<Uniforms>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
            let origin = texture_info.unwrap().origin;
            let origin = [origin.0 as f32, origin.1 as f32];
            let origin = [origin[0] / 4096.0, origin[1] / 4096.0];
            let scale = texture_info.unwrap().size;
            let scale = [scale.0 as f32, scale.1 as f32];
            let scale = [scale[0] / 4096.0, scale[1] / 4096.0];
            let uniforms = Uniforms {
                m: m.into(),
                v: v.into(),
                p: p.into(),
                origin,
                scale,
            };
            self.queue
                .write_buffer(&uniform_buffer, 0, bytemuck::cast_slice(&[uniforms]));

            let sampler = self
                .device
                .create_sampler(&wgpu::SamplerDescriptor::default());
            if texture.is_some() {
                let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("Texture Bind Group"),
                    layout: &bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: uniform_buffer.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&sampler),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: wgpu::BindingResource::TextureView(&texture.unwrap()),
                        },
                    ],
                });
                renderpass.set_bind_group(0, &bind_group, &[]);

                renderpass.set_vertex_buffer(0, vertex_buffer.as_ref().unwrap().0.slice(..));
                renderpass.set_pipeline(render_pipeline);
                renderpass.draw(0..vertex_buffer.as_ref().unwrap().1, 0..1);
            }

            drop(renderpass);

            self.queue.submit([encoder.finish()]);
            self.window.pre_present_notify();
            surface_texture.present();
        }
    }
}
