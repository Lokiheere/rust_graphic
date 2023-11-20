/*
    I followed the tutorial at: https://glium.github.io/glium/book/tuto-07-shape.html
    to understand OpenGL in the Rust language.
 */

extern crate glium;
extern crate winit;

// Connect teapot to graphic2 for render
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
        Gouraud Shading Method:

        Gouraud shading is a lighting technique used in computer graphics. The basic idea is to determine the intensity of light at each vertex of a 3D model and then interpolate these values across the surface. This method creates a smooth shading effect by considering the direction of light.

        - If the direction of the light is perpendicular to an object's surface, the surface appears bright.
        - If the direction of the light is parallel to the surface, the surface appears dark.

        This shading method is effective in simulating realistic lighting conditions on 3D objects, providing a smooth transition of colors across the surface.

        Example Usage:
        Consider a 3D model with vertices and normals. Gouraud shading calculates the intensity of light at each vertex based on its normal vector. The intensity values are then interpolated between vertices, creating a smooth shading effect on the entire surface.
    */

    let vertex_shader_src = r#"
        #version 150

        in vec3 position;
        in vec3 normal;

        out vec3 v_normal;

        uniform mat4 matrix;

        void main() {
            v_normal = transpose(inverse(mat3(matrix))) * normal;
            gl_Position = matrix * vec4(position, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 150

        in vec3 v_normal;
        out vec4 color;
        uniform vec3 u_light;

        void main() {
            float brightness = dot(normalize(v_normal), normalize(u_light));
            vec3 dark_color = vec3(0.6, 0.0, 0.0);
            vec3 regular_color = vec3(1.0, 0.0, 0.0);
            color = vec4(mix(dark_color, regular_color, brightness), 1.0);
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
                target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

                let matrix = [
                    [0.01, 0.0, 0.0, 0.0],
                    [0.0, 0.01, 0.0, 0.0],
                    [0.0, 0.0, 0.01, 0.0],
                    [0.0, 0.0, 0.0, 1.0f32],
                ];

                // the direction of the light
                let light = [-1.0, 0.4, 0.9f32];

                let params = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        .. Default::default()
                    },
                    .. Default::default()
                };

                target.draw((&positions, &normals), &indices, &program,
                            &uniform! { matrix: matrix, u_light: light },
                            &params).unwrap();
                target.finish().unwrap();
            },
            _ => (),
        };
    });
}

