use wgpu::util::DeviceExt;
use winit::window::Window;
pub mod texture;
use crate::prelude::*;
use std::mem::size_of;
pub use texture::TextureId;

#[inline]
fn screen_coords_to_wgpu((x, y): (f32, f32), screen_size: (u32, u32)) -> (f32, f32) {
    let x = (2.0 / screen_size.0 as f32) * x - 1.0;
    let y = (2.0 / screen_size.1 as f32) * y - 1.0;
    (x, -y)
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 2],
    pub color: [f32; 4],
}

impl Vertex {
    const fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct TextureVertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

impl TextureVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: size_of::<TextureVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    screen_width: f32,
    screen_height: f32,
}

pub struct WgpuBackend {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
    pub(crate) size: winit::dpi::PhysicalSize<u32>,

    render_pipeline: wgpu::RenderPipeline,
    vertex_buffer: wgpu::Buffer,
    vertices: Vec<Vertex>,

    texture_render_pipeline: wgpu::RenderPipeline,
    texture_vertex_buffer: wgpu::Buffer,
    texture_sampler: wgpu::Sampler,
    texture_bind_group_layout: wgpu::BindGroupLayout,
    texture_bind_groups: Vec<(usize, TextureId, wgpu::Texture, wgpu::BindGroup)>,
    texture_id_counter: usize,
    images: Vec<(TextureId, Rect)>,

    uniforms: Uniforms,
    uniform_buffer: wgpu::Buffer,
    uniform_bind_group: wgpu::BindGroup,
}

