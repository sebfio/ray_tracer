use std::ops::{Add, Sub, Mul};

#[derive(Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Add for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, intensity: f32) -> Color {
        Color {
            red: self.red * intensity,
            green: self.green * intensity,
            blue: self.blue * intensity,
        }
    }
}

pub struct Material {
    pub color: Color,
    pub albedo: f32,
}

#[derive(Clone, Copy)]
pub struct Vector3 {
    pub x: f64, 
    pub y: f64, 
    pub z: f64,
}

pub type Point = Vector3;

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3 {x: self.x * other.x, y: self.y * other.y, z: self.z * other.z}
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 {
        Vector3 {x: self.x * other, y: self.y * other, z: self.z * other}
    }
}

impl Vector3 {
    pub fn normalize(&self) -> Vector3{
        let length: f64 = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        Vector3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn norm(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn dot(self, rhs: &Vector3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    pub fn euclidean_distance(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Material,
}

pub struct Plane {
    pub p0: Point,
    pub normal: Vector3,
    pub material: Material,
}

pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

#[derive(Clone, Copy)]
pub struct DirectionalLight {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

#[derive(Clone, Copy)]
pub struct SphericalLight {
    pub point: Point,
    pub color: Color,
    pub intensity: f32,
}

pub enum Light {
    Directional(DirectionalLight),
    Spherical(SphericalLight),
}

impl Light {
    pub fn color(&self) -> &Color {
        match *self {
            Light::Directional(ref d) => &d.color,
            Light::Spherical(ref s) => &s.color,
        }
    }
    pub fn distance(&self, hit_point: &Point) -> Vector3 {
        match *self {
            Light::Directional(_) => Vector3 {x: 1.0e10, y: 1.0e10, z: 1.0e10},   // large distance approx inf
            Light::Spherical(s) => s.point - *hit_point,
        }
    }
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>,
    pub lights: Vec<Light>,
    pub shadow_bias: f64,
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub element: &'a Element,
}