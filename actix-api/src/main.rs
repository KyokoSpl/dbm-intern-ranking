use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use mysql::{OptsBuilder, Pool};
use std::sync::Arc;

mod handlers;
mod models;
use handlers::{addplayer, base_stats, default, deletegame, deleteplayer, fighter, game};

fn get_env_var(name: &str) -> String {
    dotenv().ok();
    match std::env::var(name) {
        Ok(val) => val,
        Err(_) => panic!("{} environment variable is not set", name),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let username = get_env_var("MYSQL_USER");
    let password = get_env_var("MYSQL_PASSWORD");
    let host = get_env_var("MYSQL_HOST");
    let database = get_env_var("MYSQL_DATABASE");

    let opts = OptsBuilder::new()
        .ip_or_hostname(Some(host))
        .db_name(Some(database))
        .user(Some(username))
        .pass(Some(password));

    println!("Connecting to database...");
    let pool = Pool::new(opts).expect("failed to create mysql pool");
    println!("✅ Connection to the database is successful!");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(Arc::new(pool.clone())))
            .service(default)
            .service(
                web::scope("/ranking")
                    .service(game)
                    .service(addplayer)
                    .service(deleteplayer)
                    .service(deletegame)
                    .service(base_stats)
                    .service(fighter),
            )
    })
    .bind(("localhost", 8000))?
    .run()
    .await
}
