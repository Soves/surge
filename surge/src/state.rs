use winit::{
    event::*
};

use winit::window::Window;

pub struct State
{
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pub size: winit::dpi::PhysicalSize<u32>
}

impl State
{
    // Creating some of the wgpu types requires async code
    pub async fn new(window: &Window) -> Self{
        let size = window.inner_size();

        //instance is a handle to gpu
        //backeds is the graphics api
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) }; //window surface

        //adapter is a handle to the actual gpu
        //can be used to retrieve info about the gpu
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                //default power preference
                //dictated by gpu power setting?
                power_preference: wgpu::PowerPreference::default(),

                //idk
                compatible_surface: Some(&surface),

                //if we want to software render as a fallback to no compatitible device
                force_fallback_adapter: false,
            },
        ).await.unwrap();


        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor { //used to speciy wgpu features
                features: wgpu::Features::empty(),
                // WebGL doesn't support all of wgpu's features, so if
                // we're building for the web we'll have to disable some.
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        ).await.unwrap();

        //config for window surface
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,  //specifies the surface is used to render to the screen
            format: surface.get_preferred_format(&adapter).unwrap(), //uses preferred format depending on display
            width: size.width, //window surface resolution
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo, //deptermines how to sync surface with display aka vsync
        };
        surface.configure(&device, &config);


        Self { //finally create the struct
            surface,
            device,
            queue,
            config,
            size,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        
        //resize window
        if new_size.width > 0 && new_size.height > 0 //make sure its a nonzero size
        {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config); //resize surface
        }

    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    pub fn update(&mut self) {
        
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        
        let output = self.surface.get_current_texture()?;

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
        }
    
        // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    
        Ok(())
    }
}