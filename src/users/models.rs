use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use chrono::offset::Utc;
use chrono::{DateTime, Duration};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use diesel::{delete, insert_into, update};
use schema::{user_sessions, users};
use uuid::Uuid;

#[derive(Debug)]
pub enum Error {
    BcryptError(BcryptError),
    QueryError(DieselError),
    Msg(String),
}

#[derive(Debug, Queryable, Insertable, Identifiable)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    pub active: bool,
}

pub fn create_user(
    conn: &PgConnection,
    username: String,
    email: String,
    password: String,
) -> Result<User, Error> {
    let crypted_password = hash(&password, DEFAULT_COST).map_err(|err| Error::BcryptError(err))?;

    let user = User {
        id: Uuid::new_v4(),
        username: username,
        email: email,
        password: crypted_password,
        active: false,
    };

    insert_into(users::table)
        .values(&user)
        .get_result(conn)
        .map_err(|err| Error::QueryError(err))
}

pub fn get_by_login_details(
    conn: &PgConnection,
    username_or_email: String,
    raw_password: String,
) -> Result<User, Error> {
    users::table
        .filter(users::username.eq(&username_or_email))
        .or_filter(users::email.eq(&username_or_email))
        .first::<User>(conn)
        .map_err(|err| Error::QueryError(err))
        .and_then(|user| match verify(&raw_password, &user.password) {
            Ok(ok) => if ok {
                Ok(user)
            } else {
                Err(Error::Msg("invalid username or password".into()))
            },
            Err(err) => Err(Error::BcryptError(err)),
        })
}

pub fn update_user(
    conn: &PgConnection,
    user: User,
    email: Option<String>,
    password: Option<String>,
) -> Result<usize, Error> {
    let email = email.unwrap_or(user.email.clone());
    let password = match password {
        Some(pwd) => hash(&pwd, DEFAULT_COST).map_err(|err| Error::BcryptError(err))?,
        None => user.password.clone(),
    };

    update(&user)
        .set((users::email.eq(email), users::password.eq(password)))
        .execute(conn)
        .map_err(|e| Error::QueryError(e))
}

pub fn delete_user(conn: &PgConnection, user: User) -> Result<usize, Error> {
    delete(&user)
        .execute(conn)
        .map_err(|e| Error::QueryError(e))
}

#[derive(Debug, Queryable, Insertable)]
pub struct UserSession {
    pub id: Uuid,
    pub token: Uuid,
    pub user_id: Uuid,
    pub expires_at: DateTime<Utc>,
}

pub fn create_session(conn: &PgConnection, user: User) -> Result<UserSession, Error> {
    let session = UserSession {
        id: Uuid::new_v4(),
        token: Uuid::new_v4(),
        user_id: user.id,
        expires_at: Utc::now() + Duration::days(30),
    };

    insert_into(user_sessions::table)
        .values(&session)
        .get_result(conn)
        .map_err(|err| Error::QueryError(err))
}

pub fn get_by_token(conn: &PgConnection, token: Uuid) -> Result<User, Error> {
    users::table
        .inner_join(user_sessions::table)
        .filter(user_sessions::token.eq(&token))
        .filter(user_sessions::expires_at.lt(Utc::now()))
        .select(users::all_columns)
        .first::<User>(conn)
        .map_err(|err| Error::QueryError(err))
}

pub fn delete_expired_sessions(conn: &PgConnection) -> Result<usize, Error> {
    delete(user_sessions::table.filter(user_sessions::expires_at.lt(Utc::now())))
        .execute(conn)
        .map_err(|err| Error::QueryError(err))
}
