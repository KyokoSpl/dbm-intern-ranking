use actix_web::{delete, get, post, web, HttpResponse, Responder};
use mysql::{prelude::Queryable, Pool};
use serde_json::json;
use std::sync::Arc;

use crate::models::{DelGame, FighterData, GameInfos, PlayerData, RemovePlayerData};

#[get("/")]
async fn default() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/game")]
async fn game(json: web::Json<GameInfos>, pool: web::Data<Arc<Pool>>) -> impl Responder {
    let mut conn = pool
        .get_conn()
        .expect("couldn't get mysql connection from pool");
    let query = r#"INSERT INTO game (player_id, wins, loses) VALUES (?, ?, ?)"#;

    match conn.exec_drop(query, (json.player_id, json.wins, json.loses)) {
        Ok(_) => {
            let status_text = format!(
                "✅ Data successfully written to the database!:\n{}\n{}\n{}\n",
                json.player_id, json.wins, json.loses
            );
            println!("{}", status_text);
            HttpResponse::Ok().body(status_text)
        }
        Err(err) => {
            eprintln!("❌ Failed to write game data to the database:\n{:?}", err);
            HttpResponse::InternalServerError().body(format!(
                "Failed to write game data to the database:\n{:?}",
                err
            ))
        }
    }
}
#[delete("/delgame")]
async fn deletegame(json: web::Json<DelGame>, pool: web::Data<Arc<Pool>>) -> impl Responder {
    let mut conn = pool
        .get_conn()
        .expect("couldn't get mysql connection from pool");
    let query = r#"DELETE FROM game WHERE id = ?"#;

    match conn.exec_drop(query, (json.id,)) {
        Ok(_) => {
            let status_text = format!(
                "✅ Data successfully deleted from the database!:\n{}\n",
                json.id,
            );
            println!("{}", status_text);
            HttpResponse::Ok().body(status_text)
        }
        Err(err) => {
            eprintln!(
                "❌ Failed to delete player data from the database:\n{:?}",
                err
            );
            HttpResponse::InternalServerError().body(format!(
                "Failed to write game data to the database:\n{:?}",
                err
            ))
        }
    }
}

#[post("/addplayer")]
async fn addplayer(json: web::Json<PlayerData>, pool: web::Data<Arc<Pool>>) -> impl Responder {
    let mut conn = pool
        .get_conn()
        .expect("couldn't get mysql connection from pool");
    let query = r#"INSERT INTO player (id, player_name) VALUES (?, ?)"#;

    match conn.exec_drop(query, (json.id, &json.player_name)) {
        Ok(_) => {
            let status_text = format!(
                "✅ Data successfully written to the database!:\n{}\n{}\n",
                json.id, json.player_name
            );
            println!("{}", status_text);
            HttpResponse::Ok().body(status_text)
        }
        Err(err) => {
            eprintln!("❌ Failed to write game data to the database:\n{:?}", err);
            HttpResponse::InternalServerError().body(format!(
                "Failed to write game data to the database:\n{:?}",
                err
            ))
        }
    }
}

#[delete("/deleteplayer")]
async fn deleteplayer(
    json: web::Json<RemovePlayerData>,
    pool: web::Data<Arc<Pool>>,
) -> impl Responder {
    let mut conn = pool
        .get_conn()
        .expect("couldn't get mysql connection from pool");
    let query = r#"DELETE FROM player WHERE id = ?"#;

    match conn.exec_drop(query, (json.id,)) {
        Ok(_) => {
            let status_text = format!(
                "✅ Data successfully deleted from the database!:\n{}\n",
                json.id
            );
            println!("{}", status_text);
            HttpResponse::Ok().body(status_text)
        }
        Err(err) => {
            eprintln!(
                "❌ Failed to delete player data from the database:\n{:?}",
                err
            );
            HttpResponse::InternalServerError().body(format!(
                "Failed to write game data to the database:\n{:?}",
                err
            ))
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
