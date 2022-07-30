use async_graphql::SimpleObject;
use entity::{comment, sea_orm_active_enums::Chats, thread};
use serde::{Deserialize, Serialize};

use super::comment_model::CommentResponse;

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct ThreadResponse {
	id: i32,
	chat: Chats,
	title: String,
	message: String,
	username: Option<String>,
	comments: Vec<CommentResponse>,
	updated_at: String,
	created_at: String,
}

impl ThreadResponse {
	pub fn from_model(data: (thread::Model, Vec<comment::Model>)) -> Self {
		Self {
			id: data.0.id,
			chat: data.0.chat,
			title: data.0.title,
			message: data.0.message,
			username: data.0.username,
			comments: data
				.1
				.into_iter()
				.map(CommentResponse::from_model)
				.collect(),
			updated_at: data.0.updated_at.to_string(),
			created_at: data.0.created_at.to_string(),
		}
	}
}
