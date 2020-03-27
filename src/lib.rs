mod scene;
mod rendering;

use image::*;
use scene::*;
use rendering::*;

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let background = Rgba::from_channels(0, 0, 100, 0);
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
        let direction_to_light = light.direction * -1.0;

        let shadow_ray = Ray {
            origin: hit_point + (direction_to_light * scene.shadow_bias),
            direction: direction_to_light,
        };

        let in_light = scene.trace(&shadow_ray).is_none();

        let light_intensity = if in_light { light.intensity } else { 0.0 };
        let light_power = (surface_normal.dot(&direction_to_light) as f32).max(0.0) *
                        light_intensity;
        let light_reflected = intersection.element.material().albedo / std::f32::consts::PI;

        let light_color = light.color * light_power * light_reflected;
        color = color + (*intersection.element.color() * light_color);
    }
    color
}

#[test]
fn test_can_render_scene() {
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
                    color: Color {
                        red: 0.4,
                        green: 1.0,
                        blue: 0.4,
                    },
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
                    color: Color {
                        red: 1.0,
                        green: 1.0,
                        blue: 0.4,
                    },
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
                    color: Color {
                        red: 0.0,
                        green: 0.2,
                        blue: 1.0,
                    },
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
                    color: Color  {
                        red: 0.4,
                        green: 0.4,
                        blue: 0.1,
                    },
                    albedo: 2.0,
                }
            })
        ],
        lights: vec![
            DirectionalLight {
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
            },
            DirectionalLight {
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
            },
        ],
        shadow_bias: 1e-10
    };

    let img: DynamicImage = render(&scene);

    img.save("output.png").unwrap();
}