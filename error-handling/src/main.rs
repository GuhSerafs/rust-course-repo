use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn main() {
    /* ----------- Erros recuperáveis ----------------- */
    // match_error_cases();
    // handle_with_unwrap_else();

    /* --------- Atalhos p/ acessar os erros ---------- */
    // handle_with_simple_unwrap();
    // handle_with_expect();

    /* ----------- Propagação de erros ---------------- */
    
    {
        // Feito na raça, com match e tudo
        let s = propagation_raw();
        match s {
            Ok(s) => println!("Sucesso ao ler dados do arquivo! Usuário: {}", s), 
            Err(e) => println!("Fracasso ao ler dados do arquivo! {:?}", e)
        }
    }

    {
        // Usando o operador ?
        let s = propagation_simples();
        match s {
            Ok(s) => println!("Sucesso ao ler dados do arquivo! Usuário: {}", s), 
            Err(e) => println!("Fracasso ao ler dados do arquivo! {:?}", e)
        }
    }

    /* ----------- Erros irrecuperáveis --------------- */
}

fn match_error_cases() -> () {
    // Caso 1 -> Usando match p/ tratar diferentes erros
    let f = File::open("Hello.txt");

    let f = match f {
        Ok(file) => file, 
        Err(error) => panic!("Problem opening the file: {:?}", error)
    };
    // Desvantagens: muito verboso, nem sempre transmite intenção
}

fn handle_with_unwrap_else() -> () {
    // Caso 2 -> unwrap or else
    let f = File::open("Hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("Hello.txt").unwrap_or_else(|error| {
                panic!("Falha ao criar o arquivo: {:?}", error);
            })
        } else {
            panic!("Falha ao abrir o arquivo: {:?}", error);
        }
    });
    // Semelhante ao caso 1, mas escrito de uma forma mais Rustacea
}

fn handle_with_simple_unwrap() -> () {
    // Caso 1 - Usando o unwrap
    let f = File::open("hello.txt").unwrap();
}

fn handle_with_expect() -> () {
    // Caso 2 - Usando o expect
    let f = File::open("hello.txt").expect("Falha ao abrir o arquivo");

    // Vantagem do expect -> Mensagem de erro customizada
}

fn propagation_raw() -> Result<String, io::Error> {
    // Implementação da propagação feita na raça 
    let f = File::open("Hello.txt"); 

    let mut f = match f {
        Ok(file) => file, 
        Err(e) => return Err(e)
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => return Ok(s), 
        Err(e) => return Err(e)
    };
}

fn propagation_simples() -> Result<String, io::Error> {
    // Implementação da propagação usando operador ?
    // Obs: o operador ? aplicado a uma função que retorna um Result
    // sempre vai propagar o erro, caso este ocorra
    let mut f = File::open("Hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    return Ok(s);
}