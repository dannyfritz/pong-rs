use Vertex;
use nalgebra::Matrix4;
use GlParts;
use glium::{self, Surface};
use pong_lib::game;

pub fn render_game(
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
            &gl_parts.program,
            &uniforms,
            &Default::default(),
        )
        .unwrap();
}
