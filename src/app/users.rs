use std::num::ParseIntError;
use actix_identity::Identity;
use actix_web::{HttpResponse};
use actix_web::web::Data;
use crate::app::AppState;
use crate::error::Error;

#[derive(Debug, Deserialize, Serialize)]
pub struct QueryUser {
    pub id: i32,
}

pub async fn get_user(state: Data<AppState>,
                      id: Identity,
) -> Result<HttpResponse, Error> {
    let db = state.db.clone();

    let id: Result<i32, ParseIntError> = id.identity().ok_or(Error::Forbidden)?.parse();
    let id = match id {
        Ok(id) => id,
        Err(_) => {
            return Err(Error::Forbidden);
        }
    };

    let res = db.send(QueryUser { id }).await?;

    match res {
        Err(e) => Err(Error::from(e)),
        Ok(res) => Ok(HttpResponse::Ok().json(res)),
    }
}

pub async fn logout(id: Identity) -> Result<HttpResponse, Error> {
    id.forget();

    Ok(
        HttpResponse::Found()
            .append_header(("Location", "https://njtumc.org"))
            .finish()
    )
}
