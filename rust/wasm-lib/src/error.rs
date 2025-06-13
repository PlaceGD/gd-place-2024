use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum AppError {
    ConfigReadFailed(toml::de::Error),
    ConfigWriteFailed(std::io::Error),
    ConfigTaken,
    EventLoopError(winit::error::EventLoopError),
    ImageLoadError(image::ImageError),
    ImageReadError(std::io::Error),
    CreateSurfaceError(wgpu::CreateSurfaceError),
    SurfaceError(wgpu::SurfaceError),
    NoAdapter,
    RequestDeviceError(wgpu::RequestDeviceError),
    ConfigValidationError { reason: String },
}

impl Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ConfigReadFailed(e) => write!(f, "failed to read config.toml: {e}"),
            AppError::ConfigWriteFailed(e) => write!(f, "failed to write default config.toml: {e}"),
            AppError::ConfigTaken => write!(f, "config already taken in resumed()"),
            AppError::EventLoopError(e) => write!(f, "failed to start event loop: {e}"),
            AppError::ImageLoadError(e) => write!(f, "failed to load image: {e}"),
            AppError::ImageReadError(e) => write!(f, "failed to read image: {e}"),
            AppError::CreateSurfaceError(e) => write!(f, "failed to create surface: {e}"),
            AppError::SurfaceError(e) => write!(f, "surface error: {e}"),
            AppError::NoAdapter => write!(f, "failed to find compatible adapter"),
            AppError::RequestDeviceError(e) => write!(f, "failed to find request device: {e}"),
            AppError::ConfigValidationError { reason } => {
                write!(f, "failed to validate config: {reason}")
            }
        }
    }
}
