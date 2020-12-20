use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problema processando argumentos: {}", err);
        process::exit(1);
    });


    if let Err(e) = minigrep::run(config) {
        eprintln!("Erro na aplicação: {}", e);

        process::exit(1);
    }
}

