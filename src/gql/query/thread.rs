use async_graphql::{Context, Object, Result};
use entity::prelude::{Comment, Thread};
use sea_orm::EntityTrait;

use crate::{db::Database, gql::models::thread_model::ThreadResponse};

#[derive(Default)]
pub struct ThreadQuery;

#[Object]
impl ThreadQuery {
	async fn get_threads(&self, ctx: &Context<'_>) -> Result<Vec<ThreadResponse>> {
		let db = ctx.data::<Database>().unwrap();

		Ok(Thread::find()
			.find_with_related(Comment)
			.all(db.get_connection())
			.await
			.map_err(|e| e.to_string())?
			.into_iter()
			.map(ThreadResponse::from_model)
			.collect())
	}
	async fn get_thread_by_id(&self, ctx: &Context<'_>, id: i32) -> Result<ThreadResponse> {
		let db = ctx.data::<Database>().unwrap();

		let db_data = Thread::find_by_id(id)
			.find_with_related(Comment)
			.all(db.get_connection())
			.await
			.map_err(|e| e.to_string())?;

		Ok(ThreadResponse::from_model(db_data[0].clone()))
	}
}
