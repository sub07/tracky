use std::num::NonZeroU64;

use joy_vector::Vector;
use ratatui_wgpu::{
    wgpu::{
        include_wgsl, AddressMode, BindGroupDescriptor, BindGroupEntry, BindGroupLayout,
        BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingResource, BindingType, Buffer,
        BufferBindingType, BufferDescriptor, BufferUsages, Color, ColorTargetState, ColorWrites,
        CommandEncoder, Device, FilterMode, FragmentState, LoadOp, MultisampleState, Operations,
        PipelineCompilationOptions, PipelineLayoutDescriptor, PrimitiveState, PrimitiveTopology,
        Queue, RenderBundle, RenderBundleDescriptor, RenderBundleEncoderDescriptor,
        RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor,
        Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages, StoreOp,
        SurfaceConfiguration, TextureSampleType, TextureView, TextureViewDimension, VertexState,
    },
    PostProcessor,
};

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const fn vector(self) -> Vector<i32, 2> {
        match self {
            Direction::Up => Vector::<_, 2>::new(0, -1),
            Direction::Down => Vector::<_, 2>::new(0, 1),
            Direction::Left => Vector::<_, 2>::new(-1, 0),
            Direction::Right => Vector::<_, 2>::new(1, 0),
        }
    }

    pub const fn is_horizontal(self) -> bool {
        matches!(self, Direction::Left | Direction::Right)
    }

    pub const fn is_vertical(self) -> bool {
        !self.is_horizontal()
    }
}

// Took the code from near_o11y crate: https://github.com/near/nearcore
pub mod invariants {
    ///
    /// If assert fails, panic on debug, and log error on release
    ///
    #[macro_export]
    macro_rules! assert_log {
        ($cond:expr) => {
            $crate::assert_log!($cond, "assertion failed: {}", stringify!($cond))
        };

        ($cond:expr, $fmt:literal $($arg:tt)*) => {
            if cfg!(debug_assertions) {
                assert!($cond, $fmt $($arg)*);
            } else {
                #[allow(clippy::neg_cmp_op_on_partial_ord)]
                if !$cond {
                    log::error!($fmt $($arg)*);
                }
            }
        };
    }

    #[macro_export]
    macro_rules! assert_log_bail {
        ($cond:expr) => {
            $crate::assert_log!($cond, "assertion failed: {}", stringify!($cond))
        };

        ($cond:expr, $fmt:literal $($arg:tt)*) => {
            if cfg!(debug_assertions) {
                assert!($cond, $fmt $($arg)*);
            } else {
                #[allow(clippy::neg_cmp_op_on_partial_ord)]
                if !$cond {
                    log::error!($fmt $($arg)*);
                    return;
                }
            }
        };
    }

    #[macro_export]
    macro_rules! assert_log_fail {
        ($fmt:literal $($arg:tt)*) => {
            $crate::assert_log!(false, $fmt $($arg)*)
        };
    }
}

pub mod math {
    use std::f32::consts::PI;

    pub const TWO_PI: f32 = 2.0 * PI;
}

pub mod ratatui_buffer_safety {
    use std::panic::panic_any;

    use easy_ext::ext;
    use log::error;
    use ratatui::{buffer::Buffer, layout::Position, style::Style};

    #[ext(BufferExt)]
    pub impl Buffer {
        fn set_cell<P: Into<Position>>(&mut self, position: P, style: Style) {
            let position: Position = position.into();
            if let Some(cell) = self.cell_mut((position.x, position.y)) {
                cell.set_style(style);
            } else {
                let error_message = format!("out of bound access on buffer: tried to get cell ({}, {}) on a buffer with size ({}, {})", position.x, position.y, self.area().width, self.area().height);
                if cfg!(debug_assertions) {
                    panic_any(error_message);
                } else {
                    error!("{error_message}");
                }
            }
        }
    }
}

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Debug, Clone, Copy)]
struct Uniforms {
    screen_size: [f32; 2],
    preserve_aspect: u32,
    use_srgb: u32,
    background_color: [f32; 4],
}

pub struct BackgroundColorEdgesPostProcessor<const PRESERVE_ASPECT: bool = true> {
    uniforms: Buffer,
    bindings: BindGroupLayout,
    sampler: Sampler,
    pipeline: RenderPipeline,
    blitter: ratatui_wgpu::wgpu::RenderBundle,
    bg_color: ratatui::style::Color,
}

