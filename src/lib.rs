mod scene;
mod rendering;

use image::*;
use scene::*;
use rendering::*;

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let background = Rgba::from_channels(50, 100, 220, 0);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            match scene.trace(&ray) {
                Some(intersection) => {
                    let color: Color = get_color(scene, &ray, &intersection);
                    let r = (color.red * 255.0) as u8;
                    let b = (color.blue * 255.0) as u8;
                    let g = (color.green * 255.0) as u8;
                    image.put_pixel(x, y, Rgba::from_channels(r, g, b, 0));
                }
                None => image.put_pixel(x, y, background)
            };
        }
    }
    image
}

fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection) -> Color {
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.element.surface_normal(&hit_point);

    let mut color = Color {
        red: 0.0,
        blue: 0.0,
        green: 0.0,
    };

    for light in &scene.lights {
        let direction_to_light = match light {
            Light::Directional(d) => d.direction * -1.0,
            Light::Spherical(s) => (s.point - hit_point).normalize(),
        };

        let shadow_ray = Ray {
            origin: hit_point + (direction_to_light * scene.shadow_bias),
            direction: direction_to_light,
        };

        let shadow_intersection = scene.trace(&shadow_ray);
        let in_light = shadow_intersection.is_none() || shadow_intersection.unwrap().distance > light.distance(&hit_point).euclidean_distance();

        let light_intensity = match light {
            Light::Directional(light) => {
                if in_light { light.intensity } else { 0.0 }
            }
            Light::Spherical(s) => {
                let r2 = (s.point - hit_point).norm() as f32;
                s.intensity / (4.0 * ::std::f32::consts::PI * r2)
            }
        };
        let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) * light_intensity;
        let light_reflected = intersection.element.material().albedo / std::f32::consts::PI;

        let light_color = light.color().clone() * light_power * light_reflected;
        let tc: TextureCoordinates = intersection.element.texture_coordinates(&hit_point);
        color = color + (intersection.element.skin().color(&tc) * light_color);
    }
    color
}

#[test]
fn test_can_render_scene() {
    let checkers = image::open("checkers.png").unwrap();

    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        elements: vec! [
             Element::Sphere( Sphere {
                center: Point {
                    x: 0.0,
                    y: 0.5,
                    z: -2.0,
                },
                radius: 0.5,
                material: Material {
                    skin : Coloration::Color( Color {
                        red: 0.4,
                        green: 1.0,
                        blue: 0.4,
                    }),
                    albedo: 0.5,
                },
            }),
            Element::Sphere( Sphere {
                center: Point {
                    x: -3.0,
                    y: 0.5,
                    z: -5.0,
                },
                radius: 1.2,
                material: Material {
                    skin : Coloration::Color( Color {
                        red: 1.0,
                        green: 1.0,
                        blue: 0.4,
                    }),
                    albedo: 1.5,
                }
            }),
            Element::Sphere( Sphere {
                center: Point {
                    x: 3.0,
                    y: 1.0,
                    z: -5.0,
                },
                radius: 1.7,
                material: Material {
                    skin : Coloration::Texture(checkers.clone()),
                    albedo: 2.0,   
                }
            }),
            Element::Plane( Plane {
                p0: Point {
                    x: 0.0,
                    y: -1.0,
                    z: -3.0,
                },
                normal: Vector3 {
                    x: 0.0,
                    y: -1.0,
                    z: 0.0,
                },
                material: Material {
                    skin : Coloration::Texture(checkers.clone()),
                    albedo: 2.0,
                }
            })
        ],
        lights: vec![
            Light::Directional( DirectionalLight {
                direction: Vector3 {
                    x: -0.07,
                    y: -0.707,
                    z: -0.707,
                },
                color: Color  {
                    red: 1.0,
                    green: 1.0,
                    blue: 1.0,
                },
                intensity: 1.0
            }),
            Light::Directional( DirectionalLight {
                direction: Vector3 {
                    x: 0.207,
                    y: -0.707,
                    z: 0.05,
                },
                color: Color  {
                    red: 0.6,
                    green: 0.5,
                    blue: 0.1,
                },
                intensity: 1.0
            }),
            Light::Spherical( SphericalLight {
                point: Point {
                    x: 0.0,
                    y: 0.0,
                    z: -1.0,
                },
                color: Color {
                    red: 1.0,
                    green: 1.0,
                    blue: 0.0,
                },
                intensity: 3.0,
            })
        ],
        shadow_bias: 1e-10
    };

    let img: DynamicImage = render(&scene);

    img.save("output.png").unwrap();
}