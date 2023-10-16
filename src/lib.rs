use std::ops::Mul;

pub const WIDTH: usize = 800;
pub const HEIGHT: usize = 600;

struct Color {
    red: u8,
    green: u8,
    blue: u8,
    alpha: u8,
}

impl Color {
    fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }
}

#[derive(Clone)]
pub struct Vertex([f32; 2]);

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self([x, y])
    }
}

impl Mul<Matrix> for Vertex {
    type Output = Self;

    fn mul(self, rhs: Matrix) -> Self {
        let mut result = Self([0., 0.]);

        for i in 0..self.0.len() {
            for j in 0..rhs.0.len() {
                result.0[j] += self.0[i] * rhs.0[j][i]
            }
        }

        result
    }
}

#[derive(Clone)]
pub struct Matrix([[f32; 2]; 2]);

pub struct Figure(Vec<Vertex>);

impl Figure {
    pub fn new(vertices: Vec<Vertex>) -> Self {
        Figure(vertices)
    }

    pub fn draw(&self, screen: &mut [u8]) {
        for i in 1..=self.0.len() {
            if i == self.0.len() {
                Self::line(&self.0[0], &self.0[i - 1], screen);
            } else {
                Self::line(&self.0[i - 1], &self.0[i], screen);
            }
        }
    }

    fn line(start: &Vertex, end: &Vertex, screen: &mut [u8]) {
        let mut current = start.clone();
        let delta = Vertex::new((end.0[0] - start.0[0]).abs(), (end.0[1] - start.0[1]).abs());
        let step = Vertex::new(
            if start.0[0] < end.0[0] { 1. } else { -1. },
            if start.0[1] < end.0[1] { 1. } else { -1. },
        );
        let mut err = delta.0[0] - delta.0[1];

        loop {
            Self::set_pixel(&current, Color::new(255, 0, 0, 255), screen);

            if current.0[0] == end.0[0] && current.0[1] == end.0[1] {
                break;
            }

            let e2 = 2. * err;

            if e2 > -delta.0[1] {
                err -= delta.0[1];
                current.0[0] += step.0[0];
            }

            if e2 < delta.0[0] {
                err += delta.0[0];
                current.0[1] += step.0[1];
            }
        }
    }

    fn set_pixel(position: &Vertex, color: Color, screen: &mut [u8]) {
        if position.0[0] < WIDTH as f32
            && position.0[1] < HEIGHT as f32
            && position.0[0] >= 0.
            && position.0[1] >= 0.
        {
            let index = 4 * (position.0[1] * WIDTH as f32 + position.0[0]) as usize;
            screen[index] = color.red;
            screen[index + 1] = color.green;
            screen[index + 2] = color.blue;
            screen[index + 3] = color.alpha;
        }
    }

    pub fn scale(&mut self, x: f32, y: f32) {
        let scale = Matrix([[x, 0.], [0., y]]);

        for i in 0..self.0.len() {
            self.0[i] = self.0[i].clone() * scale.clone();
        }
    }

    pub fn rotate(&mut self, angel: f32) {
        let scale = Matrix([[angel.cos(), -angel.sin()], [angel.sin(), angel.cos()]]);

        for i in 0..self.0.len() {
            self.0[i] = self.0[i].clone() * scale.clone();
        }
    }
}