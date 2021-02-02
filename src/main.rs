use image::ImageBuffer;

mod vec3;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() != 2 {
        eprintln!("Usage: <program> <output_file>");
        std::process::exit(1);
    }
    
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    let mut img = ImageBuffer::new(256, 256);

    for i in (0..IMAGE_WIDTH).rev() {
        eprintln!("Scanlines remaining: {}", i);
        for j in 0..IMAGE_HEIGHT {
            let b = (0.25 * 255.999) as i32;
            let r: i32 = (((j as f32) / (IMAGE_WIDTH - 1) as f32) * 255.999) as i32;
            let g: i32 = (((i as f32) / (IMAGE_HEIGHT - 1) as f32) * 255.999) as i32;

            let pixel = image::Rgb([r as u8, g as u8, b as u8]);
            img.put_pixel(i as u32, j as u32, pixel);
        }
    }

    img.save(&args[1]).unwrap();
}
