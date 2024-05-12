use actix_web::{get, post, web, HttpResponse, Responder};
use mysql::{prelude::Queryable, Pool};
use serde_json::json;
use std::sync::Arc;

use crate::models::{GameInfos, PlayerData, FighterData};

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

#[get("/player")]
async fn player(pool: web::Data<Arc<Pool>>) -> impl Responder {
    let mut conn = pool
        .get_conn()
        .expect("couldn't get mysql connection from pool");
    let query = r#"SELECT id, playername FROM player"#;

    match conn.query_map(query, |(id, playername): (u32, String)| PlayerData { id, playername }) {
        Ok(players) => {
            let players_json = json!(&players);
            println!("{}", players_json);
            HttpResponse::Ok().json(players_json)
        }
        Err(err) => {
            eprintln!("❌ Failed to retrieve data from the database: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to retrieve data from the database")
        }
    }
}

#[get("/fighter")]
async fn fighter(pool: web::Data<Arc<Pool>>) -> impl Responder {
    let mut conn = pool
        .get_conn()
        .expect("couldn't get mysql connection from pool");
    let query = r#"SELECT id, fighter_name FROM fighter"#;

    match conn.query_map(query, |(id, fighter_name): (u32, String)| FighterData { id, fighter_name }) {
        Ok(fighters) => {
            let fighters_json = json!(&fighters);
            println!("{}", fighters_json);
            HttpResponse::Ok().json(fighters_json)
        }
        Err(err) => {
            eprintln!("❌ Failed to retrieve data from the database: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to retrieve data from the database")
        }
    }
}