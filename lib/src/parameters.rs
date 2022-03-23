
use std::env::vars;
use regex::Regex;
use std::collections::BTreeMap;
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

        let mut env_vars: BTreeMap<String, String> = BTreeMap::new();

        for (ename, evalue) in vars() {
            if self.regexp.is_match(&ename) {
                env_vars.insert(ename, evalue);
                // let base_var_name: String = self.regexp.replace(&ename, "").into();

                // // Environment
                // // let len = 
                // let mut env_search_var: String = String::from(&self.search);
                // // basic_search_var.push_str(&ename[]);

                // // Basic
                // let mut basic_search_var: String = String::from(&self.search);
                // basic_search_var.push_str(&ename);
                // basic_search_var.push_str(&self.search);

                // println!("-----------------------------");
                // println!("-> FOUND var '{}': '{}' => '{}'", ename, evalue, base_var_name);
                // println!("-> basic '{}'", basic_search_var);
                // println!("-----------------------------");
                
                // output = output
                //     .replace(&basic_search_var, &evalue);
                // let n = output.replace(&basic_search_var, &evalue);
                // output = n.to_string();
            }
        }

        for (ename, evalue) in env_vars.iter().rev() {
            println!("-> var '{}': '{}'", ename, evalue);

            // Copy for mut.
            let mut enamec = String::from(ename);

            if let Some(_instance) = &self.instance {
                println!("  -> instance is some: {}", _instance);

                let sub_name_instance = &enamec[_instance.len()..];
                println!("  -> sub_name_instance: '{}'", sub_name_instance);

                if _instance == sub_name_instance {
                    let _end = enamec.len() - _instance.len() - 1;
                    enamec = String::from(&enamec[0.._end]);
                }
            }
        }

        println!("-> output: '{}'", output);
        output
    }
}
