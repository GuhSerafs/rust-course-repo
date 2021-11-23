pub mod lib {
    use std::env;
    use regex::Regex;
    use text_colorizer::*;

    #[derive(Debug)]
    pub struct Arguments {
        pub target: String,
        pub replacement: String,
        pub filename: String,
        pub output: String
    }

    pub fn print_usage() {
        eprintln!("{} - substitui as ocorrências de uma string em outra", "quickreplace".green());
        eprintln!("Usage: quickreplace <target> <replacement> <INPUT> <OUTPUT>");
    }

    pub fn parse_args() -> Arguments {
        let args: Vec<String> = env::args().skip(1).collect(); 
        if args.len() != 4 {
            print_usage();
            eprintln!("{} número incorreto de argumentos: 4 argumentos esperados, recebidos {}", "Error:".red().bold(), args.len());
            std::process::exit(1);
        } else {
            Arguments {
                target: args[0].clone(),
                replacement: args[1].clone(),
                filename: args[2].clone(),
                output: args[3].clone()
            }
        }
    }

    pub fn replace(target: &str, replacement: &str, text: &str) -> Result<String, regex::Error> {
        let regex = Regex::new(target)?;
        Ok(regex.replace_all(text, replacement).to_string())
    }
}
