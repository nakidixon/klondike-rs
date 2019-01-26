#![feature(const_fn)]

extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate log;

pub use crate::display::*;
pub use crate::game::*;

mod display;
mod game;
mod utils;
