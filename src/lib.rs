#![warn(rustdoc::missing_crate_level_docs)]
#![deny(rustdoc::invalid_codeblock_attributes)]
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
#![warn(rustdoc::private_intra_doc_links)]
#![doc(html_root_url = "https://docs.rs/tanic/0.0.1")]
#![doc(issue_tracker_base_url = "https://github.com/sdd/tanic/issues/")]

//! # Tanic
//!
//! An Iceberg Swiss Army Knife?
pub mod config;
pub mod errors;
pub mod iceberg_context;

pub mod app_message;

pub use errors::Result;
pub use errors::TanicError;
