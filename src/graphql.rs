use async_graphql::{Context, EmptyMutation, EmptySubscription, Object, Schema};
use std::sync::{atomic::Ordering, Arc};

use crate::state::AppState;

pub struct Query;

#[Object]
impl Query {
    async fn hello(&self) -> &str {
        "world"
    }

    async fn stats(&self, ctx: &Context<'_>) -> usize {
        if let Ok(state) = ctx.data::<Arc<AppState>>() {
            state.requests_total.load(Ordering::SeqCst)
        } else {
            0
        }
    }
}

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn build_schema(state: Arc<AppState>) -> AppSchema {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .data(state)
        .finish()
}