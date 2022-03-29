
include!(concat!(env!("OUT_DIR"), "/config.rs"));

use std::env::vars;
use regex::Regex;

use crate::types::Search;
use crate::types::Environment;
use crate::types::Instance;
use crate::types::EnvsMap;

pub struct Parameters {
    regexp: Regex,
    search: Search,
    env_name: Environment,
    instance: Instance,
    no_header: bool,
    // is_quiet: bool,
}

impl Parameters {
    pub fn new(regexp: String, search: String, env_name: Environment, instance: Instance, no_header: bool) -> Self {
        #[cfg(debug_assertions)]
        println!("-> Parameters::new({}, {}, e={:?}, i={:?}, nh={})", regexp, search, env_name, instance, no_header);

        Self {
            regexp: Regex::new(&regexp).expect("Invalid regexp"),
            search: search,
            env_name: env_name,
            instance: instance,
            no_header: no_header,
        }
    }

    pub fn process(&self, input: &String) -> String {
        if cfg!(debug_assertions) {
            println!("-> Parameters::process()");
            println!("-> input: '{}'", input); println!();
        }

        let mut output: String = String::from(input);

        // Collect relevant environment variables.
        let mut env_vars: EnvsMap = EnvsMap::new();
        for (ename, evalue) in vars() {
            if self.regexp.is_match(&ename) {
                env_vars.insert(ename, evalue);
            }
        }

        for (ename, evalue) in env_vars.iter().rev() {
            #[cfg(debug_assertions)]
            println!("-> var '{}': '{}'", ename, evalue);

            // Copy for mut.
            let mut enamec = String::from(ename);

            // Instance
            if let Some(_instance) = &self.instance {
                #[cfg(debug_assertions)]
                println!("  -> instance is some:  '{}'", _instance);

                let sub_instance = &enamec[(enamec.len() - _instance.len())..];

                #[cfg(debug_assertions)]
                println!("  -> sub instance: '{}'", sub_instance);

                if _instance == sub_instance {
                    let _end = enamec.len() - _instance.len() - 1;
                    enamec = String::from(&enamec[0.._end]);

                    #[cfg(debug_assertions)]
                    println!("  -> new enamec: '{}'", enamec);
                }
            }

            // Environment
            if let Some(_env_name) = &self.env_name {
                #[cfg(debug_assertions)]
                println!("  -> env is some: '{}'", _env_name);

                let sub_env = &enamec[(enamec.len() - _env_name.len())..];

                #[cfg(debug_assertions)]
                println!("  -> sub env: '{}'", sub_env);

                if _env_name == sub_env {
                    let _end = enamec.len() - _env_name.len() - 1;
                    enamec = String::from(&enamec[0.._end]);

                    #[cfg(debug_assertions)]
                    println!("  -> new enamec: '{}'", enamec);
                }
            }

            let mut tpl_var: String = String::from(&self.search);
            tpl_var.push_str(&enamec);
            tpl_var.push_str(&self.search);

            #[cfg(debug_assertions)]
            println!("  -> template variable: '{}'", tpl_var);

            output = output.replace(&tpl_var, &evalue);

            if cfg!(debug_assertions) {
                println!("  -> output: '{}'", output);
                println!();
            }
        }

        if cfg!(debug_assertions) {
            println!();
            println!("-> end output: '{}'", output);
        }

        if self.no_header {
            // Raw out
            output
        } else {
            // Add header
            format!("# Rendered by {} v{} @ {}\n\n{}", APP_NAME, APP_VERSION, chrono::Local::now().format("%Y-%m-%d %H:%M:%S %Z"), output)
        }
    }
}

#[cfg(test)]
mod tests_sub_string {
    #[test]
    fn basic_test() {
        assert!(true);
    }

    #[test]
    fn test_substring1() {
        let s1 = String::from("abcefg");
        assert_eq!("ab", &s1[0..2]);
    }

    #[test]
    fn test_substring2() {
        let s1 = String::from("abcefg");
        assert_eq!("ab", &s1[..2]);
    }

    #[test]
    fn test_substring3() {
        let s1 = String::from("abcefg");
        assert_eq!("cefg", &s1[2..]);
    }

    #[test]
    fn test_substring4() {
        let s2 = String::from("cefg");
        let s1 = String::from("abcefg");
        assert_eq!("cefg", &s1[(s1.len() - s2.len())..]);
    }
}

#[cfg(test)]
mod tests_parameters {
    use super::Parameters;
    use crate::types::Search;
    use crate::types::Environment;
    use crate::types::Instance;

    #[test]
    fn test_parameters_single() {
        let regexp: String = "^SYMF_".into();
        let search: String = "@".into();
        let input: String = "DB_USER=_@SYMF_DB_USER@#".into();

        let p1 = Parameters::new(regexp, search, None, None, true);
        assert_eq!("DB_USER=_user1#", p1.process(&input));
    }

    type SS = &'static str;
    type BasicDto = (SS, SS, SS, SS);

