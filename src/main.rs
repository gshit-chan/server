use ::async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
	response::{Html, IntoResponse},
	routing::get,
	Extension, Router,
};
use gql::schema::{build_schema, AppSchema};
use std::env::var;

mod db;
mod gql;

#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();

	let schema = build_schema().await;

	let app = Router::new()
		.route("/", get(graphql_playground).post(graphql_handler))
		.layer(Extension(schema));

	let addr = format!("[::]:{}", var("PORT").unwrap_or(String::from("3000")))
		.parse()
		.unwrap();

	axum::Server::bind(&addr)
		.serve(app.into_make_service())
		.await
		.unwrap();
}

async fn graphql_playground() -> impl IntoResponse {
	Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

async fn graphql_handler(schema: Extension<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
	schema.execute(req.into_inner()).await.into()
}
