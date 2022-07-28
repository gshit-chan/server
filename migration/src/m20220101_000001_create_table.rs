use sea_orm_migration::{
	prelude::*,
	sea_orm::{DbBackend, Schema},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		let schema = Schema::new(DbBackend::Postgres);

		manager
			.create_table(schema.create_table_from_entity(entity::thread::Entity))
			.await?;

		manager
			.create_table(schema.create_table_from_entity(entity::comment::Entity))
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(entity::thread::Entity).to_owned())
			.await?;

		manager
			.drop_table(Table::drop().table(entity::comment::Entity).to_owned())
			.await
	}
}
