use actix::{Handler, Message};
use crate::db::DbExecutor;
use crate::error;
use crate::error::Error;
use crate::models::User;
use diesel::prelude::*;
use crate::app::users::{OnlineUpdateUser, QueryUser, UpdateGender};

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

impl Message for OnlineUpdateUser {
    type Result = Result<(), error::Error>;
}

impl Handler<OnlineUpdateUser> for DbExecutor {
    type Result = Result<(), error::Error>;

    fn handle(&mut self, msg: OnlineUpdateUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get()?;

        if !users
            .filter(njtech_open_id.eq(&msg.open_id))
            .filter(school.eq("南京工业大学"))
            .first::<User>(conn).is_err() {
            return Err(Error::BadRequest(json!("该账号已经注册")));
        }

        diesel::update(users.filter(id.eq(msg.id)))
            .set((njtech_open_id.eq(msg.open_id), email.eq(msg.email), name.eq(msg.realname), school.eq("南京工业大学")))
            .execute(conn)?;

        Ok(())
    }
}
