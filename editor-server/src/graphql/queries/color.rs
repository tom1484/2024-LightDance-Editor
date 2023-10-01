use async_graphql::{Context, Object, Result as GQLResult};

use crate::server::extractors::Authentication;
use crate::graphql::types::{ColorMap, ColorMapScalar};
use crate::db::types::ColorData;

#[derive(Default)]
pub struct ColorQuery;

#[Object]
impl ColorQuery {
    async fn color_map(&self, ctx: &Context<'_>) -> GQLResult<ColorMap> {
        #[allow(unused)]
        let auth = ctx.data::<Authentication>()?;
        let mysql = &*auth.mysql_pool;
        #[allow(unused)]
        let redis = &*auth.redis_client;

        let result = sqlx::query_as!(
            ColorData,
            r#"
                SELECT * FROM Color;
            "#
        )
        .fetch_all(mysql)
        .await?
        .into_iter()
        .map(|data| (data.id, data.into()))
        .collect();

        Ok(ColorMap {
            color_map: ColorMapScalar(result),
        })
    }
}
