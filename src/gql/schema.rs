use async_graphql::{EmptySubscription, Schema};
use migration::{Migrator, MigratorTrait};

use crate::{
	db::Database,
	gql::{mutation::Mutation, query::Query},
};

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn build_schema() -> AppSchema {
	let db = Database::new().await;

	Migrator::up(db.get_connection(), None).await.unwrap();

	Schema::build(Query::default(), Mutation::default(), EmptySubscription)
		.data(db)
		.finish()
}
