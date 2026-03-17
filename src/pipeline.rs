use crate::objects::scene::Scene;
use crate::objects::vertex::Vertex;
use egui_wgpu::wgpu;
use egui_wgpu::wgpu::util::DeviceExt;

pub struct ModelRenderData {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

pub struct SceneRenderData {
    pub models: Vec<ModelRenderData>,
}

pub fn create_pipeline(
    device: &wgpu::Device,
    color_format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Triangle Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });

    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Scene Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Scene Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[Vertex::desc()],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format: color_format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
        cache: None,
    })
}

pub fn create_scene_render_data(device: &wgpu::Device, scene: &Scene) -> SceneRenderData {
    let mut models = Vec::new();

    for group in &scene.groups {
        for model in &group.models {
            let indices = model.indices();

            let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&model.vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });

            let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage: wgpu::BufferUsages::INDEX,
            });

            models.push(ModelRenderData {
                vertex_buffer,
                index_buffer,
                num_indices: indices.len() as u32,
            });
        }
    }

    SceneRenderData { models }
}

pub fn draw_scene_pass(
    encoder: &mut wgpu::CommandEncoder,
    surface_view: &wgpu::TextureView,
    render_pipeline: &wgpu::RenderPipeline,
    scene: &Scene,
    scene_render_data: &SceneRenderData,
) {
    let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("Triangle Render Pass"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view: surface_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color {
                    r: 0.1,
                    g: 0.2,
                    b: 0.3,
                    a: 1.0,
                }),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
    });

    render_pass.set_pipeline(render_pipeline);

    let mut render_data_iter = scene_render_data.models.iter();
    for group in &scene.groups {
        for _model in &group.models {
            if let Some(model_render_data) = render_data_iter.next() {
                render_pass.set_vertex_buffer(0, model_render_data.vertex_buffer.slice(..));
                render_pass.set_index_buffer(
                    model_render_data.index_buffer.slice(..),
                    wgpu::IndexFormat::Uint16,
                );
                render_pass.draw_indexed(0..model_render_data.num_indices, 0, 0..1);
            }
        }
    }
}
