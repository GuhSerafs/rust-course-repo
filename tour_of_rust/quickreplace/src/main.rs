use quickreplace::lib::*;
use std::fs;
use text_colorizer::*;

fn main() {
    let args = parse_args();
    
    let data = match fs::read_to_string(&args.filename) {
        Ok(v) => v, 
        Err(e) => {
            eprintln!("{} falha na leitura do arquivo '{}': {:?}", 
                "Erro: ".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    };

    let replaced_data = match replace(&args.target, &args.replacement, &data) {
        Ok(v) => v, 
        Err(e) => {
            eprintln!("{} falha ao substituir o texto: {:?}", 
            "Erro: ".red().bold(), e);
        std::process::exit(1);
        }
    };
    
    match fs::write(&args.output, &replaced_data) {
        Ok(_) => {}, 
        Err(e) => {
            eprintln!("{} falha ao escrever no arquivo '{}': {:?}", 
                "Erro: ".red().bold(), args.filename, e);
            std::process::exit(1);
        }
    }
}