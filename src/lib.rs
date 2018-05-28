#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate actix_web;
extern crate bcrypt;
extern crate chrono;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate regex;
extern crate serde;
extern crate uuid;

pub mod schema;
pub mod state;
pub mod users;
