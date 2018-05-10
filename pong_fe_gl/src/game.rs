use GlFrame;
use Vertex;
use glium::{self, Surface};
use nalgebra::{Matrix4, Vector3};
use pong_lib::game;

pub fn render_game(
    state: &game::State,
    gl_parts: &mut GlFrame,
    projection: Matrix4<f32>,
    view: Matrix4<f32>,
) {
    render_paddle(&state.players[0].paddle, gl_parts, projection, view);
    render_paddle(&state.players[1].paddle, gl_parts, projection, view);
}

fn render_paddle(
    paddle: &game::paddle::Paddle,
    gl_parts: &mut GlFrame,
    projection: Matrix4<f32>,
    view: Matrix4<f32>,
) {
    let mut model = Matrix4::<f32>::identity();
    let shape = {
        let x = paddle.x;
        let y = paddle.y;
        let w = paddle.width;
        let h = paddle.height;
        model.append_translation_mut(&Vector3::new(x, y, 0.0));
        vec![
            Vertex {
                position: [0.0, 0.0],
            },
            Vertex { position: [w, 0.0] },
            Vertex { position: [w, h] },
            Vertex { position: [0.0, h] },
        ]
    };
    let vertex_buffer = glium::VertexBuffer::new(gl_parts.display, &shape).unwrap();
    let indices = glium::index::IndexBuffer::new(
        gl_parts.display,
        glium::index::PrimitiveType::TrianglesList,
        &vec![0u16, 1, 2, 0, 2, 3],
    ).unwrap();
    let projection_slice: [[f32; 4]; 4] = projection.into();
    let view_slice: [[f32; 4]; 4] = view.into();
    let model_slice: [[f32; 4]; 4] = model.into();
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
            &gl_parts.program,
            &uniforms,
            &Default::default(),
        )
        .unwrap();
}
