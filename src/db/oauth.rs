use super::DbExecutor;
use crate::{error};
use crate::models::{NewUser, User};
use actix::prelude::*;
use diesel::prelude::*;
use crate::app::oauth::MCProfileResp;

impl Message for MCProfileResp {
    type Result = Result<i32, error::Error>;
}

impl Handler<MCProfileResp> for DbExecutor {
    type Result = Result<i32, error::Error>;

    fn handle(&mut self, msg: MCProfileResp, _: &mut Self::Context) -> Self::Result {
        Ok(match self.get_user_by_mc_id(&msg.id)? {
            Some(u) => {
                if u.mc_name != msg.name {
                    self.update_user_mc_name_by_id(&msg.name, u.id)?;
                }
                u.id
            }
            None => {
                self.create_user(msg.into())?.id
            }
        })
    }
}

impl DbExecutor {
    fn get_user_by_mc_id(&mut self, _mc_id: &str) -> Result<Option<User>, error::Error> {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get()?;

        Ok(users.filter(mc_id.eq(_mc_id))
            .first::<User>(conn)
            .optional()?)
    }

    fn update_user_mc_name_by_id(&mut self, _mc_name: &str, _id: i32) -> Result<(), error::Error> {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get()?;

        diesel::update(users.filter(id.eq(_id)))
            .set(mc_name.eq(_mc_name))
            .execute(conn)?;
        Ok(())
    }

    fn create_user(&mut self, new_user: NewUser) -> Result<User, error::Error> {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get()?;

        diesel::insert_into(users)
            .values(&new_user)
            .execute(conn)?;

        Ok(users
            .filter(mc_id.eq(&new_user.mc_id))
            .first::<User>(conn)?)
    }
}
