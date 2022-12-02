use std::cmp::Ordering;

use image::{DynamicImage, imageops::FilterType};
use anyhow::Result;

#[derive(clap::Subcommand, Debug)]
pub enum Action {
    Resize,
    Flip,
    Rotate {
        #[clap(default_value_t=90, short, long)]
        degree: u32,
    }
}

pub trait Transformer {
    /// # Errors
    /// Will return `Err` if open `src` file failed for invalid image file 
    fn transform(&self, src: &str, psize: u32) -> Result<DynamicImage>;
}

impl Transformer for Action {
    fn  transform(&self, src: &str, psize: u32) -> Result<DynamicImage> {
        let mut img = image::open(src)?;

        match img.height().cmp(&img.width()) {
            Ordering::Greater => img = img.crop(0,0, img.width(),img.width()),
            Ordering::Less => img = img.crop(0,0, img.height(), img.height()),
            Ordering::Equal => (),
        };

        img = img.resize(psize, psize, FilterType::Lanczos3);

        match self {
            Self::Resize => Ok(img),
            Self::Flip => Ok(img.fliph()),
            Self::Rotate { degree } => {
                match degree {
                    0..=90 => Ok(img.rotate90()),
                    91..=180 => Ok(img.rotate180()),
                    _ => Ok(img.rotate270()),
                }
            } 
        }

    }
}