    #[test]
    fn test_parameters_basic() {
        let _data: Vec<BasicDto> = vec![
            // Variable
            ("^SYMF_", "@", "DB_USER=@SYMF_DB_USER@", "DB_USER=user1"),
            ("^SYMF_", "@", "DB_USER=_@SYMF_DB_USER@#", "DB_USER=_user1#"),
            ("^SYMF_", "@", "DB_USER=/@SYMF_DB_USER@/", "DB_USER=/user1/"),

            ("^SYMF_", "@", "DB_PASS=@SYMF_DB_PASS@", "DB_PASS=password1"),
            ("^SYMF_", "@", "DB_PASS=_B_@SYMF_DB_PASS@_E_", "DB_PASS=_B_password1_E_"),
            ("^SYMF_", "@", "DB_PASS=/@SYMF_DB_PASS@/@SYMF_DB_PASS@/", "DB_PASS=/password1/password1/"),

            // Exact Environment
            ("^SYMF_", "@", "DB_PASS1=@SYMF_DB_PASS@/DB_PASS2=@SYMF_DB_PASS_PRODUCTION@", "DB_PASS1=password1/DB_PASS2=password2"),

            // Exact Instance
            ("^SYMF_", "@", "DB_PASS1=@SYMF_DB_PASS@/DB_PASS2=@SYMF_DB_PASS_PRODUCTION@/DB_PASS3=@SYMF_DB_PASS_PRODUCTION_INSTANCE1@", "DB_PASS1=password1/DB_PASS2=password2/DB_PASS3=password3"),

            // Non-existing
            ("^SYMF_", "@", "DB_PASS=/@SYMF_XYZ@/", "DB_PASS=/@SYMF_XYZ@/"),
            ("^TEST_", "@", "DB_PASS=/@SYMF_DB_USER@/", "DB_PASS=/@SYMF_DB_USER@/"),
        ];

        for _t in _data {
            let regexp: String = _t.0.into();
            let search: Search = _t.1.into();
            let input: String = _t.2.into();

            let p1 = Parameters::new(regexp, search, None, None, true);
            assert_eq!(_t.3, p1.process(&input));
        }
    }

    type EnvDto = (SS, SS, SS, SS, SS);

    #[test]
    fn test_parameters_environment() {
        let _data: Vec<EnvDto> = vec![
            // Environment
            ("^SYMF_", "@", "PRODUCTION", "DB_PASS=@SYMF_DB_PASS@", "DB_PASS=password2"),

            // Fall-back to basic Variable
            ("^SYMF_", "@", "ABC", "DB_PASS=@SYMF_DB_PASS_PRODUCTION@", "DB_PASS=password2"),

            // Non-machting Environment
            ("^SYMF_", "@", "ABC", "DB_PASS=@SYMF_DB_PASS@", "DB_PASS=password1"),

            // Non-existing Environment
            ("^SYMF_", "@", "ABC", "DB_PASS=@SYMF_DB_PASS_XYZ@", "DB_PASS=@SYMF_DB_PASS_XYZ@"),

            // No fall-back
            ("^SYMF_", "@", "PRODUCTION", "DB_NAME=@SYMF_DB_NAME@", "DB_NAME=password5b"),
            ("^SYMF_", "@", "ABC", "DB_NAME=@SYMF_DB_NAME@", "DB_NAME=@SYMF_DB_NAME@"),
        ];

        for _t in _data {
            let regexp: String = _t.0.into();
            let search: Search = _t.1.into();
            let env_name: Environment = Some(_t.2.into());
            let input: String = _t.3.into();

            let p1 = Parameters::new(regexp, search, env_name, None, true);
            assert_eq!(_t.4, p1.process(&input));
        }
    }

    type InstanceDto = (SS, SS, SS, SS, SS, SS);

    #[test]
    fn test_parameters_instance() {
        let _data: Vec<InstanceDto> = vec![
            // Instance
            ("^SYMF_", "@", "PRODUCTION", "INSTANCE1", "DB_PASS=@SYMF_DB_PASS@", "DB_PASS=password3"),
            ("^SYMF_", "@", "PRODUCTION", "INSTANCE2", "DB_PASS=@SYMF_DB_PASS@", "DB_PASS=password4"),

            // Non-machting Instance
            ("^SYMF_", "@", "PRODUCTION", "ABC", "DB_PASS=@SYMF_DB_PASS_PRODUCTION_INSTANCE1@", "DB_PASS=password3"),

            // Fall-back to Environment
            ("^SYMF_", "@", "PRODUCTION", "ABC", "DB_PASS=@SYMF_DB_PASS@", "DB_PASS=password2"),

            // Non-existing Instance
            ("^SYMF_", "@", "PRODUCTION", "ABC", "DB_PASS=@SYMF_DB_PASS_PRODUCTION@", "DB_PASS=@SYMF_DB_PASS_PRODUCTION@"),
        ];

        for _t in _data {
            let regexp: String = _t.0.into();
            let search: Search = _t.1.into();
            let env_name: Environment = Some(_t.2.into());
            let instance: Instance = Some(_t.3.into());
            let input: String = _t.4.into();

            let p1 = Parameters::new(regexp, search, env_name, instance, true);
            assert_eq!(_t.5, p1.process(&input));
        }
    }
}
