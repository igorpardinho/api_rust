use actix_web::*;

mod routes;
use routes::info::*;
use routes::ping::*;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let api = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(ping))
            .route("/info", web::get().to(info))
    });

    let porta = 9091;
    let api = api
        .bind(format!("127.0.0.1:{}", porta))
        .expect("⚠️ Não conseguiu conectar...");

    println!("conectado com sucesso! \n ✅ http:localhost:{}", porta);

    api.run().await
}
