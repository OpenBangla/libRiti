#![allow(dead_code)]
#![allow(non_snake_case)]

#[macro_use]
extern crate lazy_static;

pub mod phonetic;
mod fixed;
pub mod context;
mod settings;
mod keycodes;
pub mod suggestion;
mod utility;

pub(crate) const ENV_LAYOUT: &str = "RITI_LAYOUT_FILE";
