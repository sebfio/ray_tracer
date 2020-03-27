use super::scene::*;

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        assert!(scene.width > scene.height);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let sensor_x = ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let sensor_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;
        
        Ray {
            origin: Point{x:0.0,y:0.0,z:0.0},
            direction: Vector3 {
                    x: sensor_x,
                    y: sensor_y,
                    z: -1.0,
                }
                .normalize(),
        }
    }
}


pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<f64>;
    fn surface_normal(&self, p: &Point) -> Vector3;
}

impl <'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, object: &'b Element) -> Intersection<'b> {
        Intersection {
            distance: distance,
            element: object
        }
    }
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.elements
            .iter()
            .filter_map(|s| s.intersect(ray).map(|d| Intersection::new(d, s)))
            .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
    }
}

impl Element {
    pub fn color(&self) -> &Color {
        match *self {
            Element::Sphere(ref s) => &s.material.color,
            Element::Plane(ref p) => &p.material.color,
        }
    }
    pub fn material(&self) -> &Material {
        match *self {
            Element::Sphere(ref s) => &s.material,
            Element::Plane(ref p) => &p.material,
        }
    }
}
impl Intersectable for Element {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Element::Sphere(ref s) => s.intersect(ray),
            Element::Plane(ref p) => p.intersect(ray),
        }
    }

    fn surface_normal(&self, p: &Point) -> Vector3 {
        match *self {
            Element::Sphere(ref e) => e.surface_normal(p),
            Element::Plane(ref e) => e.surface_normal(p),
        }
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let v = self.p0 - ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }

    fn surface_normal(&self, _: &Point) -> Vector3{
        self.normal * -1.0
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<f64> {
       let l: Vector3 = self.center - ray.origin;
       let adj = l.dot(&ray.direction);
       let d2 = l.dot(&l) - (adj * adj);
       let radius2 = self.radius * self.radius;
       if d2 > radius2 {
           return None;
       }
       let thc = (radius2 - d2).sqrt();
       let t0 = adj - thc;
       let t1 = adj + thc;

       if t0 < 0.0 && t1 < 0.0 {
           return None;
       }

       let distance = if t0 < t1 { t0 } else { t1 };
       Some(distance)
    }

    fn surface_normal(&self, p: &Point) -> Vector3 {
        (*p - self.center).normalize()
    }
}