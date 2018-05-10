#[macro_use]
extern crate glium;
extern crate nalgebra;
extern crate pong_lib;
extern crate time;

mod game;
mod timer;

use game::render_game;
use glium::{Surface, glutin::{self, KeyboardInput}};
use nalgebra::Matrix4;
use pong_lib::{Intents, PongScene, PongState};
use timer::Timer;
// use pong_lib::menu::Menu;

pub struct GlFrame<'a> {
    display: &'a glium::Display,
    target: glium::Frame,
    program: &'a glium::Program,
}

impl<'a> GlFrame<'a> {
    fn new(display: &'a glium::Display, program: &'a glium::Program) -> GlFrame<'a> {
        GlFrame {
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

fn main() -> Result<(), std::io::Error> {
    let window = glutin::WindowBuilder::new()
        .with_dimensions(640, 320)
        .with_title("pong-rs");
    let context = glutin::ContextBuilder::new();
    let mut events_loop = glutin::EventsLoop::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let program = {
        glium::Program::from_source(
            &display,
            std::fs::read_to_string("./pong_fe_gl/shaders/simple.vert")?.as_str(),
            std::fs::read_to_string("./pong_fe_gl/shaders/simple.frag")?.as_str(),
            None,
        ).unwrap()
    };
    let mut timer = Timer::new(1.0 / 10.0);
    let projection = Matrix4::<f32>::new_orthographic(0.0, 64.0, 0.0, 32.0, -1.0, 1.0);
    let view = Matrix4::<f32>::identity();
    let mut pong_state = PongState::new();
    let mut intents = Intents {
        menu_up: false,
        menu_down: false,
        menu_select: false,
        paddle_1_up: 0,
        paddle_1_down: 0,
        paddle_2_up: 0,
        paddle_2_down: 0,
    };
    let mut closed = false;
    while !closed {
        timer.tick();
        while timer.drain() {
            pong_state.interpolate(timer.dt, &intents);
        }
        let mut gl_frame = GlFrame::new(&display, &program);
        gl_frame.target.clear_color(0.02, 0.02, 0.01, 1.0);
        match pong_state.scene {
            PongScene::Game(ref mut state) => render_game(&state, &mut gl_frame, projection, view),
            PongScene::Menu(ref mut _state) => {}
        };
        gl_frame.present_frame().unwrap();
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => closed = true,
                glutin::WindowEvent::KeyboardInput { input, .. } => {
                    handle_keyboard_input(&mut intents, input);
                }
                _ => {}
            },
            _ => {}
        });
    }
    Ok(())
}

fn handle_keyboard_input(intents: &mut Intents, input: KeyboardInput) {
    use glutin::ElementState::Pressed;
    match input {
        KeyboardInput {
            scancode: 17,
            state,
            ..
        } => {
            intents.paddle_1_up = bool_to_u8(state == Pressed);
        }
        KeyboardInput {
            scancode: 31,
            state,
            ..
        } => {
            intents.paddle_1_down = bool_to_u8(state == Pressed);
        }
        KeyboardInput {
            scancode: 72,
            state,
            ..
        } => {
            intents.paddle_2_up = bool_to_u8(state == Pressed);
        }
        KeyboardInput {
            scancode: 80,
            state,
            ..
        } => {
            intents.paddle_2_down = bool_to_u8(state == Pressed);
        }
        _ => {}
    };
}
