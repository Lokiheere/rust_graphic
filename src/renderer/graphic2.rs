/*
    I followed the tutorial at: https://glium.github.io/glium/book/tuto-03-animated-triangle.html
    to understand OpenGL in the Rust language.
 */

extern crate glium;
extern crate winit;

use crate::renderer::teapot;

use glium::{implement_vertex, Surface, uniform};
use winit::event::{Event, WindowEvent};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 720;

pub fn build_teapot() {
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

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normals = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                          &teapot::INDICES).unwrap();


    /*
        Define a vertex shader and a fragment shader using GLSL (OpenGL Shading Language).
        The vertex shader calculates the screen coordinates of each vertex.
        The fragment shader determines the color of each pixel.
    */

    let vertex_shader_src = r#"
        #version 140

        in vec3 position;
        in vec3 color;
        out vec3 vertex_color;

        uniform mat4 matrix;

        void main() {
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 1.0, 1.0);
        }
    "#;

    // Create a program using the defined shaders.
    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

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

                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 1.0, 1.0);

                let matrix = [
                    [0.01, 0.0, 0.0, 0.0],
                    [0.0, 0.01, 0.0, 0.0],
                    [0.0, 0.0, 0.01, 0.0],
                    [0.0, 0.0, 0.0, 1.0f32],
                ];

                target.draw((&positions, &normals), &indices, &program, &uniform! { matrix: matrix },
                            &Default::default()).unwrap();
                target.finish().unwrap();
            },
            _ => (),
        };
    });
}

