mod db;

use mysql::{Error, Pool, PooledConn};
use rocket::http::uri::Origin;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::{json, Value};
use rocket::State;

#[macro_use]
extern crate rocket;

const RANKING: Origin<'static> = uri!("/ranking");

#[post("/playername?<msg>")]
fn playername(msg: Option<&str>, pool: &State<Pool>) -> Result<Value, Value> {
    if let Some(msg) = msg {
        if msg.len() < 1 {
            // Print an error if the length of msg is below 1
            println!("Error: Message length is below 1");
            return Err(json!({
                "error": "Message length is below 1"
            }));
        }
        println!("Playername {}", msg);
        // Establish a connection to MySQL
        let mut conn = match pool.get_conn() {
            Ok(pool) => pool,
            Err(e) => {
                println!("Error connecting to MySQL: {:?}", e);
                return Err(json!({
                    "error": "Failed to connect to MySQL"
                }));
            }
        };

        // Insert data into the database
        match db::insert_player_name(&mut conn, msg) {
            Ok(_) => {
                return Ok(json!({
                    "status": "Successfully inserted into database"
                }));
            }
            Err(e) => {
                println!("Error inserting into database: {:?}", e);
                return Err(json!({
                    "error": "Failed to insert into database"
                }));
            }
        }
    }

    Err(json!({
        "notes": "It doesn't work"
    }))
}

#[post("/games-played?<msg>")]
fn gamesplayed(msg: Option<&str>, pool: &State<Pool>) -> Result<Value, Value> {
    if let Some(msg) = msg {
        if msg.len() < 1 {
            // Print an error if the length of msg is below 1
            println!("Error: Message length is below 1");
            return Err(json!({
                "error": "Message length is below 1"
            }));
        }
        println!("{msg}");
        // Establish a connection to MySQL
        let mut conn = match pool.get_conn() {
            Ok(pool) => pool,
            Err(e) => {
                println!("Error connecting to MySQL: {:?}", e);
                return Err(json!({
                    "error": "Failed to connect to MySQL"
                }));
            }
        };

        // Insert data into the database
        match db::insert_games_played(&mut conn, msg) {
            Ok(_) => {
                return Ok(json!({
                    "status": "Successfully inserted into database"
                }));
            }
            Err(e) => {
                println!("Error inserting into database: {:?}", e);
                return Err(json!({
                    "error": "Failed to insert into database"
                }));
            }
        }
    }

    Err(json!({
        "notes": "It doesn't work"
    }))
}

#[post("/wins?<msg>")]
fn wins(msg: Option<&str>, pool: &State<Pool>) -> Result<Value, Value> {
    if let Some(msg) = msg {
        if msg.len() < 1 {
            // Print an error if the length of msg is below 1
            println!("Error: Message length is below 1");
            return Err(json!({
                "error": "Message length is below 1"
            }));
        }
        println!("{msg}");

        // Establish a connection to MySQL
        let mut conn = match pool.get_conn() {
            Ok(pool) => pool,
            Err(e) => {
                println!("Error connecting to MySQL: {:?}", e);
                return Err(json!({
                    "error": "Failed to connect to MySQL"
                }));
            }
        };

        // Insert data into the db
        match db::insert_wins(&mut conn, msg) {
            Ok(_) => {
                return Ok(json!({
                    "status": "Successfully inserted into database"
                }));
            }
            Err(e) => {
                println!("Error inserting into database: {:?}", e);
                return Err(json!({
                    "error": "Failed to insert into database"
                }));
            }
        }
    }

    Err(json!({
        "notes": "It doesn't work"
    }))
}

#[post("/loses?<msg>")]
fn loses(msg: Option<&str>, pool: &State<Pool>) -> Result<Value, Value> {
    if let Some(msg) = msg {
        if msg.len() < 1 {
            // Print an error if the length of msg is below 1
            println!("Error: Message length is below 1");
            return Err(json!({
                "error": "Message length is below 1"
            }));
        }
        println!("{msg}");

        // Establish a connection to MySQL
        let mut conn = match pool.get_conn() {
            Ok(pool) => pool,
            Err(e) => {
                println!("Error connecting to MySQL: {:?}", e);
                return Err(json!({
                    "error": "Failed to connect to MySQL"
                }));
            }
        };

        // Insert data into the database
        match db::insert_losses(&mut conn, msg) {
            Ok(_) => {
                return Ok(json!({
                    "status": "Successfully inserted into database"
                }));
            }
            Err(e) => {
                println!("Error inserting into database: {:?}", e);
                return Err(json!({
                    "error": "Failed to insert into database"
                }));
            }
        }
    }

    Err(json!({
        "notes": "It doesn't work"
    }))
}

#[post("/chars?<msg>")]
fn chars(msg: Option<&str>, pool: &State<Pool>) -> Result<Value, Value> {
    if let Some(msg) = msg {
        if msg.len() < 1 {
            // Print an error if the length of msg is below 1
            println!("Error: Message length is below 1");
            return Err(json!({
                "error": "Message length is below 1"
            }));
        }
        println!("{msg}");

        // Establish a connection to MySQL
        let mut conn = match pool.get_conn() {
            Ok(pool) => pool,
            Err(e) => {
                println!("Error connecting to MySQL: {:?}", e);
                return Err(json!({
                    "error": "Failed to connect to MySQL"
                }));
            }
        };

        // Insert data into the database
        match db::insert_character(&mut conn, msg) {
            Ok(_) => {
                return Ok(json!({
                    "status": "Successfully inserted into database"
                }));
            }
            Err(e) => {
                println!("Error inserting into database: {:?}", e);
                return Err(json!({
                    "error": "Failed to insert into database"
                }));
            }
        }
    }

    Err(json!({
        "notes": "It doesn't work"
    }))
}

#[launch]
fn rocket() -> _ {
    let pool = match db::establish_connection() {
        Ok(pool) => pool,
        Err(e) => {
            println!("Error connecting to MySQL: {:?}", e);
            std::process::exit(1);
        }
    };
    rocket::build()
        .manage(pool)
        .mount(RANKING, routes![playername])
        .mount(RANKING, routes![gamesplayed])
        .mount(RANKING, routes![wins])
        .mount(RANKING, routes![loses])
        .mount(RANKING, routes![chars])
}
