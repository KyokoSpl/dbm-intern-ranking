use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use mysql::{prelude::Queryable, OptsBuilder, Pool};
use serde::Deserialize;
use std::sync::Arc;
use dotenv::dotenv;

#[derive(Deserialize)]
struct GameInfos {
    playername: u32,
    chars: u32,
    games_played: u32,
    wins: u32,
    loses: u32,
}

#[get("/")]
async fn default() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/stats")]
async fn stats(json: web::Json<GameInfos>, pool: web::Data<Arc<Pool>>) -> impl Responder {
    let mut conn = pool
        .get_conn()
        .expect("couldn't get mysql connection from pool");
    let query = r#"INSERT INTO game (player_id, fighter_id, games_played, wins, loses) VALUES (?, ?, ?, ?, ?)"#;

    match conn.exec_drop(
        query,
        (
            &json.playername,
            &json.chars,
            json.games_played,
            json.wins,
            json.loses,
        ),
    ) {
        Ok(_) => {
            let status_text = format!(
                "✅ Data successfully written to the database!:\n{}\n{}\n{}\n{}\n{}\n",
                json.playername, json.chars, json.games_played, json.wins, json.loses
            );
            println!("{}", status_text);
            HttpResponse::Ok().body(status_text)
        }
        Err(err) => {
            eprintln!("❌ Failed to write data to the database: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to write data to the database")
        }
    }
}

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
            .service(stats)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
