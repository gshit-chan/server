use sea_orm::entity::prelude::*;

#[derive(Debug, Clone, PartialEq, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "chats")]
pub enum Chats {
	#[sea_orm(string_value = "GLOBAL")]
	Global,
}
