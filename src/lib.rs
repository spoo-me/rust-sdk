//! Rust bindings for accessing the spoo.me API
//! 
//! This crate provides a client that can access all endpoints provided by spoo.me, with support for self hosted instances of it.
//!
//! Currently this library does not have a public documentation site, however running `cargo doc` will provide some information.
//! 
//! <br>
//! 
//! # Features
//! - `blocking`: Enables blocking methods for the client, allowing synchronous calls to the API.
//! - `custom_url`: Allows setting a custom base URL for the client, useful for self-hosted instances of spoo.me.

#![warn(missing_docs)]
#![warn(clippy::all)]

/// A client for the URL shortener API.
pub mod client;

/// Errors related to a request.
pub mod errors;

/// Requests and responses for the URL shortener API.
pub mod requests;

/// Tools for validating and formatting requests.
pub mod utils;