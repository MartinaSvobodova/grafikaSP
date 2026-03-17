use crate::egui_tools::EguiRenderer;
use crate::objects::model_group::ModelGroup;
use crate::objects::scene::Scene;
use crate::pipeline;
use crate::shapes;
use egui_wgpu::wgpu;
use winit::window::Window;

pub struct AppState {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface<'static>,
    pub render_pipeline: wgpu::RenderPipeline,
    pub scene: Scene,
    pub scene_render_data: pipeline::SceneRenderData,
    pub scale_factor: f32,
    pub egui_renderer: EguiRenderer,
}

impl AppState {
    pub async fn new(
        instance: &wgpu::Instance,
        surface: wgpu::Surface<'static>,
        window: &Window,
        width: u32,
        height: u32,
    ) -> Self {
        let power_pref = wgpu::PowerPreference::default();
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: power_pref,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let features = wgpu::Features::empty();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: features,
                required_limits: Default::default(),
                memory_hints: Default::default(),
                trace: Default::default(),
            })
            .await
            .expect("Failed to create device");

        let swapchain_capabilities = surface.get_capabilities(&adapter);
        let selected_format = wgpu::TextureFormat::Bgra8UnormSrgb;
        let swapchain_format = swapchain_capabilities
            .formats
            .iter()
            .find(|d| **d == selected_format)
            .expect("failed to select proper surface texture format!");

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: *swapchain_format,
            width,
            height,
            present_mode: wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: 0,
            alpha_mode: swapchain_capabilities.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &surface_config);

        let render_pipeline = pipeline::create_pipeline(&device, surface_config.format);
        let pentagon = shapes::pentagon::model();
        let default_group = ModelGroup::with_models("Default", vec![pentagon]);
        let scene = Scene::with_groups(vec![default_group]);
        let scene_render_data = pipeline::create_scene_render_data(&device, &scene);
        let egui_renderer = EguiRenderer::new(&device, surface_config.format, None, 1, window);

        Self {
            device,
            queue,
            surface,
            surface_config,
            render_pipeline,
            scene,
            scene_render_data,
            egui_renderer,
            scale_factor: 1.0,
        }
    }

    pub fn resize_surface(&mut self, width: u32, height: u32) {
        self.surface_config.width = width;
        self.surface_config.height = height;
        self.surface.configure(&self.device, &self.surface_config);
    }
}
