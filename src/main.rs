use anyhow::{anyhow, Context, Result};
use std::{env::args_os, path::PathBuf};

fn main() -> Result<()> {
    let mut args = args_os();

    let input_path: PathBuf = args
        .next()
        .ok_or_else(|| anyhow!("Input image path was not provided"))?
        .into();

    let output_path: PathBuf = args
        .next()
        .ok_or_else(|| anyhow!("Output thumbnail path was not provided"))?
        .into();

    let thumbnail_size: usize = args
        .next()
        .ok_or_else(|| anyhow!("Thumbnail size was not provided"))?
        .to_str()
        .ok_or_else(|| anyhow!("Thumbnail size was not valid UTF-8"))?
        .parse()
        .context("Failed to parse the provided thumbnail size as a number")?;

    Ok(())
}
