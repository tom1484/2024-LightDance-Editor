use async_graphql::Schema;
use crate::graphql::{MutationRoot, QueryRoot, SubscriptionRoot};

pub type AppSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub async fn build_schema() -> AppSchema {
    Schema::build(
        QueryRoot::default(),
        MutationRoot::default(),
        SubscriptionRoot::default(),
    )
    .finish()
}
