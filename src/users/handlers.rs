use actix_web::{http::StatusCode, HttpResponse, Json, State};
use regex::Regex;
use serde::Serialize;
use state::State as AppState;
use users::model;

#[derive(Deserialize)]
pub struct CreateUserData {
    username: String,
    email: String,
    password: String,
}

lazy_static! {
    static ref USERNAME_REGEX: Regex = Regex::new("[a-zA-Z0-9_]{2,}").unwrap();
    static ref EMAIL_REGEX: Regex = Regex::new(".+@.+\\..+").unwrap();
}

impl CreateUserData {
    fn is_valid(&self) -> bool {
        USERNAME_REGEX.is_match(&self.username)
            && EMAIL_REGEX.is_match(&self.email)
            && self.password.len() >= 8
    }
}

#[derive(Serialize)]
struct User {
    id: String,
    username: String,
    email: String,
    active: bool,
}

impl User {
    fn from_model(model: model::User) -> User {
        User {
            id: format!("{}", model.id),
            username: model.username,
            email: model.email,
            active: model.active,
        }
    }
}

#[derive(Serialize)]
struct Session {
    token: String,
    expires_at: i64,
    user: User,
}

impl Session {
    fn from_model(user: model::User, session: model::UserSession) -> Session {
        Session {
            token: format!("{}", session.token),
            expires_at: session.expires_at.timestamp(),
            user: User::from_model(user),
        }
    }
}

pub fn create_user(params: (State<AppState>, Json<CreateUserData>)) -> HttpResponse {
    let (state, Json(data)) = params;
    if !data.is_valid() {
        return error(model::Error::Msg("invalid user data provided".into()));
    }

    let conn = state.db.get().unwrap();

    match model::get_by_username_or_email(&conn, data.username.clone(), data.email.clone()) {
        Ok(users) => if users.len() > 0 {
            return error(model::Error::Msg(
                "email or username are already taken".into(),
            ));
        },
        Err(err) => return error(err),
    }

    match model::create_user(&conn, data.username, data.email, data.password) {
        Ok(user) => match model::create_session(&conn, &user) {
            Ok(session) => ok(Session::from_model(user, session)),
            Err(err) => error(err),
        },
        Err(err) => error(err),
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

fn error(err: model::Error) -> HttpResponse {
    let (status, msg) = match err {
        model::Error::QueryError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)),
        model::Error::BcryptError(err) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{}", err)),
        model::Error::Msg(err) => (StatusCode::BAD_REQUEST, err),
    };

    HttpResponse::build(status).json(ErrorResponse { error: msg })
}

fn ok<T: Serialize>(data: T) -> HttpResponse {
    HttpResponse::Ok().json(data)
}