impl<const PRESERVE_ASPECT: bool> PostProcessor
    for BackgroundColorEdgesPostProcessor<PRESERVE_ASPECT>
{
    type UserData = ratatui::style::Color;

    fn compile(
        device: &Device,
        text_view: &TextureView,
        surface_config: &SurfaceConfiguration,
        bg_color: Self::UserData,
    ) -> Self {
        let uniforms = device.create_buffer(&BufferDescriptor {
            label: Some("Text Blit Uniforms"),
            size: size_of::<Uniforms>() as u64,
            usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
            mapped_at_creation: false,
        });

        let sampler = device.create_sampler(&SamplerDescriptor {
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Nearest,
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        let layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Text Blit Bindings Layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: NonZeroU64::new(size_of::<Uniforms>() as u64),
                    },
                    count: None,
                },
            ],
        });

        let shader = device.create_shader_module(include_wgsl!("shader/bg-color-border.wgsl"));

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("Text Blit Layout"),
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Text Blitter Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[],
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleStrip,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(ColorTargetState {
                    format: surface_config.format,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
            multiview: None,
            cache: None,
        });

        let blitter = build_blitter(
            device,
            &layout,
            text_view,
            &sampler,
            &uniforms,
            surface_config,
            &pipeline,
        );

        Self {
            uniforms,
            bindings: layout,
            sampler,
            pipeline,
            blitter,
            bg_color,
        }
    }

    fn resize(
        &mut self,
        device: &Device,
        text_view: &TextureView,
        surface_config: &SurfaceConfiguration,
    ) {
        self.blitter = build_blitter(
            device,
            &self.bindings,
            text_view,
            &self.sampler,
            &self.uniforms,
            surface_config,
            &self.pipeline,
        );
    }

    fn process(
        &mut self,
        encoder: &mut CommandEncoder,
        queue: &Queue,
        _text_view: &TextureView,
        surface_config: &SurfaceConfiguration,
        surface_view: &TextureView,
    ) {
        {
            let mut uniforms = queue
                .write_buffer_with(
                    &self.uniforms,
                    0,
                    NonZeroU64::new(size_of::<Uniforms>() as u64).unwrap(),
                )
                .unwrap();
            let ratatui::style::Color::Rgb(r, g, b) = self.bg_color else {
                panic!("Only rgb color are supported");
            };
            let col_comp_to_f32 = |comp: u8| comp as f32 / 255.0;
            uniforms.copy_from_slice(bytemuck::bytes_of(&Uniforms {
                screen_size: [surface_config.width as f32, surface_config.height as f32],
                preserve_aspect: u32::from(PRESERVE_ASPECT),
                use_srgb: u32::from(surface_config.format.is_srgb()),
                background_color: [
                    col_comp_to_f32(r),
                    col_comp_to_f32(g),
                    col_comp_to_f32(b),
                    1.0,
                ],
            }));
        }

        let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Text Blit Pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: surface_view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::TRANSPARENT),
                    store: StoreOp::Store,
                },
            })],
            ..Default::default()
        });

        pass.execute_bundles(Some(&self.blitter));
    }
}

fn build_blitter(
    device: &Device,
    layout: &BindGroupLayout,
    text_view: &TextureView,
    sampler: &Sampler,
    uniforms: &Buffer,
    surface_config: &SurfaceConfiguration,
    pipeline: &RenderPipeline,
) -> RenderBundle {
    let bindings = device.create_bind_group(&BindGroupDescriptor {
        label: Some("Text Blit Bindings"),
        layout,
        entries: &[
            BindGroupEntry {
                binding: 0,
                resource: BindingResource::TextureView(text_view),
            },
            BindGroupEntry {
                binding: 1,
                resource: BindingResource::Sampler(sampler),
            },
            BindGroupEntry {
                binding: 2,
                resource: uniforms.as_entire_binding(),
            },
        ],
    });

    let mut encoder = device.create_render_bundle_encoder(&RenderBundleEncoderDescriptor {
        label: Some("Text Blit Pass Encoder"),
        color_formats: &[Some(surface_config.format)],
        depth_stencil: None,
        sample_count: 1,
        multiview: None,
    });

    encoder.set_pipeline(pipeline);

    encoder.set_bind_group(0, &bindings, &[]);
    encoder.draw(0..3, 0..1);

    encoder.finish(&RenderBundleDescriptor {
        label: Some("Text Blit Pass Bundle"),
    })
}
