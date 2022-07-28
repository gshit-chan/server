use async_graphql::{Context, InputObject, Object, Result};
use chrono::Local;
use entity::{sea_orm_active_enums::Chats, thread};
use sea_orm::{prelude::DateTime, ActiveModelTrait, Set};

use crate::{db::Database, gql::models::thread_model::ThreadResponse};

#[derive(Default)]
pub struct ThreadMutation;

#[derive(InputObject)]
struct CreateThreadInput {
	title: String,
	message: String,
	chat: String,
	username: Option<String>,
}

#[Object]
impl ThreadMutation {
	async fn create_thread(
		&self,
		ctx: &Context<'_>,
		data: CreateThreadInput,
	) -> Result<ThreadResponse> {
		let db = ctx.data::<Database>().unwrap();

		let local_date = Local::now();
		let now = DateTime::new(local_date.date().naive_local(), local_date.time());

		let model = thread::ActiveModel {
			title: Set(data.title),
			message: Set(data.message),
			// TODO: fix this enum problem
			chat: Set(Chats::Global),
			username: Set(data.username),
			// sea-orm still doesn't support `created_at` and `updated_at` fields
			created_at: Set(now),
			updated_at: Set(now),
			..Default::default()
		};

		Ok(ThreadResponse::from_model((
			model.insert(db.get_connection()).await?,
			vec![],
		)))
	}
}
