use std::{path::PathBuf, convert::TryFrom};

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub enum ImageFormat {
    RGB,
    SRGB,
    HDR16,
    HDR32,
}
impl Into<wgpu::TextureFormat> for ImageFormat {
    fn into(self) -> wgpu::TextureFormat {
        match self {
            ImageFormat::HDR16 => wgpu::TextureFormat::Rgba16Float,
            ImageFormat::HDR32 => wgpu::TextureFormat::Rgba32Float,
            ImageFormat::RGB => wgpu::TextureFormat::Rgba8Unorm,
            ImageFormat::SRGB => wgpu::TextureFormat::Rgba8UnormSrgb,
        }
    }
}

// Image represents data on the CPU.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Image {
    // Byte data representing the pixels of the image.
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub(crate) path: PathBuf,
}

impl TryFrom<(PathBuf, Vec<u8>)> for Image {
    type Error = std::io::Error;
    fn try_from((path, data): (PathBuf, Vec<u8>)) -> Result<Self, Self::Error> {
        let image = image::load_from_memory(&data).unwrap().to_rgba();
        let (width, height) = image.dimensions();
        Ok(Self {
            data: image.into_raw(),
            width,
            height,
            path,
        })
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, Eq, PartialEq)]
pub struct ImageRon {
    pub format: ImageFormat,
}

impl TryFrom<(PathBuf, Vec<u8>)> for ImageRon {
    type Error = ron::de::Error;
    fn try_from((_p, v): (PathBuf, Vec<u8>)) -> Result<Self, Self::Error> {
        ron::de::from_bytes(&v)
    }
}