#[macro_use]
extern crate glium;
extern crate nalgebra;
extern crate pong_lib;
extern crate time;

use glium::{glutin, Surface};
use nalgebra::Matrix4;
use pong_lib::{Intents, PongScene, PongState};
use pong_lib::game;
// use pong_lib::menu::Menu;

#[derive(Debug)]
struct Timer {
    dt: f64,
    current_time: f64,
    accumulator: f64,
}

impl Timer {
    fn new(dt: f64) -> Timer {
        Timer {
            dt: dt,
            current_time: time::precise_time_s(),
            accumulator: 0.0,
        }
    }
}

struct GlParts<'a> {
    display: &'a glium::Display,
    target: &'a mut glium::Frame,
    program: &'a glium::Program,
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
    let window = glutin::WindowBuilder::new().with_dimensions(640, 320);
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
        let mut target = display.draw();
        let mut gl_parts = GlParts {
            display: &display,
            target: &mut target,
            program: &program,
        };
        gl_parts.target.clear_color(0.0, 0.0, 1.0, 1.0);
        match pong_state.scene {
            PongScene::Game(ref mut state) => render_game(&state, &mut gl_parts, projection, view),
            PongScene::Menu(ref mut _state) => {}
        };
        gl_parts.target.finish().unwrap();
        // let ten_millis = Duration::from_millis(10);
        // thread::sleep(ten_millis);
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

fn render_game(
    state: &game::State,
    gl_parts: &mut GlParts,
    projection: Matrix4<f32>,
    view: Matrix4<f32>,
) {
    render_paddle(&state.players[0].paddle, gl_parts, projection, view);
    render_paddle(&state.players[1].paddle, gl_parts, projection, view);
}

fn render_paddle(
    paddle: &game::paddle::Paddle,
    gl_parts: &mut GlParts,
    projection: Matrix4<f32>,
    view: Matrix4<f32>,
) {
    let projection_slice: [[f32; 4]; 4] = projection.into();
    let view_slice: [[f32; 4]; 4] = view.into();
    let model = Matrix4::<f32>::identity();
    let model_slice: [[f32; 4]; 4] = model.into();
    let shape = {
        let x = paddle.x;
        let y = paddle.y;
        let w = paddle.width;
        let h = paddle.height;
        vec![
            Vertex { position: [x, y] },
            Vertex {
                position: [x + w, y],
            },
            Vertex {
                position: [x + w, y + h],
            },
            Vertex {
                position: [x, y + h],
            },
        ]
    };
    let vertex_buffer = glium::VertexBuffer::new(gl_parts.display, &shape).unwrap();
    let indices = vec![0u16, 1, 2, 0, 2, 3];
    let indices = glium::index::IndexBuffer::new(
        gl_parts.display,
        glium::index::PrimitiveType::TrianglesList,
        &indices,
    ).unwrap();
    let uniforms = uniform! {
        projection: projection_slice,
        view: view_slice,
        model: model_slice,
    };
    gl_parts
        .target
        .draw(
            &vertex_buffer,
            &indices,
            gl_parts.program,
            &uniforms,
            &Default::default(),
        )
        .unwrap();
}

fn get_file_string(file_name: &str) -> String {
    use std::fs::File;
    use std::io::prelude::*;
    let mut file = File::open(file_name).unwrap();
    let mut file_src = String::new();
    file.read_to_string(&mut file_src).unwrap();
    file_src
}
