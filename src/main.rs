#![feature(in_band_lifetimes)]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use std::{env, io};

mod app;
mod db;
mod error;

#[actix_rt::main]
async fn main() -> io::Result<()>  {
    dotenv::dotenv().ok();

    if env::var("RUST_LOG").ok().is_none() {
        env::set_var("RUST_LOG", "conduit=debug,actix_web=info");
    }
    env_logger::init();

    app::start().await
}
