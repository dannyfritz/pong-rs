#[macro_use]
extern crate glium;
extern crate nalgebra;
extern crate pong_lib;
extern crate time;

mod timer;
mod game;

use glium::{glutin, Surface};
use nalgebra::Matrix4;
use pong_lib::{Intents, PongScene, PongState};
use timer::Timer;
use game::render_game;
// use pong_lib::menu::Menu;

pub struct GlParts<'a> {
    display: &'a glium::Display,
    target: glium::Frame,
    program: &'a glium::Program,
}

impl<'a> GlParts<'a> {
    fn new(display: &'a glium::Display, program: &'a glium::Program) -> GlParts<'a> {
        GlParts {
            display,
            target: display.draw(),
            program,
        }
    }
    fn present_frame(self) -> Result<(), glium::SwapBuffersError> {
        self.target.finish()
    }
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn bool_to_u8(b: bool) -> u8 {
    if b {
        u8::max_value()
    } else {
        u8::min_value()
    }
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(640, 320)
        .with_title("pong-rs");
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let program = glium::Program::from_source(
        &display,
        get_file_string("./pong_fe_gl/shaders/simple.vert").as_str(),
        get_file_string("./pong_fe_gl/shaders/simple.frag").as_str(),
        None,
    ).unwrap();
    let projection = Matrix4::<f32>::new_orthographic(0.0, 64.0, 0.0, 32.0, -1.0, 1.0);
    let view = Matrix4::<f32>::identity();
    let mut pong_state = PongState::new();
    let mut timer = Timer::new(1.0 / 10.0);
    let mut closed = false;
    let mut intents = Intents {
        menu_up: false,
        menu_down: false,
        menu_select: false,
        paddle_1_up: 0,
        paddle_1_down: 0,
        paddle_2_up: 0,
        paddle_2_down: 0,
    };
    while !closed {
        let new_time = time::precise_time_s();
        let frame_time = new_time - timer.current_time;
        timer.current_time = new_time;
        timer.accumulator += frame_time;
        while timer.accumulator >= timer.dt {
            pong_state.interpolate(timer.dt, &intents);
            timer.accumulator -= timer.dt;
        }
        let mut gl_parts = GlParts::new(&display, &program);
        gl_parts.target.clear_color(0.02, 0.02, 0.01, 1.0);
        match pong_state.scene {
            PongScene::Game(ref mut state) => render_game(&state, &mut gl_parts, projection, view),
            PongScene::Menu(ref mut _state) => {}
        };
        gl_parts.present_frame().unwrap();
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => closed = true,
                glutin::WindowEvent::KeyboardInput { input, .. } => {
                    use glutin::ElementState::Pressed;
                    // println!("{:?}", input);
                    match input.scancode {
                        72 => {
                            intents.paddle_1_up = bool_to_u8(input.state == Pressed);
                        }
                        80 => {
                            intents.paddle_1_down = bool_to_u8(input.state == Pressed);
                        }
                        _ => {}
                    };
                }
                _ => {}
            },
            _ => (),
        });
    }
}

fn get_file_string(file_name: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::open(file_name).unwrap();
    let mut file_src = String::new();
    file.read_to_string(&mut file_src).unwrap();
    file_src
}
