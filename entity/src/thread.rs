use super::sea_orm_active_enums::Chats;
use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "thread")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: i32,
	pub chat: Chats,
	#[sea_orm(column_type = "Text")]
	pub title: String,
	#[sea_orm(column_type = "Text")]
	pub message: String,
	#[sea_orm(column_type = "Text", nullable)]
	pub username: Option<String>,
	#[sea_orm(column_name = "updated_at")]
	pub updated_at: DateTime,
	#[sea_orm(column_name = "created_at")]
	pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(has_many = "super::comment::Entity")]
	Comment,
}

impl Related<super::comment::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Comment.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
