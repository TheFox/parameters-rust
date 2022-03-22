
use std::env::args;
use std::fs::read_to_string;
use std::io::stdin;
use std::io::BufRead;
use std::io::Result;
use parameters_lib::app::App;
use parameters_lib::parameters::Parameters;

const APP_NAME: &'static str = env!("CARGO_PKG_NAME");
const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
const APP_AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const APP_HOMEPAGE: &'static str = env!("CARGO_PKG_HOMEPAGE");

#[cfg(debug_assertions)]
const APP_BUILD_AT: &'static str = "APP_BUILD_AT";
#[cfg(not(debug_assertions))]
const APP_BUILD_AT: &'static str = env!("APP_BUILD_AT");

fn print_app_info() {
    println!("{} {} ({})", APP_NAME, APP_VERSION, APP_BUILD_AT);
    println!("{}", APP_AUTHORS);
    println!("{}", APP_HOMEPAGE);
    println!("");
}

fn print_usage() {
    println!("Usage:\n  parameters [<OPTIONS...>]");
    println!("");
    println!("Options:");
    println!("  -h|--help                       Show help.");
    println!("  -i|--input <input_file>         Input path to template file or '-' for STDIN. (required)");
    println!("  -o|--output <output_file>       Output file path. Default: STDOUT");
    println!("  -r|--regexp <regexp>            Search regular expression for environment variable names. (required)");
    println!("  -e|--env|--environment <name>   Name of the Environment. For example: production");
    println!("  -n|--instance <name>            Name of the Instance. For example: instance1, or instance2.");
    println!("  -s|--search <string>            Search char for template variables. Default: @");
    // TODO: --quiet
    // println!("  -q|--quiet                      Do not throw an error if there are variables missing being replaced.");
    // TODO: --noheader
    // println!("  -H|--noheader                   Skip header.");
    println!("");
}

fn main() -> Result<()> {
    print_app_info();

    #[cfg(debug_assertions)]
    println!("-> start");

    let args: Vec<String> = args().collect();
    let argc = args.len();

    if cfg!(debug_assertions) {
        println!("-> args: {:?}", args);
        println!("-> argc: {:?}", argc);
    }

    if argc == 1 {
        print_usage();
        return Ok(());
    }

    let mut app = App::new();
    let mut skip_next = false;
    for index in 1..argc {
        if skip_next {
            skip_next = false;
            continue;
        }
        let arg = &args[index];
        let next = &args.get(index + 1);

        #[cfg(debug_assertions)]
        println!("-> arg: #{} {:?}", index, arg);

        match arg.as_str() {
            "-h" | "--help" => {
                print_usage();
                return Ok(());
            },
            "-V" | "--version" => {
                print_usage();
                return Ok(());
            },
            "-i" | "--input" => {
                if let Some(_next) = next {
                    app.input_file_path = Some(_next.to_string());
                    skip_next = true;
                }
            },
            "-o" | "--output" => {
                if let Some(_next) = next {
                    app.output_file_path = Some(_next.to_string());
                    skip_next = true;
                }
            },
            "-r" | "--regexp" => {
                if let Some(_next) = next {
                    app.regexp = Some(_next.to_string());
                    skip_next = true;
                }
            },
            "-e" | "--env" | "--environment" => {
                if let Some(_next) = next {
                    app.env_name = Some(_next
                        .to_string()
                        .to_uppercase());
                    skip_next = true;
                }
            },
            "-n" | "--instance" => {
                if let Some(_next) = next {
                    app.instance = Some(_next
                        .to_string()
                        .to_uppercase());
                    skip_next = true;
                }
            },
            "-s" | "--search" => {
                if let Some(_next) = next {
                    app.search = _next.to_string();
                    skip_next = true;
                }
            },
            "-q" | "--quiet" => {
                app.is_quiet = true;
            },
            "-H" | "--noheader" => {
                app.no_header = true;
            },
            _ => {
                panic!("Unrecognized argument: {}", arg);
            },
        }
    }

    if cfg!(debug_assertions) {
        println!("-> app.input_file_path: {:?}", app.input_file_path);
        println!("-> app.output_file_path: {:?}", app.output_file_path);
        println!("-> app.regexp: {:?}", app.regexp);
        println!("-> app.env_name: {:?}", app.env_name);
        println!("-> app.instance: {:?}", app.instance);
        println!("-> app.search: {:?}", app.search);
        println!("-> app.is_quiet: {:?}", app.is_quiet);
        println!("-> app.no_header: {:?}", app.no_header);
    }

    let input: String = {
        match app.input_file_path {
            Some(_input_file_path) => {
                // println!("-> _input_file_path: {}", _input_file_path);
                if _input_file_path == "-" {
                    println!("-> use stdin for input");
                    let mut buffer = String::new();
                    let stdin = stdin();
                    let mut handle = stdin.lock();
                    handle.read_line(&mut buffer)?;
                    buffer
                } else {
                    read_to_string(_input_file_path)
                        .expect("Cannot read file")
                }
            },
            None => panic!("--input argument is required."),
        }
    };

    if app.regexp.is_none() {
        panic!("--regexp argument is required.");
    }

    let parameters = Parameters::new(app.regexp.unwrap(), app.env_name, app.instance, app.search);
    parameters.process_input(&input);
    
    #[cfg(debug_assertions)]
    println!("-> end");

    Ok(())
}

#[cfg(test)]
mod tests_base {
    #[test]
    fn test1() {
        assert!(true);
    }
}
