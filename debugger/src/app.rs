use glium::Surface;
use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextAttributesBuilder, NotCurrentGlContext},
    display::{GetGlDisplay, GlDisplay},
    surface::{SurfaceAttributesBuilder, WindowSurface},
};
use std::fs;
use std::num::NonZeroU32;

use imgui_winit_support::winit::{dpi::LogicalSize, event_loop::EventLoop, window::WindowBuilder};
use raw_window_handle::HasRawWindowHandle;
use winit::{
    event::{Event, WindowEvent},
    window::Window,
};

use crate::{emulator::Emulator, ui::DebuggerUI};

pub struct App {
    width: u32,
    height: u32,
}
impl App {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
    pub fn run_app(&mut self, title: &str) {
        let (event_loop, window, mut display) = self.create_window(title);
        let (mut winit_platform, mut imgui_context) = App::imgui_init(&window);

        let mut renderer = imgui_glium_renderer::Renderer::init(&mut imgui_context, &display)
            .expect("Failed to initialize renderer");

        let bios: Box<[u8]> = fs::read("./binaries/SCPH1001.BIN").unwrap().into_boxed_slice();
        let mut emu = Emulator::new(bios);

        let mut debugger_ui = DebuggerUI::new(&mut renderer, &mut display);

        event_loop
            .run(move |event, window_target| {
                match event {
                    // Triggered before waiting for events, preparing the frame here
                    Event::AboutToWait => {
                        // Prepare the frame for ImGui
                        winit_platform
                            .prepare_frame(imgui_context.io_mut(), &window)
                            .expect("Failed to prepare frame");

                        // Request to redraw the window
                        window.request_redraw();
                    }

                    // Event triggered when the window needs to redraw the UI
                    Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                        // Start ImGui frame rendering
                        let ui = imgui_context.frame();

                        // Only run emulator logic if it's running
                        if emu.running {
                            //println!("Running frame with PC at: 0x{:08X}", emu.ps.cpu.pc);
                            emu.run();
                        }

                        // Render the debugger UI and pass it the current emulator state and the UI context
                        debugger_ui.render_ui(&mut emu, &ui, &mut renderer);

                        // Prepare the Glium rendering target
                        let mut target = display.draw();
                        target.clear_color_srgb(0.1, 0.1, 0.15, 1.0);

                        // Render the ImGui draw data
                        let draw_data = imgui_context.render();
                        renderer.render(&mut target, draw_data).expect("Rendering Failed!");

                        // Finish drawing and swap buffers
                        target.finish().expect("Failed to swap buffers");
                    }

                    // Event triggered when the window is resized
                    Event::WindowEvent { event: WindowEvent::Resized(new_size), .. } => {
                        // Adjust the display size only if the new size is valid
                        if new_size.width > 0 && new_size.height > 0 {
                            display.resize((new_size.width, new_size.height));
                        }

                        // Handle the window event in ImGui
                        winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
                    }

                    // Event triggered when the window is closed
                    Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                        window_target.exit(); // Exiting the application
                    }

                    // Handle other events, logging them for debugging
                    event => {
                        winit_platform.handle_event(imgui_context.io_mut(), &window, &event);
                    }
                }
            })
            .expect("EventLoop error");
    }
    fn create_window(&self, title: &str) -> (EventLoop<()>, Window, glium::Display<WindowSurface>) {
        let event_loop = EventLoop::new().expect("Failed to create EventLoop");

        let window_builder = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(self.width, self.height));

        let (window, cfg) = glutin_winit::DisplayBuilder::new()
            .with_window_builder(Some(window_builder))
            .build(&event_loop, ConfigTemplateBuilder::new(), |mut configs| configs.next().unwrap())
            .expect("Failed to create OpenGL window");
        let window = window.unwrap();

        let context_attribs =
            ContextAttributesBuilder::new().build(Some(window.raw_window_handle()));
        let context = unsafe {
            cfg.display()
                .create_context(&cfg, &context_attribs)
                .expect("Failed to create OpenGL context")
        };

        let surface_attribs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            window.raw_window_handle(),
            NonZeroU32::new(self.width).unwrap(),
            NonZeroU32::new(self.height).unwrap(),
        );
        let surface = unsafe {
            cfg.display()
                .create_window_surface(&cfg, &surface_attribs)
                .expect("Failed to create OpenGL surface")
        };

        let context =
            context.make_current(&surface).expect("Failed to make OpenGL context current");

        let display = glium::Display::from_context_surface(context, surface)
            .expect("Failed to create glium Display");

        (event_loop, window, display)
    }

    fn imgui_init(window: &Window) -> (imgui_winit_support::WinitPlatform, imgui::Context) {
        let mut imgui_context = imgui::Context::create();
        imgui_context.set_ini_filename(None);

        let mut winit_platform = imgui_winit_support::WinitPlatform::init(&mut imgui_context);
        winit_platform.attach_window(
            imgui_context.io_mut(),
            window,
            imgui_winit_support::HiDpiMode::Default,
        );

        imgui_context.fonts().add_font(&[imgui::FontSource::DefaultFontData {
            config: Some(imgui::FontConfig { size_pixels: 14.0, ..Default::default() }),
        }]);

        (winit_platform, imgui_context)
    }
}
