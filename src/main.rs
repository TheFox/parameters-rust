
extern crate clap;
use clap::{App, Arg, ArgMatches};
use std::env::args;

const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
// const APP_NAME: &'static str = "ParametersRust";
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const APP_HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");

fn main() {
    println!("-> start");

    let args: Vec<String> = args().collect();
    println!("-> args: '{:?}'", args);

    // Vars Sub Command
    let vars_subcmd = App::new("vars")
        .about("Print variables.");

    // Main App
    let app = App::new(APP_NAME)
        .version(APP_VERSION)
        .author(APP_AUTHORS)
        .about(APP_HOMEPAGE)
        .usage("parameters [OPTIONS] [SUBCOMMAND]")
        .subcommand(vars_subcmd);
    
    // Get Argument matches.
    let matches = app.get_matches();
    println!("-> matches '{:?}'", matches);

    match matches.subcommand() {
        ("vars", _) => {
            println!("-> cmd: vars");
            println!("APP_NAME '{}'", APP_NAME);
            println!("APP_VERSION '{}'", APP_VERSION);

            return;
        },
        _ => {
            println!("No command.");
        },
    }

    println!("-> end");
}

#[cfg(test)]
mod tests_base {
    #[test]
    fn test1() {
        assert!(true);
    }
}
