use crate::error;
use bon;
use color_eyre as eyre;
use diesel;
use r2d2;

pub struct Database {
    pub pool: r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::sqlite::SqliteConnection>>,
}

#[bon::bon]
impl Database {
    #[builder]
    pub fn new(database_url: String) -> eyre::Result<Database, error::Error> {
        let conn_manager =
            diesel::r2d2::ConnectionManager::<diesel::sqlite::SqliteConnection>::new(&database_url);
        let pool = r2d2::Pool::builder().build(conn_manager).map_err(|e| {
            error::Error::DatabaseError(format!(
                "failed to open database pool, error: {:#?}",
                e.to_string()
            ))
        })?;

        return Ok(Database { pool });
    }
}
