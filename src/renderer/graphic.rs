extern crate glium;
extern crate winit;

use glium::Surface;
use winit::event::{Event, WindowEvent};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 720;

pub fn build() {
    //infinite loop
    let event_loop: winit::event_loop::EventLoop<()> =
        winit::event_loop::EventLoopBuilder::new().build();

    //Helps build window
    let (_window, display) =
        glium::backend::glutin::SimpleWindowBuilder::new()
            .with_title("Glium_Window")
            .with_inner_size(WIDTH, HEIGHT)
            .build(&event_loop);

    let mut frame = display.draw();
    frame.clear_color(0.0, 0.0, 1.0, 1.0);
    frame.finish().unwrap();

    event_loop.run(move |event, _, controlflow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                controlflow.set_exit();
            },
            _ => ()
        }
    });
}

