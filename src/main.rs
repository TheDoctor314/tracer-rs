use image::ImageBuffer;

pub mod vec3; //fix later
pub mod ray;

use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        eprintln!("Usage: <program> <output_file>");
        std::process::exit(1);
    }
    
    const aspect_ratio: f32 = 16.0 / 9.0;
    let IMAGE_WIDTH = 400;
    let IMAGE_HEIGHT = ( (IMAGE_WIDTH as f32) / aspect_ratio ) as i32;

    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = Point3::from(0., 0., 0.);
    let horizontal = Vec3::from(viewport_width, 0., 0.);
    let vertical = Vec3::from(0., viewport_height, 0.);

    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::from(0., 0., focal_length);

    let mut img = ImageBuffer::new(IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32);

    for i in 0..IMAGE_WIDTH {
        for j in (0..IMAGE_HEIGHT).rev() {
            let u = (i as f32) / (IMAGE_WIDTH - 1) as f32;
            let v = (j as f32) / (IMAGE_HEIGHT -1) as f32;

            let r = Ray::new(&origin, &(lower_left_corner + horizontal * u + vertical * v - origin));
            let pixel = ray_color(&r);

            img.put_pixel(i as u32, j as u32, get_color(pixel));
        }
    }
    img.save(&args[1]).unwrap();
}

fn get_color(p: vec3::Color) -> image::Rgb<u8> {
    let r: u8 = ( 255.999 * p.x ) as u8;
    let g: u8 = ( 255.999 * p.y ) as u8;
    let b: u8 = ( 255.999 * p.z ) as u8;

    image::Rgb([r, g, b])
}

fn ray_color(r: &Ray) -> vec3::Color {
    let mut unit_dir = r.dir().normalize();
    let t = 0.5 * (*unit_dir.y() + 1.0);

    (1.0 - t) * vec3::Color::from(1.0, 1.0, 1.0) + t * vec3::Color::from(0.5, 0.7, 1.0)
}
