extern crate actix_web;
extern crate cosmos;
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
use actix_web::{http::Method, server, App};
use cosmos::state::State;
use cosmos::users;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use r2d2_diesel::ConnectionManager;
use std::env;

fn main() {
    dotenv().ok();
    let manager =
        ConnectionManager::<PgConnection>::new(env::var("DATABASE_URL").unwrap_or("".into()));
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database connection pool.");

    server::new(move || {
        App::with_state(State { db: pool.clone() }).resource(r"/users/create", |r| {
            r.method(Method::POST).with(users::handlers::create_user)
        })
    }).bind("0.0.0.0:8000")
        .expect("Unable to start server on port 8000")
        .run();
}
