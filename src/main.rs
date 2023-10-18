use pixels::{Pixels, SurfaceTexture};
use transform2d::{clear, Figure, Vertex, HEIGHT, WIDTH};
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

const SPEED: f32 = 300.;

struct Control {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
    scale_up: bool,
    scale_down: bool,
    rotate_left: bool,
    rotate_right: bool,
}

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
    let mut control = Control {
        up: false,
        down: false,
        left: false,
        right: false,
        scale_up: false,
        scale_down: false,
        rotate_left: false,
        rotate_right: false,
    };

    let mut figure = Figure::new(vec![
        Vertex::new(100., 100.),
        Vertex::new(100., 200.),
        Vertex::new(200., 200.),
        Vertex::new(200., 100.),
    ]);

    let mut center = -1;

    let mut now = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        let new_now = std::time::Instant::now();
        let delta_time = new_now.duration_since(now).as_secs_f32();
        now = new_now;

        if let Event::RedrawRequested(_) = event {
            clear(pixels.frame_mut());
            figure.draw(pixels.frame_mut());
            if let Err(_) = pixels.render() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        let (mouse_cell, _) = input
            .mouse()
            .map(|(mx, my)| {
                let (dx, dy) = input.mouse_diff();
                let prev_x = mx - dx;
                let prev_y = my - dy;

                let (mx_i, my_i) = pixels
                    .window_pos_to_pixel((mx, my))
                    .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));

                let (px_i, py_i) = pixels
                    .window_pos_to_pixel((prev_x, prev_y))
                    .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));

                (
                    (mx_i as isize, my_i as isize),
                    (px_i as isize, py_i as isize),
                )
            })
            .unwrap_or_default();

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::W) {
                control.up = true;
            } else if input.key_released(VirtualKeyCode::W) {
                control.up = false;
            }

            if input.key_pressed(VirtualKeyCode::S) {
                control.down = true;
            } else if input.key_released(VirtualKeyCode::S) {
                control.down = false;
            }

            if input.key_pressed(VirtualKeyCode::A) {
                control.left = true;
            } else if input.key_released(VirtualKeyCode::A) {
                control.left = false;
            }

            if input.key_pressed(VirtualKeyCode::D) {
                control.right = true;
            } else if input.key_released(VirtualKeyCode::D) {
                control.right = false;
            }

            if input.key_pressed(VirtualKeyCode::H) {
                control.scale_up = true;
            } else if input.key_released(VirtualKeyCode::H) {
                control.scale_up = false;
            }

            if input.key_pressed(VirtualKeyCode::G) {
                control.scale_down = true;
            } else if input.key_released(VirtualKeyCode::G) {
                control.scale_down = false;
            }

            if input.key_pressed(VirtualKeyCode::E) {
                control.rotate_right = true;
            } else if input.key_released(VirtualKeyCode::E) {
                control.rotate_right = false;
            }

            if input.key_pressed(VirtualKeyCode::Q) {
                control.rotate_left = true;
            } else if input.key_released(VirtualKeyCode::Q) {
                control.rotate_left = false;
            }

            if input.key_pressed(VirtualKeyCode::Up) {
                figure.push(Vertex::new(0., 0.));
            } else if input.key_pressed(VirtualKeyCode::Down) {
                figure.pop();
            }

            if input.mouse_pressed(0) {
                center = figure.get_vertex(mouse_cell);
            }
        }

        if control.up {
            figure.translate(0., -SPEED * delta_time);
        } else if control.down {
            figure.translate(0., SPEED * delta_time);
        }

        if control.left {
            figure.translate(-SPEED * delta_time, 0.);
        } else if control.right {
            figure.translate(SPEED * delta_time, 0.);
        }

        if control.scale_up {
            figure.scale(1.001, 1.001);
        } else if control.scale_down {
            figure.scale(0.999, 0.999);
        }

        if control.rotate_right {
            figure.rotate(SPEED / 2. * delta_time, center);
        } else if control.rotate_left {
            figure.rotate(-SPEED / 2. * delta_time, center);
        }

        window.request_redraw();
    });
}
