mod client;
pub use client::{HTTPNodeClient};

use thiserror::Error;

/// Error type wrapping underlying module errors.
#[derive(Error, Debug)]
pub enum HttpClientError {
	/// RPC Error
    #[error("{0}")]
	RPCError(String),
}

