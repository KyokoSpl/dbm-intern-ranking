use actix_web::{delete, get, post, web, HttpResponse, Responder};
use mysql::{prelude::Queryable, Pool};
use serde_json::json;
use std::sync::Arc;

use crate::models::{BaseStatsData, DelGame, FighterData, GameInfos, PlayerData, RemovePlayerData};

#[get("/")]
async fn default() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/game")]
async fn game(json: web::Json<GameInfos>, pool: web::Data<Arc<Pool>>) -> impl Responder {
    let mut conn = pool
        .get_conn()
        .expect("couldn't get mysql connection from pool");
    let query = r#"INSERT INTO game (player_id, fighter_id_1, wins, loses) VALUES (?, ?, ?, ?)"#;

    match conn.exec_drop(
        query,
        (json.player_id, json.fighter_id_1, json.wins, json.loses),
    ) {
        Ok(_) => {
            let status_text = format!(
                "✅ Data successfully written to the database!:\n{}\n{}\n{}\n{}\n",
                json.player_id, json.fighter_id_1, json.wins, json.loses
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
    let query2 = r#"DELETE FROM player WHERE id = ?"#;
    let query = r#"DELETE FROM game WHERE player_id = ? "#;

    match conn.exec_drop(query, (json.id,)) {
        Ok(_) => {
            let status_text = format!(
                "✅ Data successfully deleted from the game table!:\n{}\n",
                json.id
            );
            println!("{}", status_text);
            match conn.exec_drop(query2, (json.id,)) {
                Ok(_) => {
                    let status_text = format!(
                        "✅ Data successfully deleted from the player table:\n{}\n",
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

#[get("/player")]
async fn player(pool: web::Data<Arc<Pool>>) -> impl Responder {
    let mut conn = pool
        .get_conn()
        .expect("couldn't get mysql connection from pool");
    let query = r#"SELECT id, player_name FROM player"#;

    match conn.query_map(query, |(id, player_name): (u64, String)| PlayerData {
        id,
        player_name,
    }) {
        Ok(player_list) => {
            let player_list_json = json!(&player_list);
            println!("{}", player_list_json);
            HttpResponse::Ok().json(player_list_json)
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

#[get("/base_stats/{player_id}")]
async fn base_stats(path: web::Path<u64>, pool: web::Data<Arc<mysql::Pool>>) -> impl Responder {
    let player_id = path.into_inner();

    let mut conn = pool.get_conn().expect("Failed to get database connection");

    let query = r#"
        SELECT player_id, SUM(wins) AS wins, SUM(loses) AS loses 
        FROM game 
        WHERE player_id = ?
        GROUP BY player_id
    "#;

    // Execute query with parameter
    match conn.exec::<(u64, u32, u32), _, _>(query, (player_id,)) {
        Ok(results) => {
            let base_stats: Vec<BaseStatsData> = results
                .into_iter()
                .map(|(player_id, wins, loses)| BaseStatsData {
                    player_id,
                    wins,
                    loses,
                })
                .collect();

            HttpResponse::Ok().json(json!(&base_stats))
        }
        Err(err) => {
            eprintln!("Database error: {:?}", err);
            HttpResponse::InternalServerError().body("Error fetching stats")
        }
    }
}
