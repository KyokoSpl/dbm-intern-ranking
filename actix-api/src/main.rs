use actix_web::{middleware::Logger, web::{self, Data}, App, HttpServer};
use dotenv::dotenv;
use mysql::{OptsBuilder, Pool};
use std::sync::Arc;

mod handlers;
mod models;
use handlers::{default, fighter, player, stats};

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
        .user(Some(username))
        .pass(Some(password))
        .ip_or_hostname(Some(host))
        .db_name(Some(database));

    println!("Connecting to database...");
    let pool = match Pool::new(opts) {
        Ok(_) => {
            println!("✅ Connection to the database is successful!");
        }
        Err(err) => {
            println!("❌ Failed to create mysql pool: {}", err)
        }
    };

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::new(pool.clone()))
            .service(default)
            .service(player)
            .service(fighter)
            .service(stats)
            .service(
                web::scope("/ranking")
                    .service(stats)
                    .service(player)
                    .service(fighter),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
