// Dependencies
extern crate env_logger;
#[macro_use] extern crate log;
#[macro_use] extern crate ndarray;
extern crate piston_window;


// Submodules
pub mod demo;

mod input_state;
mod polytopes;


// Version
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
