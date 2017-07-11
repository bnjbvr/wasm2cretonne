extern crate wasmparser;
extern crate cton_frontend;
extern crate cretonne;

pub mod module_translator;

pub mod translation_utils;
mod code_translator;
mod sections_translator;
pub mod runtime;

/// Version number of the cretonne crate.
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
