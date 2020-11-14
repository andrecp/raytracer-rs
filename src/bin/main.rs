use std::fs::File;
use std::io::prelude::*;

use raytracer_rs::generate_ppm_image;

fn main() -> std::io::Result<()> {
    let ppm_file_content = generate_ppm_image();
    let mut file =  File::create("image.ppm")?;
    file.write_all(ppm_file_content.as_bytes())?;
    Ok(())
}
