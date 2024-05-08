use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use serde::Deserialize;
use sqlx::mysql::MySqlConnectOptions;
use sqlx::{ConnectOptions, MySqlConnection};
use std::sync::Arc;

pub struct AppState {
    db: Arc<MySqlConnection>,
    // db: MySqlPool,
}

#[derive(Deserialize)]
struct GameInfos {
    playername: String,
    played_games: u32,
    wins: u32,
    loses: u32,
    chars: String,
}

#[get("/")]
async fn default() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/stats")]
async fn stats(json: web::Json<GameInfos>, data: web::Data<AppState>) -> impl Responder {
    // let query_result = sqlx::query(r#"INSERT INTO stats (player_name, games_played, wins, losses, character) VALUES (?, ?, ?, ?, ?)"#)
    //     .bind(json.playername)
    //     .bind(json.played_games)
    //     .bind(json.wins)
    //     .bind(json.loses)
    //     .bind(json.chars)
    //     .execute(&data.db)
    //     .await;

    HttpResponse::Ok().body(format!(
        "{}\n{}\n{}\n{}\n{}",
        json.playername, json.played_games, json.wins, json.loses, json.chars
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = match MySqlConnectOptions::new()
        .username("user1")
        .password("N32%5dQ4")
        .host("212.132.108.197")
        .database("ranking")
        .connect()
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };
    let pool_arc = Arc::new(pool);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: Arc::clone(&pool_arc), }))
            .service(default)
            .service(stats)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
