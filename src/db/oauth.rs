use super::DbExecutor;
use crate::app::oauth::MCProfileResp;
use crate::{error};
use crate::models::{NewUser, User};
use actix::prelude::*;
use diesel::prelude::*;
use crate::app::users::QueryUser;
use crate::error::Error;

impl Message for MCProfileResp {
    type Result = Result<User, error::Error>;
}

impl Handler<MCProfileResp> for DbExecutor {
    type Result = Result<User, error::Error>;

    fn handle(&mut self, msg: MCProfileResp, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let conn = &self.0.get()?;

        let user = match users
            .filter(mc_id.eq(&msg.id))
            .first::<User>(conn)
            .optional()? {
            Some(u) => u,
            None => {
                let new_user = NewUser {
                    mc_name: msg.name,
                    mc_id: msg.id.clone(),
                    name: None,
                    email: None,
                    referrer_id: None,
                };
                diesel::insert_into(users)
                    .values(new_user)
                    .execute(conn)?;
                users
                    .filter(mc_id.eq(&msg.id))
                    .first::<User>(conn)?
            }
        };
        Ok(user)
    }
}

impl Message for QueryUser {
    type Result = Result<User, error::Error>;
}

impl Handler<QueryUser> for DbExecutor {
    type Result = Result<User, error::Error>;

    fn handle(&mut self, msg: QueryUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let conn = &self.0.get()?;

        let user = users
            .filter(id.eq(&msg.id))
            .first::<User>(conn)
            .optional()?.ok_or(Error::Forbidden)?;

        Ok(user)
    }
}
