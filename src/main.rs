
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

    // Common Arguments
    let input_arg = Arg::with_name("input")
        .short("i")
        .long("input")
        .value_name("PATH")
        .help("Input path to template file. (required)")
        .takes_value(true);
    
    let output_arg = Arg::with_name("output")
        .short("o")
        .long("output")
        .value_name("PATH")
        .help("Path to output file. Defaut: STDOUT")
        .takes_value(true);
    
    let regexp_arg = Arg::with_name("regexp")
        .short("r")
        .long("regexp")
        .value_name("STRING")
        .help("Search regular expression for environment variable names. (required)")
        .takes_value(true);

    let env_arg = Arg::with_name("env")
        .short("e")
        .long("env")
        .value_name("STRING")
        .help("Name of the environment. For example: production")
        .takes_value(true);

    let instance_arg = Arg::with_name("instance")
        .short("n")
        .long("instance")
        .value_name("STRING")
        .help("Name of the Instance. For example: SHOPA, or SHOPB.")
        .takes_value(true);

    let search_arg = Arg::with_name("search")
        .short("s")
        .long("search")
        .value_name("CHAR")
        .help("Search char for template variables. Default: @")
        .takes_value(true);

    let quiet_arg = Arg::with_name("quiet")
        .short("q")
        .long("quiet")
        .value_name("CHAR")
        .help("Do not throw an error if there are variables missing being replaced.")
        .takes_value(false);

    // Main App
    let app = App::new(APP_NAME)
        .version(APP_VERSION)
        .author(APP_AUTHORS)
        .about(APP_HOMEPAGE)
        .usage("parameters [OPTIONS] [SUBCOMMAND]")
        .subcommand(vars_subcmd)
        .arg(input_arg)
        .arg(output_arg)
        .arg(regexp_arg)
        .arg(env_arg)
        .arg(instance_arg)
        .arg(search_arg)
        .arg(quiet_arg);
    
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
