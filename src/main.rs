use pixels::{Pixels, SurfaceTexture};
use transform2d::{Figure, Vertex, HEIGHT, WIDTH};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

fn main() {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = WindowBuilder::new()
        .with_title("2d transform")
        .with_inner_size(LogicalSize::new(WIDTH as u32, HEIGHT as u32))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();
    let surface_texture = SurfaceTexture::new(
        WIDTH.try_into().unwrap(),
        HEIGHT.try_into().unwrap(),
        &window,
    );
    let mut pixels = Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture).unwrap();

    let mut square = Figure::new(vec![
        Vertex::new(100., 100.),
        Vertex::new(100., 200.),
        Vertex::new(200., 200.),
        Vertex::new(200., 100.),
    ]);

    square.scale(2., 2.);
    square.rotate(45.);

    event_loop.run(move |event, _, control_flow| {
        if let Event::RedrawRequested(_) = event {
            println!("{square:#?}");
            square.draw(pixels.frame_mut());
            if let Err(_) = pixels.render() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        window.request_redraw();
    });
}
