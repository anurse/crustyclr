extern crate byteorder;

#[macro_use]
extern crate bitflags;

mod error;
mod utils;

/// Contains CLI metadata structures
// pub mod cli;

/// Contains PE structures
pub mod pe;

pub use error::Error;