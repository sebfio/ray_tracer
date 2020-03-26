use std::num::*;
use std::ops::{Sub};

#[derive(Clone, Copy)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
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

impl Vector3 {
    pub fn normalize(&mut self) -> Vector3{
        let length: f64 = (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt();
        Vector3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn dot(self, rhs: &Vector3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

pub struct Plane {
    pub p0: Point,
    pub normal: Vector3,
    pub color: Color,
}

pub enum Element {
    Sphere(Sphere),
    Plane(Plane),
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub elements: Vec<Element>,
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub object: &'a Element,
}