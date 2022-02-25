pub mod oauth;
pub mod users;
pub mod mail;

use std::string::String;
use crate::db::{new_pool, DbExecutor, new_redis};
use actix::prelude::{Addr, SyncArbiter};
use actix_web::{
    middleware::Logger,
    web::Data,
    web,
    App, HttpRequest,
    HttpServer,
};
use std::{env, io};
use std::num::ParseIntError;
use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::cookie::time::Duration;
use rand::Rng;
use crate::app::oauth::OauthSetting;
use crate::error::Error;

pub struct AppState {
    pub db: Addr<DbExecutor>,
}

async fn index(_req: HttpRequest) -> &'static str {
    "Hello mother fucker!"
}

pub async fn start() -> io::Result<()> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let oauth_setting = OauthSetting {
        client_id: env::var("CLIENT_ID").expect("CLIENT_ID must be set"),
        client_secret: env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set"),
        redirect_url: env::var("REDIRECT_URL").expect("REDIRECT_URL must be set"),
        online_secret: env::var("ONLINE_SECRET").expect("ONLINE_SECRET must be set"),
    };
    let bind_address = match env::var("BIND_ADDRESS") {
        Ok(v) => v,
        _ => String::from("0.0.0.0:8080")
    };

    let database_pool = new_pool(database_url);
    let database_address = SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));
    new_redis();

    let private_key = rand::thread_rng().gen::<[u8; 32]>();

    let server = HttpServer::new(move || {
        let state = AppState {
            db: database_address.clone(),
        };
        App::new()
            .app_data(Data::new(oauth_setting.clone()))
            .app_data(Data::new(state))
            .wrap(Logger::default())
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&private_key)
                    .domain("njtumc.org")
                    .max_age(Duration::days(21))
                    .name("auth")
                    .secure(false),
            ))
            .configure(routes)
    })
        .bind(&bind_address)
        .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address));

    println!("You can access the server at {}", bind_address);
    server.run().await
}

fn routes(app: &mut web::ServiceConfig) {
    app
        .service(web::resource("/").to(index))
        .service(web::resource("/auth")
            .route(web::get().to(oauth::auth))
        )
        .service(web::scope("/api")
            .service(web::resource("user")
                .route(web::get().to(users::get_user))
            )
            .service(web::resource("user/authorize")
                .route(web::get().to(users::get_user_authorize))
                .route(web::post().to(users::post_user_authorize))
            )
            .service(web::resource("user/gender")
                .route(web::put().to(users::put_user_gender))
            )
            .service(web::resource("user/logout")
                .route(web::get().to(users::logout))
            )
            .service(web::resource("email")
                .route(web::put().to(mail::put_mail))
                .route(web::post().to(mail::post_mail))
            )
        );
}

pub fn get_login_user_id(id: Identity) -> Result<i32, Error> {
    let id: Result<i32, ParseIntError> = id.identity().ok_or(Error::Unauthorized)?.parse();
    match id {
        Ok(id) => Ok(id),
        Err(_) => Err(Error::Forbidden),
    }
}
