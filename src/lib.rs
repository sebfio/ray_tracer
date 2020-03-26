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
                    let color: Color = *intersection.object.color();
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
                    y: 0.0,
                    z: -5.0,
                },
                radius: 1.0,
                color: Color {
                    red: 0.4,
                    green: 1.0,
                    blue: 0.4,
                },
            }),
            Element::Sphere( Sphere {
                center: Point {
                    x: -3.0,
                    y: 0.0,
                    z: -5.0,
                },
                radius: 1.2,
                color: Color {
                    red: 1.0,
                    green: 1.0,
                    blue: 0.4,
                },
            }),
            Element::Sphere( Sphere {
                center: Point {
                    x: 3.0,
                    y: 0.0,
                    z: -5.0,
                },
                radius: 1.7,
                color: Color {
                    red: 0.0,
                    green: 0.2,
                    blue: 1.0,
                },
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
                color: Color  {
                    red: 0.4,
                    green: 0.4,
                    blue: 0.1,
                },
            })
        ],
    };

    let img: DynamicImage = render(&scene);

    img.save("output.png").unwrap();
}