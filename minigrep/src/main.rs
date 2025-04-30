use std::env::args;
use std::process;
use minigrep::Config;

fn main() {
    let args: Vec<String> = args().collect();

    let config   = Config::build(&args).unwrap_or_else(|_err| {
        eprintln!("Problem parsing arguments: {_err}");
        process::exit(1);
    });

    if let Err(_e) =  minigrep::run(config) {
        eprint!("Application error: {_e}");
        process::exit(1);
    }
}



