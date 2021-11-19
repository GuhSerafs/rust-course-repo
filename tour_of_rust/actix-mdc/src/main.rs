use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[derive(Deserialize)]
struct MdcParameters {
    n: u64, 
    m: u64,
}

fn main() {
    let server = HttpServer::new(|| {
            App::new()
                .route("/", web::get().to(get_index))
                .route("/mdc", web::post().to(post_mdc))
            });

    println!("Servidor no ar em http://localhost:3000 ...");

    server
        .bind("127.0.0.1:3000")
        .expect("Erro conectando o servidor ao endereço")
        .run()
        .expect("Erro ao inicializar o servidor.");
}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
                <title>Calculadora de MDC</title>
                <form action="/mdc" method="post">
                    <input type="text" name="n"/>
                    <input type="text" name="m"/>
                    <button type="submit">Calcular</button>
                </form>
            "#,
    )
}

fn post_mdc(form: web::Form<MdcParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computar o MDC com valores nulos eh um pouco impossivel\n");
    }

    let response = 
        format!("O maior divisor comum entre {} e {} eh <b>{}</b>\n", 
                form.n, form.m, mcd(form.n, form.m));
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn mcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}