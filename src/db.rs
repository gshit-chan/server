use sea_orm::DatabaseConnection;

pub struct Database {
	pub connection: DatabaseConnection,
}

impl Database {
	pub async fn new() -> Self {
		let connection = sea_orm::Database::connect(
			std::env::var("DATABASE_URL").unwrap_or_else(|_| panic!("Couldn't get $DATABASE_URL")),
		)
		.await
		.expect("Could not connect to database");

		Database { connection }
	}

	pub fn get_connection(&self) -> &DatabaseConnection {
		&self.connection
	}
}
