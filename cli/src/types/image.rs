use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ImageError {}

#[derive(Debug, Clone, Serialize)]
pub struct Image {
    image: String,
}

impl Image {
    pub fn from_str(image: &str) -> Result<Image, ImageError> {
        return Ok(Image {
            image: image.to_string(),
        });
    }
}
