
use std::env::vars;
use crate::app::App;

pub struct Parameters {
    regexp: Option<String>,
    env_name: Option<String>,
    instance: Option<String>,
    search: Option<String>,
}

impl Parameters {
    pub fn new(regexp: Option<String>,
        env_name: Option<String>,
        instance: Option<String>,
        search: Option<String>) -> Self {
        Self {
            regexp: regexp,
            env_name: env_name,
            instance: instance,
            search: search,
        }
    }

    pub fn process_input(&self, input: &String) -> String {
        println!("-> Parameters::process()");
        println!("-> input: '{}'", input);

        for (key, value) in vars() {
            println!("-> var {}: {}", key, value);
        }

        String::new()
    }
}
