mod app;
mod camera;
mod fake_texels;
mod game_state;
mod glsl_compiler;
mod heightmap;
mod heightmap_editor;
mod heightmap_gpu;
mod input_state;
mod model;
mod model_gpu;
mod phy_state;

extern crate nalgebra as na;
extern crate shaderc;

use winit::event::Event;
use winit::event_loop::{ControlFlow, EventLoop};

#[derive(Debug)]
pub enum AppMsg {
    EventMessage { event: Event<()> },
    MapReadAsyncMessage { vec: Vec<f32> },
    Render,
}

pub enum EventLoopMsg {
    Stop,
}

//mod test;
fn main() {
    //    test::main();

    env_logger::init();
    log::trace!("Starting actix system");

    let event_loop = winit::event_loop::EventLoop::new();

    let window = winit::window::Window::new(&event_loop).unwrap();

    let (tx_app, rx_app) = std::sync::mpsc::channel();
    let tx_app_for_event_loop = std::sync::mpsc::Sender::clone(&tx_app);

    let (tx_event_loop, rx_event_loop) = std::sync::mpsc::channel::<EventLoopMsg>();

    std::thread::spawn(move || {
        let _ = tx_app.send(AppMsg::Render);
        let mut app = app::App::new(window, tx_app, rx_app, tx_event_loop);
        loop {
            app.receive();
        }
    });

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { .. } => {
            tx_app_for_event_loop
                .send(AppMsg::EventMessage { event })
                .unwrap();
        }
        Event::EventsCleared => {
            match rx_event_loop.try_recv() {
                Ok(EventLoopMsg::Stop) => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            }
            std::thread::sleep(std::time::Duration::from_millis(4));
        }
        _ => {}
    });
}
