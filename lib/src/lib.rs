
extern crate regex;

const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[cfg(debug_assertions)]
const APP_BUILD_AT: &'static str = "APP_BUILD_AT";
#[cfg(not(debug_assertions))]
const APP_BUILD_AT: &'static str = env!("APP_BUILD_AT");

pub mod types;
pub mod app;
pub mod parameters;
