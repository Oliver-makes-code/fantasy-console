#![feature(stmt_expr_attributes, decl_macro, let_chains)]
use std::{
    thread::sleep,
    time::{Duration, Instant},
};

use egui_glium::egui_winit::egui;
use egui_glium::{egui_winit::egui::ViewportId, EguiGlium};
use frame::{FrameBuffer, HEIGHT, WIDTH};
use gamepad::GamepadStateManager;
use glium::{
    backend::glutin::SimpleWindowBuilder, glutin::surface::WindowSurface, implement_vertex,
    index::NoIndices, uniform, Display, Program, Surface, VertexBuffer,
};
use tile::{Sprite, TileState};
use wasm::WasmCart;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{EventLoop, EventLoopWindowTarget},
    window::Window,
};

pub mod color;
pub mod frame;
pub mod gamepad;
pub mod math;
pub mod tile;
pub mod wasm;

const VERTEX_SHADER: &str = include_str!("./shaders/vert.glsl");

const FRAGMENT_SHADER: &str = include_str!("./shaders/frag.glsl");

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    position: (f32, f32),
    uv: (f32, f32),
}
implement_vertex!(Vertex, position location(0), uv location(1));

pub struct WindowState {
    pub window: Window,
    pub display: Display<WindowSurface>,
    pub frame: FrameBuffer,
    pub vertex_buffer: VertexBuffer<Vertex>,
    pub indices: NoIndices,
    pub shaders: Program,
    pub egui: EguiGlium,
}

impl WindowState {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let (window, display) = SimpleWindowBuilder::new()
            .with_title("owo")
            .with_inner_size(WIDTH as u32, HEIGHT as u32)
            .build(event_loop);

        let frame = FrameBuffer::new();

        let shape = vec![
            Vertex {
                position: (-1., 1.),
                uv: (0., 1.),
            },
            Vertex {
                position: (1., 1.),
                uv: (1., 1.),
            },
            Vertex {
                position: (-1., -1.),
                uv: (0., 0.),
            },
            Vertex {
                position: (1., -1.),
                uv: (1., 0.),
            },
        ];

        let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

        let shaders =
            glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

        let egui = egui_glium::EguiGlium::new(ViewportId::ROOT, &display, &window, &event_loop);

        Self {
            window,
            display,
            frame,
            vertex_buffer,
            indices,
            shaders,
            egui,
        }
    }

    pub fn run_event_loop(mut self, event_loop: EventLoop<()>) {
        self.init();

        event_loop
            .run(|event, window_target| self.event_handler(event, window_target))
            .unwrap();
    }

    fn event_handler(&mut self, event: Event<()>, window_target: &EventLoopWindowTarget<()>) {
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    // This event is sent by the OS when you close the Window, or request the program to quit via the taskbar.
                    WindowEvent::CloseRequested => window_target.exit(),
                    WindowEvent::RedrawRequested => self.render(),
                    winit::event::WindowEvent::Resized(window_size) => {
                        self.display.resize(window_size.into());
                    }
                    _ => (),
                }
                let _ = self.egui.on_event(&self.window, &event);
            }
            winit::event::Event::AboutToWait => {
                self.window.request_redraw();
            }
            _ => (),
        };
    }

    fn init(&mut self) {
        WasmCart::init();
    }

    fn update(&mut self) {
        GamepadStateManager::update();

        WasmCart::update();
    }

    fn draw_frame(&mut self) {
        for y in 0..HEIGHT {
            WasmCart::v_blank(y as u32);
            let tile_state = TileState::get();
            let mut sprites: Vec<Sprite> = Vec::with_capacity(64);
            for sprite in tile_state.sprites {
                if (y as isize) >= sprite.position.1 as isize
                    && (y as isize) < sprite.position.1 as isize + 16
                    && sprite.visible
                {
                    sprites.push(sprite);
                }
            }
            sprites.sort_by(|a, b| a.position.cmp(&b.position));
            for x in 0..WIDTH {
                let mut color = tile_state.palette[tile_state.background_color as usize];
                for i in 0..8 {
                    if !tile_state.backgrounds[i].visible {
                        continue;
                    }
                    let (palette_offset, palette) =
                        tile_state.backgrounds[i].get_color_offset(&tile_state, x, y);
                    if palette_offset != 0 {
                        color = tile_state.palette[palette as usize + palette_offset as usize - 1];
                        break;
                    }
                }
                for sprite in sprites.iter() {
                    if (x as isize) >= sprite.position.0 as isize
                        && (x as isize) < sprite.position.0 as isize + 16
                    {
                        let (palette_offset, palette) = sprite.get_color_offset(&tile_state, x, y);
                        if palette_offset != 0 {
                            color =
                                tile_state.palette[palette as usize + palette_offset as usize - 1];
                            break;
                        }
                    }
                }
                self.frame.write_pixel(x, y, color);
            }
        }
    }

    fn render(&mut self) {
        let start = Instant::now();
        self.update();
        self.draw_frame();

        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(
            self.frame.data(),
            (WIDTH as u32, HEIGHT as u32),
        );
        let texture = glium::texture::Texture2d::new(&self.display, image).unwrap();

        let width = self.window.inner_size().width;
        let height = self.window.inner_size().height;

        let uniforms = uniform! {
            sam: texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest).wrap_function(glium::uniforms::SamplerWrapFunction::Clamp),
            d_width: width,
            d_height: height
        };

        let mut target = self.display.draw();
        target
            .draw(
                &self.vertex_buffer,
                &self.indices,
                &self.shaders,
                &uniforms,
                &Default::default(),
            )
            .unwrap();

        self.egui.run(&self.window, |egui_ctx| {
            egui::Window::new("Background State")
                .resizable(true)
                .max_width(1024.)
                .show(egui_ctx, |ui| {
                    let backgrounds = &TileState::get().backgrounds;
                    for i in 0..8 {
                        let bg = backgrounds[i];
                        ui.heading(format!("BG{}", i));
                        ui.label(format!("Pre-scroll: {:?}", bg.pre_offset));
                        ui.label(format!("Post-scroll: {:?}", bg.post_offset));
                        ui.label(format!(
                            "Matrix: ({}, {}, {}, {})",
                            bg.matrix.0 .0, bg.matrix.0 .1, bg.matrix.1 .0, bg.matrix.1 .1
                        ));
                    }
                });
        });
        self.egui.paint(&self.display, &mut target);
        target.finish().unwrap();

        let elapsed = start.elapsed();

        if elapsed.as_millis() >= 32 {
            return;
        }

        sleep(Duration::from_millis(32) - elapsed)
    }
}
