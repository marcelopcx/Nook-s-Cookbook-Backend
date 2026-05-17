use actix_web::{web, App, HttpServer};
use backend::config::AppConfig;
use backend::{db, routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = AppConfig::from_env();
    let pool = db::create_pool(&config.database_url)
        .await
        .expect("No se pudo conectar a la base de datos");

    let host = config.host.clone();
    let port = config.port;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .configure(routes::configure)
    })
    .bind((host.as_str(), port))?
        .run()
        .await
}