impl WgpuBackend {
    pub(crate) async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            format: adapter.get_swap_chain_preferred_format(&surface).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        // Setup textures
        let texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("Texture Sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler {
                            comparison: false,
                            filtering: true,
                        },
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });

        // Setup uniforms
        let uniforms = Uniforms {
            screen_width: window.inner_size().width as f32,
            screen_height: window.inner_size().height as f32,
        };

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(&[uniforms]),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("uniform_bind_group_layout"),
            });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("uniform_bind_group"),
        });

        // Setup render pipeline
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Render Pipeline Layout"),
                bind_group_layouts: &[&uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let vs_module =
            device.create_shader_module(&wgpu::include_spirv!("ressources/shader.vert.spv"));
        let fs_module =
            device.create_shader_module(&wgpu::include_spirv!("ressources/shader.frag.spv"));

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: sc_desc.format,
                    blend: Some(wgpu::BlendState {
                        alpha: wgpu::BlendComponent::REPLACE,
                        color: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: None,
                polygon_mode: wgpu::PolygonMode::Fill,
                clamp_depth: false, // Might be useful later
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        // Setup texture render pipeline
        let texture_render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Texture Render Pipeline Layout"),
                bind_group_layouts: &[&uniform_bind_group_layout, &texture_bind_group_layout],
                push_constant_ranges: &[],
            });

        let texture_vs_module = device
            .create_shader_module(&wgpu::include_spirv!("ressources/texture-shader.vert.spv"));
        let texture_fs_module = device
            .create_shader_module(&wgpu::include_spirv!("ressources/texture-shader.frag.spv"));

        let texture_render_pipeline =
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("Textured Render Pipeline"),
                layout: Some(&texture_render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &texture_vs_module,
                    entry_point: "main",
                    buffers: &[TextureVertex::desc()],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &texture_fs_module,
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: sc_desc.format,
                        blend: Some(wgpu::BlendState {
                            alpha: wgpu::BlendComponent::REPLACE,
                            color: wgpu::BlendComponent::REPLACE,
                        }),
                        write_mask: wgpu::ColorWrite::ALL,
                    }],
                }),
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: wgpu::FrontFace::Ccw,
                    cull_mode: None,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    clamp_depth: false,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: wgpu::MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false,
                },
            });

        // Setup vertex buffers
        let data = vec![0; 1_000_000];
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: &data,
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
        });

        let texture_vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Texture Vertex Buffer"),
            contents: &data,
            usage: wgpu::BufferUsage::VERTEX | wgpu::BufferUsage::COPY_DST,
        });

        Self {
            surface,
            device,
            queue,
            sc_desc,
            swap_chain,
            size,

            render_pipeline,
            vertex_buffer,
            vertices: Vec::new(),

            texture_render_pipeline,
            texture_vertex_buffer,
            texture_sampler,
            texture_id_counter: 0,
            texture_bind_group_layout,
            texture_bind_groups: Vec::new(),
            images: Vec::new(),

            uniforms,
            uniform_buffer,
            uniform_bind_group,
        }
    }

    pub(crate) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;

        self.uniforms.screen_width = new_size.width as f32;
        self.uniforms.screen_height = new_size.height as f32;
        self.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms]),
        );

        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub(crate) fn update(&mut self) {
        self.queue
            .write_buffer(&self.vertex_buffer, 0, bytemuck::cast_slice(&self.vertices));

        self.images.sort_by_key(|(id, _)| *id.id);
        let mut texture_vertices = Vec::with_capacity(6 * self.images.len());
        for (_, rect) in &self.images {
            texture_vertices.push(TextureVertex {
                position: [rect.max.0, rect.min.1],
                tex_coords: [1.0, 0.0],
            });
            texture_vertices.push(TextureVertex {
                position: [rect.min.0, rect.max.1],
                tex_coords: [0.0, 1.0],
            });
            texture_vertices.push(TextureVertex {
                position: [rect.min.0, rect.min.1],
                tex_coords: [0.0, 0.0],
            });

            texture_vertices.push(TextureVertex {
                position: [rect.max.0, rect.min.1],
                tex_coords: [1.0, 0.0],
            });
            texture_vertices.push(TextureVertex {
                position: [rect.min.0, rect.max.1],
                tex_coords: [0.0, 1.0],
            });
            texture_vertices.push(TextureVertex {
                position: [rect.max.0, rect.max.1],
                tex_coords: [1.0, 1.0],
            });
        }
        self.queue.write_buffer(
            &self.texture_vertex_buffer,
            0,
            bytemuck::cast_slice(&texture_vertices),
        );

        // Sweep unused textures
        self.texture_bind_groups
            .retain(|(_, texture_id, texture, _)| {
                if std::sync::Arc::strong_count(&texture_id.id) > 1 {
                    true
                } else {
                    texture.destroy();
                    false
                }
            });
    }

    /**
    Adds a [Vertex] to the buffer.  
    It will be drawn at the next frame and then removed.
    **/
    pub fn add_vertex(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }

    /**
    Draws an image at the specified position.  
    A [TextureId] can be obtained with [WgpuBackend::create_texture].
    **/
    pub fn add_image(&mut self, mut position: Rect, texture_id: TextureId) {
        position.min = screen_coords_to_wgpu(position.min, (self.size.width, self.size.height));
        position.max = screen_coords_to_wgpu(position.max, (self.size.width, self.size.height));
        self.images.push((texture_id, position));
    }
    
    /**
    Creates a new texture that will be destroyed once all clones of the returned [TextureId] are dropped.  
    Panics if the image data is not consistent with the indicated image dimensions (its len must be `4*width*height` bytes).
    **/
    pub fn create_texture(&mut self, image_dimensions: (u32, u32), image_rgba: &[u8]) -> TextureId {
        assert_eq!(
            image_dimensions.0 as usize * image_dimensions.1 as usize * 4,
            image_rgba.len()
        );

        let texture_size = wgpu::Extent3d {
            width: image_dimensions.0,
            height: image_dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            size: texture_size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            label: Some("Texture"),
        });

        self.queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            image_rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * image_dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(image_dimensions.1),
            },
            texture_size,
        );

        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let texture_bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &self.texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.texture_sampler),
                },
            ],
            label: Some("texture_bind_group"),
        });

        let texture_id_usize = self.texture_id_counter;
        self.texture_id_counter += 1;
        let texture_id = TextureId::new(texture_id_usize);
        self.texture_bind_groups.push((
            texture_id_usize,
            texture_id.clone(),
            texture,
            texture_bind_group,
        ));

        texture_id
    }

    pub(crate) fn render(&mut self) -> Result<(), wgpu::SwapChainError> {
        let frame = self.swap_chain.get_current_frame()?.output;
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &frame.view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });

        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..self.vertices.len() as u32, 0..1);

        if !self.images.is_empty() {
            render_pass.set_pipeline(&self.texture_render_pipeline);
            render_pass.set_vertex_buffer(0, self.texture_vertex_buffer.slice(..));
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]); // Is this required?

            let mut image_id = 0;
            'image_rendering: for (id, _, _, texture_bind_group) in &self.texture_bind_groups {
                if *self.images[image_id].0.id == *id {
                    render_pass.set_bind_group(1, texture_bind_group, &[]);
                    while *self.images[image_id].0.id == *id {
                        render_pass.draw(
                            (image_id * 6 * size_of::<TextureVertex>()) as u32
                                ..((image_id + 1) * 6 * size_of::<TextureVertex>()) as u32,
                            0..1,
                        );
                        image_id += 1;
                        if image_id >= self.images.len() {
                            break 'image_rendering;
                        }
                    }
                }
            }
        }

        std::mem::drop(render_pass);

        self.queue.submit(std::iter::once(encoder.finish()));
        self.vertices.clear();
        self.images.clear();

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coords_conversion() {
        assert_eq!(screen_coords_to_wgpu((0.0, 0.0), (21654, 212)), (-1.0, 1.0));
        assert_eq!(screen_coords_to_wgpu((21654.0, 212.0), (21654, 212)), (1.0, -1.0));
        
        assert_eq!(screen_coords_to_wgpu((0.0, 50.0), (100, 100)), (-1.0, 0.0));
        assert_eq!(screen_coords_to_wgpu((50.0, 50.0), (100, 100)), (0.0, 0.0));
        assert_eq!(screen_coords_to_wgpu((100.0, 50.0), (100, 100)), (1.0, 0.0));

        assert_eq!(screen_coords_to_wgpu((50.0, 0.0), (100, 100)), (0.0, 1.0));
        assert_eq!(screen_coords_to_wgpu((50.0, 100.0), (100, 100)), (0.0, -1.0));
    }
}
