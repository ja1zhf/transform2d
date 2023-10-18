use std::ops::{Add, Mul, Sub};

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

#[derive(Clone, Copy)]
pub struct Vertex([f32; 3]);

impl Vertex {
    pub fn new(x: f32, y: f32) -> Self {
        Self([x, y, 1.])
    }

    fn round(self) -> Self {
        Self([self.0[0].round(), self.0[1].round(), self.0[2]])
    }
}

impl Add for Vertex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self([self.0[0] + rhs.0[0], self.0[1] + rhs.0[1], self.0[2]])
    }
}

impl Sub for Vertex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self([self.0[0] - rhs.0[0], self.0[1] - rhs.0[1], self.0[2]])
    }
}

impl Mul<Matrix> for Vertex {
    type Output = Self;

    fn mul(self, rhs: Matrix) -> Self {
        let mut result = Self([0., 0., 0.]);

        for i in 0..self.0.len() {
            for j in 0..rhs.0.len() {
                result.0[j] += self.0[i] * rhs.0[j][i]
            }
        }

        result
    }
}

#[derive(Clone)]
pub struct Matrix([[f32; 3]; 3]);

pub struct Figure(Vec<Vertex>);

impl Figure {
    pub fn new(vertices: Vec<Vertex>) -> Self {
        Figure(vertices)
    }

    pub fn push(&mut self, vertex: Vertex) {
        self.0.push(vertex);
        self.reposition_vertices();
    }

    pub fn pop(&mut self) {
        self.0.pop();
        self.reposition_vertices();
    }

    fn reposition_vertices(&mut self) {
        let n = self.0.len();
        let radius = 100.0;
        let center = self.get_center();

        for i in 0..n {
            let angle = i as f32 * 2.0 * std::f32::consts::PI / n as f32;
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            self.0[i].0[0] = x + center.0[0];
            self.0[i].0[1] = y + center.0[1];
        }
    }

    pub fn draw(&self, screen: &mut [u8]) {
        for i in 1..=self.0.len() {
            if i == self.0.len() {
                Self::line(&self.0[0].round(), &self.0[i - 1].round(), screen);
            } else {
                Self::line(&self.0[i - 1].round(), &self.0[i].round(), screen);
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

    fn get_center(&self) -> Vertex {
        let mut result = Vertex::new(0., 0.);

        for vertex in &self.0 {
            result.0[0] += vertex.0[0];
            result.0[1] += vertex.0[1];
        }

        result.0[0] /= self.0.len() as f32;
        result.0[1] /= self.0.len() as f32;

        result
    }

    pub fn get_vertex(&self, mouse: (isize, isize)) -> isize {
        let center = self.get_center();
        let mut distance = f32::sqrt(
            f32::powi(mouse.0 as f32 - center.0[0], 2) + f32::powi(mouse.1 as f32 - center.0[1], 2),
        );
        let mut result = -1;

        for (i, vertex) in self.0.iter().enumerate() {
            let dist = f32::sqrt(
                f32::powi(mouse.0 as f32 - vertex.0[0], 2)
                    + f32::powi(mouse.1 as f32 - vertex.0[1], 2),
            );

            if dist < distance {
                distance = dist;
                result = i as isize;
            }
        }

        result
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
        let scale = Matrix([[x, 0., 0.], [0., y, 0.], [0., 0., 1.]]);

        let center = self.get_center();

        for i in 0..self.0.len() {
            self.0[i] = (self.0[i] - center) * scale.clone() + center;
        }
    }

    pub fn rotate(&mut self, mut angle: f32, origin: isize) {
        let mut center = self.get_center();
        if origin >= 0 {
            center = self.0[origin as usize]
        }
        angle *= std::f32::consts::PI / 180.;
        let rotate = Matrix([
            [angle.cos(), -angle.sin(), 0.],
            [angle.sin(), angle.cos(), 0.],
            [0., 0., 1.],
        ]);

        for i in 0..self.0.len() {
            self.0[i] = (self.0[i] - center) * rotate.clone() + center;
        }
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        let position = Matrix([[1., 0., x], [0., 1., y], [0., 0., 1.]]);

        let center = self.get_center();

        for i in 0..self.0.len() {
            self.0[i] = (self.0[i] - center) * position.clone() + center;
        }
    }
}

pub fn clear(screen: &mut [u8]) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            Figure::set_pixel(
                &Vertex::new(x as f32, y as f32),
                Color::new(0, 0, 0, 255),
                screen,
            )
        }
    }
}
