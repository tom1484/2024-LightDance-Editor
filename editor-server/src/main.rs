pub mod db;
pub mod graphql;
pub mod server;

use async_graphql::http::GraphiQLSource;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse, GraphQLSubscription};
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Extension, Router,
};
use dotenv;
use once_cell::sync::OnceCell;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use crate::db::clients::{build_mysql_pool, build_redis_client};
use crate::server::{extractors::Authentication, state::AppState};
use graphql::schema::{build_schema, AppSchema};

async fn graphql(
    auth: Authentication,
    schema: Extension<AppSchema>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner().data(auth)).await.into()
}

async fn graphiql() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/api/graphql")
            .subscription_endpoint("/ws")
            .finish(),
    )
}

pub static APP_STATE: OnceCell<AppState> = OnceCell::new();

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let schema = build_schema().await;
    let mysql_pool = build_mysql_pool().await;
    let redis_client = build_redis_client().await;

    if let Err(write_error) = fs::write(Path::new("src/graphql/schema.graphql"), schema.sdl()) {
        println!("Error writing schema file: {}", write_error);
    }

    APP_STATE
        .set(AppState {
            mysql_pool: Arc::clone(&mysql_pool),
            redis_client: Arc::clone(&redis_client),
        })
        .unwrap();

    let app = Router::new()
        .route("/api/graphql", get(graphiql).post(graphql))
        .route_service("/ws", GraphQLSubscription::new(schema.clone()))
        .layer(Extension(schema));

    #[allow(non_snake_case)]
    let SERVER_PORT = std::env::var("SERVER_PORT").unwrap_or_else(|_| "4000".to_string());

    println!("GraphiQL: http://localhost:{}/graphiql", SERVER_PORT);
    println!("Playground: http://localhost:{}/playground", SERVER_PORT);

    axum::Server::bind(&format!("0.0.0.0:{}", SERVER_PORT).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
