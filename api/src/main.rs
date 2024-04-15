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
        if msg.len() < 1 {
            // Print an error if the length of msg is below 1
            println!("Error: Message length is below 1");
            return Err(json!({
                "error": "Message length is below 1"
            }));
        }
        println!("{msg}");
        return Ok(json!({
            "Games Played": msg
        }));
    }
    Err(json!({
        "notes": "It doesn't work"
    }))
}

#[post("/games-played?<msg>")]
fn gamesplayed(msg: Option<&str>) -> Result<Value, Value> {
    if let Some(msg) = msg {
        if msg.len() < 1 {
            // Print an error if the length of msg is below 1
            println!("Error: Message length is below 1");
            return Err(json!({
                "error": "Message length is below 1"
            }));
        }
        println!("{msg}");
        return Ok(json!({
            "Games Played": msg
        }));
    }
    Err(json!({
        "notes": "It doesn't work"
    }))
}

#[post("/wins?<msg>")]
fn wins(msg: Option<&str>) -> Result<Value, Value> {
    if let Some(msg) = msg {
        if msg.len() < 1 {
            // Print an error if the length of msg is below 1
            println!("Error: Message length is below 1");
            return Err(json!({
                "error": "Message length is below 1"
            }));
        }
        println!("{msg}");
        return Ok(json!({
            "wins": msg
        }));
    }
    Err(json!({
        "notes": "It doesn't work"
    }))
}

#[post("/loses?<msg>")]
fn loses(msg: Option<&str>) -> Result<Value, Value> {
    if let Some(msg) = msg {
        if msg.len() < 1 {
            // Print an error if the length of msg is below 1
            println!("Error: Message length is below 1");
            return Err(json!({
                "error": "Message length is below 1"
            }));
        }
        println!("{msg}");
        return Ok(json!({
            "loses": msg
        }));
    }
    Err(json!({
        "notes": "It doesn't work"
    }))
}

#[post("/chars?<msg>")]
fn chars(msg: Option<&str>) -> Result<Value, Value> {
    if let Some(msg) = msg {
        if msg.len() < 1 {
            // Print an error if the length of msg is below 1
            println!("Error: Message length is below 1");
            return Err(json!({
                "error": "Message length is below 1"
            }));
        }
        println!("{msg}");
        return Ok(json!({
            "chars": msg
        }));
    }
    Err(json!({
        "notes": "It doesn't work"
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
