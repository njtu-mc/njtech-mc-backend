use actix::{Handler, Message};
use crate::db::DbExecutor;
use crate::error;
use crate::error::Error;
use crate::models::User;
use diesel::prelude::*;
use crate::app::mail::UpdateMail;
use crate::app::users::{UpdateUserAuthorize, QueryUser, UpdateGender};

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

    fn update_user_mail_by_id(&mut self, _mail: &str, _id: i32) -> Result<(), error::Error> {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get()?;

        diesel::update(users.filter(id.eq(_id)))
            .set(email.eq(_mail))
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

impl Message for UpdateUserAuthorize {
    type Result = Result<(), error::Error>;
}

impl Handler<UpdateUserAuthorize> for DbExecutor {
    type Result = Result<(), error::Error>;

    fn handle(&mut self, msg: UpdateUserAuthorize, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;
        let conn = &self.0.get()?;

        if !users
            .filter(njtech_open_id.eq(&msg.open_id))
            .filter(school.eq("??????????????????"))
            .first::<User>(conn).is_err() {
            return Err(Error::BadRequest(json!("?????????????????????")));
        }

        diesel::update(users.filter(id.eq(msg.id)))
            .set((njtech_open_id.eq(msg.open_id), email.eq(msg.email), name.eq(msg.real_name), school.eq("??????????????????")))
            .execute(conn)?;

        Ok(())
    }
}

impl Message for UpdateMail {
    type Result = Result<User, error::Error>;
}

impl Handler<UpdateMail> for DbExecutor {
    type Result = Result<User, error::Error>;

    fn handle(&mut self, msg: UpdateMail, _: &mut Self::Context) -> Self::Result {
        let id = msg.id.ok_or(Error::InternalServerError)?;

        self.update_user_mail_by_id(&msg.mail, id)?;
        Ok(self.get_user_by_id(id)?.ok_or(Error::Forbidden)?)
    }
}
