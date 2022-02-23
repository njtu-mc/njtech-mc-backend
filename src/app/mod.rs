pub mod oauth;
pub mod users;

use std::string::String;
use crate::db::{new_pool, DbExecutor};
use actix::prelude::{Addr, SyncArbiter};
use actix_web::{
    middleware::Logger,
    web::Data,
    web,
    App, HttpRequest,
    HttpServer,
    http::header::{AUTHORIZATION, CONTENT_TYPE},
};
use actix_cors::Cors;
use std::{env, io};
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::cookie::time::Duration;
use rand::Rng;
use crate::app::oauth::OauthSetting;

pub struct AppState {
    pub db: Addr<DbExecutor>,
    pub oauth_setting: OauthSetting,
}

async fn index(_req: HttpRequest) -> &'static str {
    "Hello mother fucker!"
}

pub async fn start() -> io::Result<()> {
    let frontend_origin = env::var("FRONTEND_ORIGIN").ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let oauth = OauthSetting {
        client_id: env::var("CLIENT_ID").expect("CLIENT_ID must be set"),
        client_secret: env::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set"),
        redirect_url: env::var("REDIRECT_URL").expect("REDIRECT_URL must be set"),
    };
    let bind_address = match env::var("BIND_ADDRESS") {
        Ok(v) => v,
        _ => String::from("0.0.0.0:8080")
    };

    let database_pool = new_pool(database_url);
    let database_address = SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));

    let private_key = rand::thread_rng().gen::<[u8; 32]>();

    let server = HttpServer::new(move || {
        let state = AppState {
            db: database_address.clone(),
            oauth_setting: oauth.clone(),
        };
        let cors = match frontend_origin {
            Some(ref origin) => Cors::default()
                .allowed_origin(origin)
                .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
                .max_age(3600),
            None => Cors::default()
                .allowed_origin("*")
                .send_wildcard()
                .allowed_headers(vec![AUTHORIZATION, CONTENT_TYPE])
                .max_age(3600),
        };
        App::new()
            .app_data(Data::new(state))
            .wrap(Logger::default())
            .wrap(cors)
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
            .service(web::resource("user/logout")
                .route(web::get().to(users::logout))
            )
        );
}
