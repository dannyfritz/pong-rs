use GlFrame;
use Vertex;
use glium::{self, Surface};
use nalgebra::{Matrix4, Vector3};
use pong_lib::{game, Sound};
use rodio;
use rodio::Source;
use std::fs::File;
use std::io::BufReader;

pub fn render_game_sound(state: &mut game::State, audio_device: &rodio::Device) {
    for sound in state.sounds.drain(0..) {
        match sound {
            Sound::Bounce => {
                let file = File::open("./assets/sounds/bounce.wav").unwrap();
                let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                rodio::play_raw(audio_device, source.convert_samples());
            }
            Sound::Score => {
                let file = File::open("./assets/sounds/score.wav").unwrap();
                let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
                rodio::play_raw(audio_device, source.convert_samples());
            }
        }
    }
}

pub fn render_game_graphics(
    state: &game::State,
    gl_parts: &mut GlFrame,
    projection: Matrix4<f32>,
    view: Matrix4<f32>,
) {
    render_paddle(&state.players[0].paddle, gl_parts, projection, view);
    render_paddle(&state.players[1].paddle, gl_parts, projection, view);
    if let Some(ref ball) = state.ball {
        render_ball(ball, gl_parts, projection, view);
    }
}

fn render_paddle(
    paddle: &game::paddle::Paddle,
    gl_parts: &mut GlFrame,
    projection: Matrix4<f32>,
    view: Matrix4<f32>,
) {
    let mut model = Matrix4::<f32>::identity();
    let shape = {
        let x = paddle.pos.x;
        let y = paddle.pos.y;
        let w = paddle.dim.x;
        let h = paddle.dim.y;
        model.append_translation_mut(&Vector3::new(x, y, 0.0));
        vec![
            Vertex {
                position: [0.0, 0.0],
            },
            Vertex { position: [w, 0.0] },
            Vertex { position: [w, -h] },
            Vertex {
                position: [0.0, -h],
            },
        ]
    };
    let vertex_buffer = glium::VertexBuffer::new(gl_parts.display, &shape).unwrap();
    let indices = glium::index::IndexBuffer::new(
        gl_parts.display,
        glium::index::PrimitiveType::TrianglesList,
        &vec![0u16, 1, 2, 0, 2, 3],
    ).unwrap();
    let uniforms = {
        let projection_slice: [[f32; 4]; 4] = projection.into();
        let view_slice: [[f32; 4]; 4] = view.into();
        let model_slice: [[f32; 4]; 4] = model.into();
        uniform! {
            projection: projection_slice,
            view: view_slice,
            model: model_slice,
        }
    };
    gl_parts
        .target
        .draw(
            &vertex_buffer,
            &indices,
            &gl_parts.program,
            &uniforms,
            &Default::default(),
        )
        .unwrap();
}

fn render_ball(
    ball: &game::ball::Ball,
    gl_parts: &mut GlFrame,
    projection: Matrix4<f32>,
    view: Matrix4<f32>,
) {
    let mut model = Matrix4::<f32>::identity();
    let shape = {
        let x = ball.pos.x;
        let y = ball.pos.y;
        let r = ball.r;
        let d = r * 2.0;
        model.append_translation_mut(&Vector3::new(x - r, y - r, 0.0));
        vec![
            Vertex {
                position: [0.0, 0.0],
            },
            Vertex { position: [d, 0.0] },
            Vertex { position: [d, d] },
            Vertex { position: [0.0, d] },
        ]
    };
    let vertex_buffer = glium::VertexBuffer::new(gl_parts.display, &shape).unwrap();
    let indices = glium::index::IndexBuffer::new(
        gl_parts.display,
        glium::index::PrimitiveType::TrianglesList,
        &vec![0u16, 1, 2, 0, 2, 3],
    ).unwrap();
    let uniforms = {
        let projection_slice: [[f32; 4]; 4] = projection.into();
        let view_slice: [[f32; 4]; 4] = view.into();
        let model_slice: [[f32; 4]; 4] = model.into();
        uniform! {
            projection: projection_slice,
            view: view_slice,
            model: model_slice,
        }
    };
    gl_parts
        .target
        .draw(
            &vertex_buffer,
            &indices,
            &gl_parts.program,
            &uniforms,
            &Default::default(),
        )
        .unwrap();
}
