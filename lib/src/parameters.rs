
use std::env::vars;
use crate::app::App;

pub struct Parameters {}

impl Parameters {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn process(app: &App) {
        println!("-> Parameters::process()");

        for (key, value) in vars() {
            println!("var {}: {}", key, value);
        }
    }
}
