#[macro_use]
extern crate glium;

use glium::{Display, Surface, VertexBuffer};
use glium::draw_parameters::DrawParameters;
use glium::glutin::{
    ContextBuilder, ControlFlow, ElementState, Event, EventsLoop, MouseButton, MouseScrollDelta,
    TouchPhase, VirtualKeyCode, WindowBuilder, WindowEvent
};
use glium::index::{NoIndices, PrimitiveType};

#[derive(Copy, Clone)]
struct Vertex {
    xy: [f32; 2],
}

impl Vertex {
    fn new(x: f32, y: f32) -> Self {
        Self {
            xy: [x, y],
        }
    }
}

implement_vertex!(Vertex, xy);

const VERTEX_SHADER: &str = include_str!("mandelbrot.glslv");
const FRAGMENT_SHADER: &str = include_str!("mandelbrot.glslf");

fn main() {
    let width = 512;
    let height = 512;
    let mut ev = EventsLoop::new();
    let window = WindowBuilder::new()
        .with_dimensions(width, height);

    let context = ContextBuilder::new();

    let display = Display::new(window, context, &ev).unwrap();

    let vertices = [
        // Top-left vertex.
        Vertex::new(-1.0,  1.0),
        Vertex::new( 1.0,  1.0),
        Vertex::new(-1.0, -1.0),

        // Bottom-right vertex.
        Vertex::new(-1.0, -1.0),
        Vertex::new( 1.0,  1.0),
        Vertex::new( 1.0, -1.0),
    ];

    let program = glium::program::Program::from_source(
        &display,
        VERTEX_SHADER,
        FRAGMENT_SHADER,
        None
    ).unwrap();

    let vbuf = VertexBuffer::new(&display, &vertices).unwrap();

    let mut zoom = 0.5f64;
    let mut ratio = width as f64 / height as f64;
    let mut mx = 1.0f64;
    let mut my = 1.0f64;

    let mut current_button = None;
    let mut prev_position = (0f64, 0f64);
    let mut current_position = (0f64, 0f64);

    ev.run_forever(|event| {
        let mut frame = display.draw();

        let control = match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::KeyboardInput { input, .. } => {
                        match input.virtual_keycode {
                            Some(VirtualKeyCode::R) => {
                                zoom = 0.5;
                                ratio = width as f64 / height as f64;
                                mx = 1.0;
                                my = 1.0;
                            }
                            Some(..) | None => {}
                        }

                        ControlFlow::Continue
                    }
                    WindowEvent::MouseWheel { delta: MouseScrollDelta::PixelDelta(.., dy), phase, .. } => {
                        match phase {
                            TouchPhase::Moved => {
                                let dy = -dy;
                                let scale = 1. + 0.1 * (dy as f64 / height as f64);
                                zoom *= scale;
                            }
                            _ => {}
                        }
                        ControlFlow::Continue
                    }
                    WindowEvent::MouseInput { state: ElementState::Pressed, button, .. } => {
                        current_button = Some(button);
                        ControlFlow::Continue
                    }
                    WindowEvent::MouseInput { state: ElementState::Released, .. } => {
                        current_button = None;
                        ControlFlow::Continue
                    }
                    WindowEvent::MouseMoved { position, .. } => {
                        prev_position = current_position;
                        current_position = position;
                        ControlFlow::Continue
                    }
                    WindowEvent::MouseLeft { .. } => {
                        current_button = None;
                        ControlFlow::Continue
                    }
                    WindowEvent::Closed => ControlFlow::Break,
                    _ => {
                        ControlFlow::Continue
                    }
                }
            }
            _ => {
                ControlFlow::Continue
            }
        };

        match current_button {
            Some(MouseButton::Left) => {
                let dx = current_position.0 - prev_position.0;
                let dy = current_position.1 - prev_position.1;

                let dx = if dx.abs() <= 1.0 {
                    0.0
                } else {
                    dx / height as f64
                };

                let dy = if dy.abs() <= 1.0 {
                    0.0
                } else {
                    dy / height as f64
                };

                mx -= 0.5 * dx / zoom;
                my -= 0.5 * dy / zoom;
            }
            Some(..) | None => {}
        }


        let indices = NoIndices(PrimitiveType::TrianglesList);
        let uniform = uniform! {
            matrix: [
                [-1.0 / zoom as f32,  0.0,                         0.0, 0.0],
                [ 0.0,                1.0 / (zoom * ratio) as f32, 0.0, 0.0],
                [ 0.0,                0.0,                         1.0, 0.0],
                [-mx as f32,         -my as f32,                   0.0, 1.0],
            ],
        };
        let options = DrawParameters {
            .. Default::default()
        };

        frame.draw(&vbuf, &indices, &program, &uniform, &options).unwrap();
        frame.finish().unwrap();
        control
    });
}
