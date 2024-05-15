use actix_web::{get, post, web, HttpResponse, Responder};
use mysql::{prelude::Queryable, Pool};
use serde_json::json;
use std::sync::Arc;

use crate::models::{FighterData, GameInfos, PlayerData};

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
            &json.player_name,
            &json.chars,
            json.games_played,
            json.wins,
            json.loses,
        ),
    ) {
        Ok(_) => {
            let status_text = format!(
                "✅ Data successfully written to the database!:\n{}\n{}\n{}\n{}\n{}\n",
                json.player_name, json.chars, json.games_played, json.wins, json.loses
            );
            println!("{}", status_text);
            HttpResponse::Ok().body(status_text)
        }
        Err(err) => {
            eprintln!("❌ Failed to write game data to the database: {:?}", err);
            HttpResponse::InternalServerError().body("Failed to write game data to the database")
        }
    }
}

#[get("/player")]
async fn player(pool: web::Data<Arc<Pool>>) -> impl Responder {
    let mut conn = pool
        .get_conn()
        .expect("couldn't get mysql connection from pool");
    let query = r#"SELECT id, player_name FROM player"#;

    match conn.query_map(query, |(id, player_name): (u32, String)| PlayerData {
        id,
        player_name,
    }) {
        Ok(players) => {
            let players_json = json!(&players);
            println!("{}", players_json);
            HttpResponse::Ok().json(players_json)
        }
        Err(err) => {
            eprintln!(
                "❌ Failed to retrieve player data from the database: {:?}",
                err
            );
            HttpResponse::InternalServerError()
                .body("Failed to retrieve player data from the database")
        }
    }
}

#[get("/fighter")]
async fn fighter(pool: web::Data<Arc<Pool>>) -> impl Responder {
    let mut conn = pool
        .get_conn()
        .expect("couldn't get mysql connection from pool");
    let query = r#"SELECT id, fighter_name FROM fighter"#;

    match conn.query_map(query, |(id, fighter_name): (u32, String)| FighterData {
        id,
        fighter_name,
    }) {
        Ok(fighters) => {
            let fighters_json = json!(&fighters);
            println!("{}", fighters_json);
            HttpResponse::Ok().json(fighters_json)
        }
        Err(err) => {
            eprintln!(
                "❌ Failed to retrieve fighter data from the database: {:?}",
                err
            );
            HttpResponse::InternalServerError()
                .body("Failed to retrieve fighter data from the database")
        }
    }
}
