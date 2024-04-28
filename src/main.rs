use anyhow::{anyhow, Context, Result};
use image::{imageops::FilterType, GenericImageView};
use std::{
    cmp::{max, min},
    env::args_os,
};

fn main() -> Result<()> {
    let mut args = args_os();

    let input_path = args
        .next()
        .ok_or_else(|| anyhow!("Input image path was not provided"))?;

    let output_path = args
        .next()
        .ok_or_else(|| anyhow!("Output thumbnail path was not provided"))?;

    let thumbnail_size: u32 = args
        .next()
        .ok_or_else(|| anyhow!("Thumbnail size was not provided"))?
        .to_str()
        .ok_or_else(|| anyhow!("Thumbnail size was not valid UTF-8"))?
        .parse()
        .context("Failed to parse the provided thumbnail size as a number")?;

    let input_image = image::open(input_path).context("Failed to open input image")?;

    // crop input image to be a square
    let (width, height) = input_image.dimensions();
    let size = min(width, height);
    let image = input_image.crop_imm(
        max(0, (width - height) / 2),
        max(0, (height - width) / 2),
        size,
        size,
    );

    let output_image = image.resize_exact(thumbnail_size, thumbnail_size, FilterType::Lanczos3);

    image::save_buffer(
        output_path,
        output_image.as_bytes(),
        thumbnail_size,
        thumbnail_size,
        output_image.color(),
    )
    .context("Failed to save thumbnail image")?;

    Ok(())
}
