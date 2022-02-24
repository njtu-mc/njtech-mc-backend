mod form;

use actix_identity::Identity;
use actix_web::{HttpResponse};
use actix_web::web::{Data, Json};
use crate::app::{AppState, get_login_user_id};
use crate::error::Error;
use validator::{Validate};
pub use form::*;

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
        HttpResponse::Found()
            .append_header(("Location", "https://njtumc.org"))
            .finish()
    )
}
