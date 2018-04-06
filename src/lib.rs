// Dependencies
extern crate piston_window;
extern crate env_logger;
#[macro_use]
extern crate log;


// Submodules
pub mod demo;

mod input_state;


// Version
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
