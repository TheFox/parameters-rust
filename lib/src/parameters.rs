
use std::env::vars;
use regex::Regex;
use crate::app::App;

pub struct Parameters {
    regexp: Regex,
    env_name: Option<String>,
    instance: Option<String>,
    search: String,
}

impl Parameters {
    pub fn new(regexp: String, env_name: Option<String>, instance: Option<String>, search: String) -> Self {
        println!("-> Parameters::new({})", regexp);
        let p = Self {
            regexp: Regex::new(&regexp).expect("Invalid regexp"),
            env_name: env_name,
            instance: instance,
            search: search,
        };
        println!("-> regexp: {:?}", p.regexp);
        p
    }

    pub fn process_input(&self, input: &String) -> String {
        println!("-> Parameters::process()");
        println!("-> input: '{}'", input);
        let mut output: String = String::from(input);

        for (key, value) in vars() {
            println!("-> var '{}': '{}'", key, value);

            if self.regexp.is_match(&key) {
                let base_var_name: String = self.regexp.replace(&key, "").into();

                // Environment
                // let len = 
                let mut env_search_var: String = String::from(&self.search);
                // basic_search_var.push_str(&key[]);

                // Basic
                let mut basic_search_var: String = String::from(&self.search);
                basic_search_var.push_str(&key);
                basic_search_var.push_str(&self.search);

                println!("-----------------------------");
                println!("-> FOUND var '{}': '{}' => '{}'", key, value, base_var_name);
                println!("-> basic '{}'", basic_search_var);
                println!("-----------------------------");
                
                output = output
                    .replace(&basic_search_var, &value);
                // let n = output.replace(&basic_search_var, &value);
                // output = n.to_string();
            }
        }

        println!("-> output: '{}'", output);
        output
    }
}
