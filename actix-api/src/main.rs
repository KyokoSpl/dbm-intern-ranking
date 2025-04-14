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
                web::scope("/ranking")
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

    use crate::models::{DelGame, GameInfos, PlayerData, RemovePlayerData};

    use super::*;
    fn create_pool() -> Pool {
        let opts = OptsBuilder::new()
            .ip_or_hostname(Some("SOMEHOST"))
            .db_name(Some("SOMEDB_TEST"))
            .user(Some("SOMEUSER"))
            .pass(Some("SOMEPW"));
        Pool::new(opts).expect("failed to create mysql pool")
    }

    #[test]
    async fn test_getenv() {
        let test_user = get_env_var("MYSQL_USER");
        let test_pw = get_env_var("MYSQL_PASSWORD");
        let test_host = get_env_var("MYSQL_HOST");
        let test_database = get_env_var("MYSQL_DATABASE");

        assert_eq!(test_user, "SOMEUSER");
        assert_ne!(test_pw, "somepass");
        assert_eq!(test_host, "SOMEHOST");
        assert_eq!(test_database, "SOMEDB");
    }

    #[actix_web::test]
    async fn test_base_stats_get() {
        let pool = create_pool();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(pool.clone())))
                .service(web::scope("/ranking").service(base_stats)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/ranking/base_stats/111111111")
            .insert_header(ContentType::plaintext())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_player_post() {
        let pool = create_pool();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(pool.clone())))
                .service(web::scope("/ranking").service(addplayer)),
        )
        .await;
        let data = PlayerData {
            id: 333333333,
            player_name: "testuser3".to_string(),
        };

        let req = test::TestRequest::post()
            .uri("/ranking/addplayer")
            .insert_header((actix_web::http::header::CONTENT_TYPE, "application/json"))
            .set_json(&data)
            .to_request();

        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp.status());
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_player_delete() {
        let pool = create_pool();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(pool.clone())))
                .service(web::scope("/ranking").service(deleteplayer)),
        )
        .await;
        let data = RemovePlayerData { id: 333333333 };

        let req = test::TestRequest::delete()
            .uri("/ranking/deleteplayer")
            .insert_header((actix_web::http::header::CONTENT_TYPE, "application/json"))
            .set_json(&data)
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp.status());
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_game_post() {
        let pool = create_pool();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(pool.clone())))
                .service(web::scope("/ranking").service(game)),
        )
        .await;
        let data = GameInfos {
            player_id: 111111111,
            fighter_id_1: 99,
            wins: 3,
            loses: 1,
        };

        let req = test::TestRequest::post()
            .uri("/ranking/game")
            .insert_header((actix_web::http::header::CONTENT_TYPE, "application/json"))
            .set_json(&data)
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp.status());
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_game_delete() {
        let pool = create_pool();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(Arc::new(pool.clone())))
                .service(web::scope("/ranking").service(deletegame)),
        )
        .await;
        let data = DelGame { id: 936 };

        let req = test::TestRequest::delete()
            .uri("/ranking/delgame")
            .insert_header((actix_web::http::header::CONTENT_TYPE, "application/json"))
            .set_json(&data)
            .to_request();
        let resp = test::call_service(&app, req).await;
        println!("{:?}", resp.status());
        assert!(resp.status().is_success());
    }
}
