use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "comment")]
pub struct Model {
	#[sea_orm(primary_key)]
	pub id: i32,
	#[sea_orm(column_type = "Text")]
	pub message: String,
	#[sea_orm(column_name = "thread_id")]
	pub thread_id: i32,
	#[sea_orm(column_name = "parent_id", nullable)]
	pub parent_id: Option<i32>,
	#[sea_orm(column_name = "updated_at")]
	pub updated_at: DateTime,
	#[sea_orm(column_name = "created_at")]
	pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "Entity",
		from = "Column::ParentId",
		to = "Column::Id",
		on_update = "Cascade",
		on_delete = "SetNull"
	)]
	SelfRef,
	#[sea_orm(
		belongs_to = "super::thread::Entity",
		from = "Column::ThreadId",
		to = "super::thread::Column::Id",
		on_update = "Cascade",
		on_delete = "Restrict"
	)]
	Thread,
}

impl Related<super::thread::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Thread.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
