mod form;

use actix_identity::Identity;
use actix_web::{HttpResponse};
use actix_web::web::{Data, Json};
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;
use redis::Commands;
use crate::app::{AppState, get_login_user_id};
use crate::error::Error;
use validator::Validate;
pub use form::*;
use crate::app::oauth::OauthSetting;
use crate::db::REDIS_CONN;
use crate::util::check_mail_addr;

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

    let user = db.send(form).await??;

    Ok(HttpResponse::Ok().json(user))
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
    let code: String = thread_rng()
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

pub async fn post_user_authorize(
    online: Json<OnlineUpdateUserAuthorize>,
    oauth_setting: Data<OauthSetting>,
    state: Data<AppState>,
) -> Result<HttpResponse, Error> {
    if oauth_setting.online_secret != online.secret {
        return Err(Error::Forbidden);
    }
    let id: String = REDIS_CONN.lock()?.get(format!("code:{}", &online.code))?;
    let id: i32 = id.parse()?;
    let db = state.db.clone();

    db.send(UpdateUserAuthorize {
        id,
        real_name: online.u.real_name.clone(),
        email: online.u.email.clone(),
        open_id: online.u.open_id.clone(),
    }).await??;
    Ok(HttpResponse::Ok().finish())
}

pub async fn put_user_authorize(
    form: Json<EmailUpdateUserAuthorize>,
    id: Identity,
    state: Data<AppState>,
) -> Result<HttpResponse, Error> {
    let form = form.into_inner();
    form.validate()?;
    let id = get_login_user_id(id)?;

    let mut email = form.open_id.clone();
    email.push_str("@njtech.edu.cn");

    check_mail_addr(id, &email, &form.code)?;

    let db = state.db.clone();
    db.send(UpdateUserAuthorize {
        id,
        email,
        open_id: form.open_id.clone(),
        real_name: form.real_name
    }).await??;
    Ok(HttpResponse::Ok().finish())
}
