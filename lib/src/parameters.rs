
use std::env::vars;
use regex::Regex;
use std::collections::BTreeMap;
use crate::app::App;

pub struct Parameters {
    // is_quiet: bool,
    // no_header: bool,
    regexp: Regex,
    search: String,
    env_name: Option<String>,
    instance: Option<String>,
}

impl Parameters {
    pub fn new(regexp: String, search: String, env_name: Option<String>, instance: Option<String>) -> Self {
        println!("-> Parameters::new({}, {}, e={:?}, i={:?})", regexp, search, env_name, instance);
        Self {
            regexp: Regex::new(&regexp).expect("Invalid regexp"),
            search: search,
            env_name: env_name,
            instance: instance,
        }
    }

    pub fn process(&self, input: &String) -> String {
        println!("-> Parameters::process()");
        println!("-> input: '{}'", input); println!();
        let mut output: String = String::from(input);

        let mut env_vars: BTreeMap<String, String> = BTreeMap::new();

        for (ename, evalue) in vars() {
            if self.regexp.is_match(&ename) {
                env_vars.insert(ename, evalue);
            }
        }

        for (ename, evalue) in env_vars.iter().rev() {
            println!("-> var '{}': '{}'", ename, evalue);

            // Copy for mut.
            let mut enamec = String::from(ename);

            // Instance
            if let Some(_instance) = &self.instance {
                // println!("  -> instance is some:  '{}'", _instance);

                let sub_instance = &enamec[(enamec.len() - _instance.len())..];
                // println!("  -> sub instance: '{}'", sub_instance);
                
                if _instance == sub_instance {
                    let _end = enamec.len() - _instance.len() - 1;
                    enamec = String::from(&enamec[0.._end]);
                    // println!("  -> new enamec: '{}'", enamec);
                }
            }

            // Environment
            if let Some(_env_name) = &self.env_name {
                // println!("  -> env is some: '{}'", _env_name);

                let sub_env = &enamec[(enamec.len() - _env_name.len())..];
                // println!("  -> sub env: '{}'", sub_env);

                if _env_name == sub_env {
                    let _end = enamec.len() - _env_name.len() - 1;
                    enamec = String::from(&enamec[0.._end]);
                    // println!("  -> new enamec: '{}'", enamec);
                }
            }

            if let Some(_value) = env_vars.get(&enamec) {
                // println!("  -> {} => {:?}", enamec, _value);
            } else {
                // TODO: what can we do here?
                panic!("variable '{}' not found in environment variables", enamec);
            }

            let mut tpl_var: String = String::from(&self.search);
            tpl_var.push_str(&enamec);
            tpl_var.push_str(&self.search);

            output = output.replace(&tpl_var, &evalue);

            // println!();
        }

        println!();
        println!("-> output: '{}'", output);
        output
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

    #[test]
    fn test_parameters0() {
        let regexp: String = "^SYMF_".into();
        let search: String = "@".into();
        let input: String = "DB_USER=_@SYMF_DB_USER@#".into();

        let p1 = Parameters::new(regexp, search, None, None);
        assert_eq!("DB_USER=_user1#", p1.process(&input));
    }

    type SS = &'static str;
    type BasicDto = (SS, SS, SS, SS);

    #[test]
    fn test_parameters_basic() {
        let _data: Vec<BasicDto> = vec![
            ("^SYMF_", "@", "DB_USER=@SYMF_DB_USER@", "DB_USER=user1"),
            ("^SYMF_", "@", "DB_USER=_@SYMF_DB_USER@#", "DB_USER=_user1#"),
            ("^SYMF_", "@", "DB_USER=/@SYMF_DB_USER@/", "DB_USER=/user1/"),

            ("^SYMF_", "@", "DB_PASS=@SYMF_DB_PASS@", "DB_PASS=password1"),
            ("^SYMF_", "@", "DB_PASS=_B_@SYMF_DB_PASS@_E_", "DB_PASS=_B_password1_E_"),
            ("^SYMF_", "@", "DB_PASS=/@SYMF_DB_PASS@/@SYMF_DB_PASS@/", "DB_PASS=/password1/password1/"),
        ];
        
        for _t in _data {
            let regexp: String = _t.0.into();
            let search: String = _t.1.into();
            let input: String = _t.2.into();

            let p1 = Parameters::new(regexp, search, None, None);
            assert_eq!(_t.3, p1.process(&input));
        }
    }

    type EnvDto = (SS, SS, SS, SS, SS);

    #[test]
    fn test_parameters_environment() {
        let _data: Vec<EnvDto> = vec![
            // ("^SYMF_", "@", "production", "DB_PASS=@SYMF_DB_PASS@", "DB_PASS=password1"),
            ("^SYMF_", "@", "_roduction", "DB_PASS=@SYMF_DB_PASS_PRODUCTION@", "DB_PASS=password2"),
        ];
        
        for _t in _data {
            let regexp: String = _t.0.into();
            let search: String = _t.1.into();
            let env_name: Option<String> = Some(_t.2.into());
            let input: String = _t.3.into();

            let p1 = Parameters::new(regexp, search, env_name, None);
            assert_eq!(_t.4, p1.process(&input));
        }
    }
}
