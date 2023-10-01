use async_graphql::{Enum, SimpleObject, Subscription};
use futures_core::stream::Stream;

use crate::graphql::subscriptor::Subscriptor;

#[derive(Enum, Clone, Copy, Eq, PartialEq, Default)]
pub enum ColorMutationMode {
    #[default]
    UPDATED,
    CREATED,
    DELETED,
}

#[derive(SimpleObject, Clone, Default)]
pub struct ColorPayload {
    pub mutation: ColorMutationMode,
    pub id: i32,
    pub color: Option<String>,
    pub color_code: Option<Vec<i32>>,
    pub edit_by: i32,
}

#[derive(Default)]
pub struct ColorSubscription;

#[Subscription]
impl ColorSubscription {
    async fn color_subscription(&self) -> impl Stream<Item = ColorPayload> {
        Subscriptor::<ColorPayload>::subscribe()
    }
}
