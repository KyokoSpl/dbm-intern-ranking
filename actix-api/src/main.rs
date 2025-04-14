use actix_web::{middleware::Logger, web, App, HttpServer};
use dotenv::dotenv;
use mysql::{OptsBuilder, Pool};
use std::sync::Arc;

mod handlers;
mod models;
use handlers::{addplayer, base_stats, default, deletegame, deleteplayer, fighter, game};

fn get_env_var(name: &str) -> String {
    dotenv().ok();
    match std::env::var(name) {
        Ok(val) => val,
        Err(_) => panic!("{} environment variable is not set", name),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let username = get_env_var("MYSQL_USER");
    let password = get_env_var("MYSQL_PASSWORD");
    let host = get_env_var("MYSQL_HOST");
    let database = get_env_var("MYSQL_DATABASE");

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
            .service(
                web::scope("/SOMEDB")
                    .service(game)
                    .service(addplayer)
                    .service(deleteplayer)
                    .service(deletegame)
                    .service(base_stats)
                    .service(fighter),
            )
    })
    .bind(("localhost", 8000))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{http::header::ContentType, test, App};

    use crate::models::{PlayerData, RemovePlayerData};

    use super::*;

    #[test]
    async fn test_getenv() {
        let test_user = get_env_var("MYSQL_USER");
        let test_pw = get_env_var("MYSQL_PASSWORD");
        let test_host = get_env_var("MYSQL_HOST");
        let test_database = get_env_var("MYSQL_DATABASE");

        assert_eq!(test_user, "SOMEUSER");
        assert_ne!(test_pw, "somepw");
        assert_eq!(test_host, "SOMEHOST");
        assert_eq!(test_database, "SOMEDB");
    }

    #[actix_web::test]
    async fn test_index_get() {
        let pool = Pool::new(
            OptsBuilder::new()
                .ip_or_hostname(Some("SOMEHOST"))
                .db_name(Some("SOMEDB"))
                .user(Some("SOMEUSER"))
                .pass(Some("SOMEPW")),
        )
        .expect("failed to create mysql pool");
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(pool.clone())))
                .service(web::scope("/SOMEDB").service(base_stats)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/SOMEDB/base_stats/780712968320843819")
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_index_post() {
        let pool = Pool::new(
            OptsBuilder::new()
                .ip_or_hostname(Some("SOMEHOST"))
                .db_name(Some("SOMEDB"))
                .user(Some("SOMEUSER"))
                .pass(Some("SOMEPW")),
        )
        .expect("failed to create mysql pool");
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(pool.clone())))
                .service(web::scope("/SOMEDB").service(addplayer)),
        )
        .await;
        let data = PlayerData {
            id: 111111111,
            player_name: "testuser".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/SOMEDB/addplayer")
            .insert_header((actix_web::http::header::CONTENT_TYPE, "application/json"))
            .set_json(&data)
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp.status());
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_index_delete() {
        let pool = Pool::new(
            OptsBuilder::new()
                .ip_or_hostname(Some("SOMEHOST"))
                .db_name(Some("SOMEDB"))
                .user(Some("SOMEUSER"))
                .pass(Some("SOMEPW")),
        )
        .expect("failed to create mysql pool");
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(pool.clone())))
                .service(web::scope("/SOMEDB").service(deleteplayer)),
        )
        .await;
        let data = RemovePlayerData { id: 111111111 };

        let req = test::TestRequest::delete()
            .uri("/SOMEDB/deleteplayer")
            .insert_header((actix_web::http::header::CONTENT_TYPE, "application/json"))
            .set_json(&data)
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp.status());
        assert!(resp.status().is_success());
    }
}
