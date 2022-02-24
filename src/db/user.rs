use actix::{Handler, Message};
use crate::app::users::{QueryUser, UpdateGender};
use crate::db::DbExecutor;
use crate::error;
use crate::error::Error;
use crate::models::User;
use diesel::prelude::*;

impl Message for UpdateGender {
    type Result = Result<User, error::Error>;
}

impl Handler<UpdateGender> for DbExecutor {
    type Result = Result<User, error::Error>;

    fn handle(&mut self, msg: UpdateGender, _: &mut Self::Context) -> Self::Result {
        let id = msg.id.ok_or(Error::InternalServerError)?;

        self.update_user_gender_by_id(&msg.gender, id)?;
        Ok(self.get_user_by_id(id)?.ok_or(Error::Forbidden)?)
    }
}

impl Message for QueryUser {
    type Result = Result<User, error::Error>;
}

impl Handler<QueryUser> for DbExecutor {
    type Result = Result<User, error::Error>;

    fn handle(&mut self, msg: QueryUser, _: &mut Self::Context) -> Self::Result {
        Ok(self.get_user_by_id(msg.id)?.ok_or(Error::Forbidden)?)
    }
}

impl DbExecutor {
    fn update_user_gender_by_id(&mut self, _gender: &i32, _id: i32) -> Result<(), error::Error> {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get()?;

        diesel::update(users.filter(id.eq(_id)))
            .set(gender.eq(_gender))
            .execute(conn)?;
        Ok(())
    }

    fn get_user_by_id(&mut self, _id: i32) -> Result<Option<User>, error::Error> {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get()?;

        Ok(users
            .filter(id.eq(_id))
            .first::<User>(conn)
            .optional()?)
    }
}