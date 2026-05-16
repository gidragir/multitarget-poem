use poem::{
    get,
    middleware::AddData,
    EndpointExt, IntoEndpoint, Route,
};
use std::sync::Arc;

use crate::{graphql::AppSchema, handlers, state::AppState};

pub fn public_routes(state: Arc<AppState>, schema: AppSchema) -> impl IntoEndpoint {
    Route::new()
        .at("/api", get(handlers::api_handler))
        .at(
            "/graphql",
            get(handlers::graphql_playground).post(handlers::graphql_handler),
        )
        .with(AddData::new(state))
        .with(AddData::new(schema))
}

pub fn internal_routes(state: Arc<AppState>) -> impl IntoEndpoint {
    Route::new()
        .at("/metrics", get(handlers::metrics_handler))
        .at("/health", get(handlers::health_handler))
        .with(AddData::new(state))
}