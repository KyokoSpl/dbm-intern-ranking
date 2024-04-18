use rocket::http::uri::Origin;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::{json, Value};

#[macro_use]
extern crate rocket;

const RANKING: Origin<'static> = uri!("/ranking");

#[post("/playername?<msg>")]
fn playername(msg: Option<&str>) -> Result<Value, Value> {
    if let Some(msg) = msg {
        println!("{msg}");
        return Ok(json!({
            "playername": msg
        }));
    }
    Err(json!({
        "notes": "It dosen't work"
    }))
}

#[post("/games-played?<msg>")]
fn gamesplayed(msg: Option<&str>) -> Result<Value, Value> {
    if let Some(msg) = msg {
        println!("{msg}");
        return Ok(json!({
            "Number of Games": msg
        }));
    }
    Err(json!({
        "notes": "It dosen't work"
    }))
}

#[post("/wins?<msg>")]
fn wins(msg: Option<&str>) -> Result<Value, Value> {
    if let Some(msg) = msg {
        println!("{msg}");
        return Ok(json!({
            "wins": msg
        }));
    }
    Err(json!({
        "notes": "It dosen't work"
    }))
}

#[post("/loses?<msg>")]
fn loses(msg: Option<&str>) -> Result<Value, Value> {
    if let Some(msg) = msg {
        println!("{msg}");
        return Ok(json!({
            "loses": msg
        }));
    }
    Err(json!({
        "notes": "It dosen't work"
    }))
}

#[post("/chars?<msg>")]
fn chars(msg: Option<&str>) -> Result<Value, Value> {
    if let Some(msg) = msg {
        println!("{msg}");
        return Ok(json!({
            "Played Character": msg
        }));
    }
    Err(json!({
        "notes": "It dosen't work"
    }))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(RANKING, routes![playername])
        .mount(RANKING, routes![gamesplayed])
        .mount(RANKING, routes![wins])
        .mount(RANKING, routes![loses])
        .mount(RANKING, routes![chars])
}
