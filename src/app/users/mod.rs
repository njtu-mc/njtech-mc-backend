mod form;

use actix_identity::Identity;
use actix_web::{HttpResponse};
use actix_web::web::{Data, Json};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use redis::Commands;
use crate::app::{AppState, get_login_user_id};
use crate::error::Error;
use validator::{Validate};
pub use form::*;
use crate::db::REDIS_CONN;

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryUser {
    pub id: i32,
}

pub async fn get_user(state: Data<AppState>,
                      id: Identity,
) -> Result<HttpResponse, Error> {
    let id = get_login_user_id(id)?;
    let db = state.db.clone();

    let res = db.send(QueryUser { id }).await??;

    Ok(HttpResponse::Ok().json(res))
}

pub async fn put_user_gender(
    state: Data<AppState>,
    id: Identity,
    form: Json<UpdateGender>,
) -> Result<HttpResponse, Error> {
    let mut form = form.into_inner();
    form.validate()?;

    form.id = Some(get_login_user_id(id)?);
    let db = state.db.clone();

    let res = db.send(form).await??;

    Ok(HttpResponse::Ok().json(res))
}

pub async fn logout(id: Identity) -> Result<HttpResponse, Error> {
    id.forget();

    Ok(
        HttpResponse::Ok().finish()
    )
}

pub async fn get_user_authorize(
    id: Identity,
) -> Result<HttpResponse, Error> {
    let code : String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(15)
        .map(char::from)
        .collect();

    let id = get_login_user_id(id)?;
    let _: () = REDIS_CONN.lock()?.set_ex(format!("code:{}", code), id.to_string(), 90).unwrap();

    Ok(
        HttpResponse::Found()
            .append_header(("Location", format!("https://online.njtech.edu.cn/api/v1/minecraft/authorize?code={}", code)))
            .finish()
    )
}
