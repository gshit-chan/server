use async_graphql::Enum;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
	EnumIter, DeriveActiveEnum, Enum, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq,
)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "chats")]
pub enum Chats {
	#[sea_orm(string_value = "GLOBAL")]
	Global,
}
