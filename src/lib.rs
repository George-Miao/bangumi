#![doc = include_str!("../README.md")]

mod model;

pub mod endpoints;
pub use model::*;
pub use rustified::{self, Client as ReqwestClient, Endpoint};

pub const DEFAULT_DOMAIN: &str = "https://bangumi.moe";

use rustified::errors::ClientError as RustifyError;
use thiserror::Error;

pub fn client() -> ReqwestClient {
    ReqwestClient::default(DEFAULT_DOMAIN)
}

#[derive(Debug, Error)]
pub enum Error {
    /// Some endpoints return different data depend on their version
    #[error("Api version error: {0}")]
    Version(String),

    #[error("rustify error: {0}")]
    Rustify(#[from] RustifyError),

    #[error("Search found nothing")]
    NotFound,
}
