use image::{GrayImage, Luma};
use rayon::prelude::*;
use worldgen::generate_normalized_noise_map; // Import rayon for parallel processing

fn main() {
    // Define parameters
    let radius = 300;
    let chunk_size = 16;
    let num_chunks = 2 * radius + 1; // number of chunks per dimension (65 here)
    let img_width = num_chunks * chunk_size;
    let img_height = num_chunks * chunk_size;

    // Generate all chunk coordinates from -radius to +radius
    let chunk_coords: Vec<(i32, i32)> = (-radius..=radius)
        .flat_map(|cx| (-radius..=radius).map(move |cz| (cx, cz)))
        .collect();

    // Compute noise maps for each chunk in parallel using Rayon
    let chunk_results: Vec<(i32, i32, [[f64; 16]; 16])> = chunk_coords
        .into_par_iter()
        .map(|(cx, cz)| {
            let noise_map = generate_normalized_noise_map(42, cx, cz, 0.001);
            (cx, cz, noise_map)
        })
        .collect();

    // Create the big image with the appropriate dimensions
    let mut big_img: GrayImage = GrayImage::new(img_width as u32, img_height as u32);

    // Write each chunk's noise map into the global image at the corresponding position
    for (cx, cz, noise_map) in chunk_results {
        // Compute pixel offset: we shift de coordonnées de chunk pour obtenir des indices positifs
        let offset_x = ((cx + radius) * chunk_size) as u32;
        let offset_y = ((cz + radius) * chunk_size) as u32;
        write_chunk_to_image(&mut big_img, offset_x, offset_y, noise_map);
    }

    // Save the generated image to a file
    big_img
        .save("big_noise_map.png")
        .expect("Failed to save image");
    println!("Image saved as big_noise_map.png");
}

/// Writes a 16x16 chunk noise map into the provided image at the specified offset.
/// Noise values are normalized from the range [-64, 324] to [0, 255].
fn write_chunk_to_image(
    img: &mut GrayImage,
    offset_x: u32,
    offset_y: u32,
    noise_map: [[f64; 16]; 16],
) {
    let min_val = -64.0;
    let max_val = 324.0;
    let scale = 255.0 / (max_val - min_val);

    // Iterate over each pixel in the 16x16 noise map
    for (row_index, row) in noise_map.iter().enumerate() {
        for (col_index, &value) in row.iter().enumerate() {
            // Map noise value from [-64, 324] to [0, 255]
            let pixel_value = (((value - min_val) * scale).round() as u8).min(255);
            img.put_pixel(
                offset_x + col_index as u32,
                offset_y + row_index as u32,
                Luma([pixel_value]),
            );
        }
    }
}
