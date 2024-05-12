use actix_web::{middleware::Logger, web, App, HttpServer};
use mysql::{OptsBuilder, Pool};
use dotenv::dotenv;
use std::sync::Arc;

mod models;
mod handlers;
use handlers::{default, stats, player, fighter};

fn get_env_var(name: &str) -> String {
    dotenv().ok();
    match std::env::var(name) {
        Ok(val) => val,
        Err(_) => panic!("{} environment variable is not set", name),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
    let pool = Pool::new(opts).expect("failed to create mysql pool");
    println!("✅ Connection to the database is successful!");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(Arc::new(pool.clone())))
            .service(default)
            .service(stats)
            .service(player)
            .service(fighter)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
