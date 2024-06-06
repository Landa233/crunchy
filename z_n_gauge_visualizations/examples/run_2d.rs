#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use std::sync::mpsc::channel;

use crunchy::crarray::shape::Shape;
use iris::{setup, Camera, CameraBuilder, ScreenInfo, VecN};
use winit::keyboard::{Key, NamedKey};
use z_n_gauge::{experiment::experiment::generate_cosine, gauge_fields::lattice::ZNParameters};
use z_n_gauge_visualizations::{
    renderers::two_d::grid::{GridRenderer, GridSettings},
    simulation::simulation::{Package2D, Simulation2D},
};

#[derive(Debug)]
pub enum CustomEventEnum {
    Timer,
    SimUpload,
}

async fn run() {
    let (event_loop, screen) = setup::<CustomEventEnum>(ScreenInfo {
        width: 1800,
        height: 1600,
        depth_buffer: false,
    })
    .await;

    let mut camera = Camera::new(
        &screen,
        CameraBuilder::TwoD {
            position: VecN::new([0.0, 0.0]),
            camera_speed: 500.0,
            scale: 0.5,
        },
    );

    let event_loop_proxy = event_loop.create_proxy();
    event_loop_proxy.send_event(CustomEventEnum::Timer).ok();
    std::thread::spawn(move || {
        // Wake up the `event_loop` once every second and dispatch a custom event
        // from a different thread.
        loop {
            std::thread::sleep(std::time::Duration::from_secs_f32(0.016));
            event_loop_proxy.send_event(CustomEventEnum::Timer).ok();
        }
    });

    let mut last_render_time = instant::Instant::now();

    let shape = Shape::new([20, 20]);

    const ZORDER: usize = 3;

    let cosines = generate_cosine();

    let sim_parameters = ZNParameters::<ZORDER> {
        beta: 1.0,
        cosines,
        lambda: 0.0,
    };

    let mut sim = Simulation2D::new(shape, sim_parameters);

    for edge in sim.sim.edges.shape().iter() {
        println!("{:?}", edge);
    }

    let colors = vec![
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 1.0],
    ];
    let grid_settings = GridSettings {
        spacing: 10.0,
        colors,
    };

    let mut grid_renderer = GridRenderer::new(&screen, &camera, shape, grid_settings, &sim);

    grid_renderer
        .plaquette_renderer
        .update(&screen, [1, 0], [1.0, 0.0, 0.0, 1.0]);

    let (send, recv) = channel();
    let (send2, recv2) = channel();

    let event_loop_proxy = event_loop.create_proxy();
    event_loop_proxy.send_event(CustomEventEnum::Timer).ok();

    // move simulation to other thread
    std::thread::spawn(move || loop {
        std::thread::sleep(std::time::Duration::from_secs_f32(0.001));
        //recv3.recv().unwrap();

        let package = sim.edge_update();

        match package {
            Some(package) => {
                event_loop_proxy.send_event(CustomEventEnum::SimUpload).ok();
                send.send(package).unwrap();
                recv2.recv().unwrap();
            }
            None => (),
        }
    });

    event_loop
        .run(move |event, ewlt| match &event {
            winit::event::Event::UserEvent(my_event) => match my_event {
                CustomEventEnum::Timer => {
                    screen.window.request_redraw();
                }
                CustomEventEnum::SimUpload => {
                    let package = recv.recv().unwrap();

                    println!("{:?}", package);

                    let Package2D { edges, plaquettes } = package;

                    for (plaquette_id, holonomy) in plaquettes {
                        let color = grid_renderer.grid_settings.colors[holonomy];
                        let coord = [plaquette_id[1], plaquette_id[2]];
                        grid_renderer
                            .plaquette_renderer
                            .update(&screen, coord, color);
                    }

                    send2.send("Feedback").unwrap();
                }
            },
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::RedrawRequested => {
                    let mut encoder =
                        screen
                            .device
                            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: Some("Redraw"),
                            });

                    // Get the next frame
                    let frame = screen.get_frame();
                    let view = &frame
                        .texture
                        .create_view(&wgpu::TextureViewDescriptor::default());

                    let now = instant::Instant::now();
                    let dt = now - last_render_time;
                    last_render_time = now;

                    camera.update(&screen, dt.as_secs_f32());

                    // Clear frame
                    {
                        let mut render_pass = screen.produce_render_pass(&mut encoder, view, None);

                        grid_renderer.render(&mut render_pass);
                    }

                    screen.queue.submit(Some(encoder.finish()));
                    frame.present();
                }

                winit::event::WindowEvent::CloseRequested
                | winit::event::WindowEvent::KeyboardInput {
                    event:
                        winit::event::KeyEvent {
                            logical_key: Key::Named(NamedKey::Escape),
                            ..
                        },
                    ..
                } => ewlt.exit(),

                _ => camera.handle_window_event(event),
            },

            winit::event::Event::DeviceEvent { event, .. } => {
                camera.handle_device_event(event);
            }

            _ => {}
        })
        .unwrap();
}

fn main() {
    pollster::block_on(run());
}
