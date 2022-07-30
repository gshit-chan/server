use ::async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
	http::{header::CONTENT_TYPE, HeaderValue, Method},
	response::{Html, IntoResponse},
	routing::get,
	Extension, Router,
};
use gql::schema::{build_schema, AppSchema};
use std::env::var;
use tower_http::cors::CorsLayer;

mod db;
mod gql;

#[tokio::main]
async fn main() {
	dotenv::dotenv().ok();

	let schema = build_schema().await;

	let mut cors = CorsLayer::new();
	for origin in var("ORIGINS")
		.unwrap_or("http://localhost:3000".to_string())
		.split(";")
	{
		cors = cors.allow_origin(origin.parse::<HeaderValue>().unwrap());
	}

	let app = Router::new()
		.route("/", get(graphql_playground).post(graphql_handler))
		.layer(Extension(schema))
		.layer(
			cors.allow_methods([Method::POST])
				.allow_headers([CONTENT_TYPE]),
		);

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
