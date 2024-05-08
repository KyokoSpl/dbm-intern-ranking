use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use mysql::{prelude::Queryable, OptsBuilder, Pool};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
struct GameInfos {
    playername: String,
    games_played: u32,
    wins: u32,
    loses: u32,
    chars: String,
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
    let query = r#"INSERT INTO ranking (player_name, games_played, wins, losses, chars) VALUES (?, ?, ?, ?, ?)"#;

    match conn.exec_drop(
        query,
        (
            &json.playername,
            json.games_played,
            json.wins,
            json.loses,
            &json.chars,
        ),
    ) {
        Ok(_) => {
            let status_text = format!(
                "✅ Data successfully written to the database!:\n{}\n{}\n{}\n{}\n{}\n",
                json.playername, json.games_played, json.wins, json.loses, json.chars
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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let username = "user1";
    let password = "N32%5dQ4";
    let host = "212.132.108.197";
    let database = "ranking";

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
