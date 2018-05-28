use diesel::pg::PgConnection;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;

pub struct State {
    pub db: Pool<ConnectionManager<PgConnection>>,
}
