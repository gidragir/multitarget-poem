use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    Request, Response,
};
use poem::{
    handler,
    web::{Data, Html, Json},
};
use std::sync::{atomic::Ordering, Arc};

use crate::{graphql::AppSchema, state::AppState};

#[handler]
pub async fn api_handler(state: Data<&Arc<AppState>>) -> String {
    state.requests_total.fetch_add(1, Ordering::SeqCst);
    "API response".to_string()
}

#[handler]
pub async fn metrics_handler(state: Data<&Arc<AppState>>) -> String {
    let count = state.requests_total.load(Ordering::SeqCst);
    format!(
        "# HELP requests_total Total requests\n\
         # TYPE requests_total counter\n\
         requests_total {}",
        count
    )
}

#[handler]
pub fn health_handler() -> &'static str {
    "OK"
}

#[handler]
pub async fn graphql_handler(
    schema: Data<&AppSchema>,
    req: Json<Request>,
) -> Json<Response> {
    Json(schema.execute(req.0).await)
}

#[handler]
pub fn graphql_playground() -> Html<String> {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        "/graphql",
    )))
}