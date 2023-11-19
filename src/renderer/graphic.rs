/*
    I followed the tutorial at: https://glium.github.io/glium/book/tuto-03-animated-triangle.html
    to understand OpenGL in the Rust language.
 */

extern crate glium;
extern crate winit;

use glium::{implement_vertex, Surface, uniform};
use winit::event::{Event, WindowEvent};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 720;

pub fn build() {
    //infinite loop
    let event_loop: winit::event_loop::EventLoop<()> =
        winit::event_loop::EventLoopBuilder::new().build();

    //Helps build window
    let (window, display) =
        glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title("Glium_Window")
            .with_inner_size(WIDTH, HEIGHT)
            .build(&event_loop);

    /*
        The shape is a collection of vertices
        So Struct named Vertex main purpose is to
        describe each individual vertex. The collection
        of vertices can be later represented a Vec<Vertex>
     */
    #[derive(Copy, Clone)]
    struct Vertex {
        // "position" represents the 2D coordinates of the vertex.
        position: [f32; 2],
        // "color" represents rgba of the fragment shader
        color: [f32; 3],
    }
    implement_vertex!(Vertex, position, color);

    let shape = vec![
        Vertex { position: [-0.5, -0.5], color: [1.0, 0.0, 0.0] },
        Vertex { position: [ 0.0,  0.5], color: [0.0, 1.0, 0.0] },
        Vertex { position: [ 0.5, -0.25], color: [0.0, 0.0, 1.0] }
    ];

    // Create a vertex buffer to store the shape's vertices.
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    /*
        Define a vertex shader and a fragment shader using GLSL (OpenGL Shading Language).
        The vertex shader calculates the screen coordinates of each vertex.
        The fragment shader determines the color of each pixel.
    */

    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec3 color;
        out vec3 vertex_color;

        uniform mat4 matrix;

        void main() {
            vertex_color = color;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        in vec3 vertex_color;
        out vec4 color;

        void main() {
            color = vec4(vertex_color, 1.0);
        }
    "#;

    // Create a program using the defined shaders.
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    // Draw the shape using the program and shaders.
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);
    target.draw(&vertex_buffer, &indices, &program, &glium::uniforms::EmptyUniforms,
                &Default::default()).unwrap();
    target.finish().unwrap();

    let mut t: f32 = 0.0;
    event_loop.run(move |event, _, controlflow| {
        match event {
            Event::WindowEvent {event, ..} => match event {
                WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                WindowEvent::CloseRequested => controlflow.set_exit(),
                _ => (),
            },
            Event::RedrawEventsCleared => {
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                // We update `t`
                t += 0.02;
                // We use the sine of t as an offset, this way we get a nice smooth animation
                let x_off = t.sin() * 0.5;

                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 1.0, 1.0);
                let uniforms = uniform! {
                    matrix: [
                        [1.0, 0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [x_off, 0.0, 0.0, 1.0f32],
                    ]
                };
                target.draw(&vertex_buffer, &indices, &program, &uniforms,
                            &Default::default()).unwrap();
                target.finish().unwrap();
            },
            _ => (),
        };
    });
}

