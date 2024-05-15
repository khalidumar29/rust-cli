use std::{ env, process };
use cli_app::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem with parsing arguments: {} ", err);
        process::exit(1)
    });

    if let Err(e) = cli_app::run(config) {
        println!("Application erro: {}", e);
        process::exit(1);
    }
}
