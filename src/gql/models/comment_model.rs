use async_graphql::{ComplexObject, Context, Error, Result, SimpleObject};
use entity::comment;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

use crate::db::Database;

#[derive(SimpleObject, Serialize, Deserialize)]
#[graphql(complex)]
pub struct CommentResponse {
	id: i32,
	message: String,
	parent_id: Option<i32>,
	thread_id: i32,
	updated_at: String,
	created_at: String,
}

#[ComplexObject]
impl CommentResponse {
	// FIX N+1
	async fn parent(&self, ctx: &Context<'_>) -> Result<Option<CommentResponse>> {
		let db = ctx.data::<Database>().unwrap();

		let data = comment::Entity::find_by_id(match self.parent_id {
			Some(id) => id,
			None => return Ok(None),
		})
		.one(db.get_connection())
		.await
		.map_err(|e| e.to_string())?;

		if let Some(data) = data {
			return Ok(Some(CommentResponse::from_model(data)));
		}
		Err(Error {
			message: "no parent found?".to_string(),
			source: None,
			extensions: None,
		})
	}
}

impl CommentResponse {
	pub fn from_model(model: comment::Model) -> Self {
		Self {
			id: model.id,
			message: model.message,
			parent_id: model.parent_id,
			thread_id: model.thread_id,
			updated_at: model.updated_at.to_string(),
			created_at: model.created_at.to_string(),
		}
	}
}
