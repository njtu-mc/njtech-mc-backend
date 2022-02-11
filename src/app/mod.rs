// use crate::db::{new_pool, DbExecutor};
use actix::prelude::{Addr, SyncArbiter};
use actix_web::{
    get,
    middleware::Logger,
    web::Data,
    web,
    App, HttpRequest,
    HttpServer,
    http::header::{AUTHORIZATION, CONTENT_TYPE},
};
use actix_cors::Cors;
use std::{env, io};

pub struct AppState {
    // pub db: Addr<DbExecutor>,
}

#[get("/")]
async fn index(_req: HttpRequest) -> &'static str {
    "Hello 1!"
}

pub async fn start() -> io::Result<()>  {
    let frontend_origin = env::var("FRONTEND_ORIGIN").ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    // let database_pool = new_pool(database_url).expect("Failed to create pool.");
    // let database_address = SyncArbiter::start(num_cpus::get(), move || DbExecutor(database_pool.clone()));

    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    let server = HttpServer::new(move || {
        // let state = AppState {
        //      db: database_address.clone(),
        // };
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
            // .register_data(Data::new(state))
            .wrap(Logger::default())
            .wrap(cors)
            .configure(routes)
    })
        .bind(&bind_address)
        .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address));

    println!("You can access the server at {}", bind_address);
    server.run().await
}


fn routes(app: &mut web::ServiceConfig) {
    app
        .service(index)
        .service(web::scope("/api")
            // User routes ↓
        );
}
