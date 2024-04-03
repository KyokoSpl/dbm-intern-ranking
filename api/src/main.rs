use rocket::http::uri::Origin;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::serde::json::{json, Value};

#[macro_use]
extern crate rocket;

const TAURI_RELEASES_PREFIX: Origin<'static> = uri!("/tauri-releases");

#[get("/")]
fn index() -> Redirect {
    let msg: Option<&str> = None;
    Redirect::to(uri!(
        TAURI_RELEASES_PREFIX,
        google_keep_desktop_api("linux-x86_64", "v1.0.14", msg)
    ))
}
//tauri-releases/google-keep-desktop/win64/1.18.0?msg=""
#[get("/google-keep-desktop/<_platform>/<current_version>?<msg>")]
fn google_keep_desktop_api(
    _platform: &str,
    current_version: &str,
    msg: Option<&str>,
) -> Result<Value, Status> {
    //Status::NoContent
    if let Some(msg) = msg {
        println!("{msg}");
        return Err(Status::NoContent);
    }
    Ok(json!({
        "notes": "IT Works"
    }))
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/tauri-releases", routes![google_keep_desktop_api])
}
