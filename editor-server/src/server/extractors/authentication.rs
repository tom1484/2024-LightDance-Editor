use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use redis::Client;
use sqlx::{MySql, Pool};
use std::sync::Arc;

use crate::server::state::AppState;
use crate::APP_STATE;

#[derive(Debug)]
pub struct Authentication {
    pub username: String,
    pub user_id: i32,
    pub mysql_pool: Arc<Pool<MySql>>,
    pub redis_client: Arc<Client>,
}

#[async_trait]
impl<S> FromRequestParts<S> for Authentication
where
    S: Send + Sync,
{
    type Rejection = &'static str;

    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let AppState {
            mysql_pool,
            redis_client,
        } = APP_STATE.get().unwrap();
        let mysql = &**mysql_pool;

        let test_user = sqlx::query!(
            r#"
                SELECT * FROM User ORDER BY id LIMIT 1;
            "#
        )
        .fetch_one(mysql)
        .await;

        if let Ok(test_user) = test_user {
            let _ = sqlx::query!(
                r#"
                    INSERT IGNORE INTO EditingControlFrame
                    (user_id) VALUES (?);
                "#,
                test_user.id
            )
            .execute(mysql)
            .await;

            let _ = sqlx::query!(
                r#"
                    INSERT IGNORE INTO EditingPositionFrame
                    (user_id) VALUES (?);
                "#,
                test_user.id
            )
            .execute(mysql)
            .await;

            let _ = sqlx::query!(
                r#"
                    INSERT IGNORE INTO EditingLEDEffect
                    (user_id) VALUES (?);
                "#,
                test_user.id
            )
            .execute(mysql)
            .await;

            Ok(Authentication {
                username: String::from(test_user.name),
                user_id: test_user.id,
                mysql_pool: mysql_pool.clone(),
                redis_client: redis_client.clone(),
            })
        } else {
            Err("No test user found")
        }
    }
}

impl Drop for Authentication {
    fn drop(&mut self) {
        let mysql_pool = self.mysql_pool.clone();
        let user_id = self.user_id;

        tokio::spawn(async move {
            let mysql = &*mysql_pool;

            let _ = sqlx::query!(
                r#"
                    UPDATE EditingControlFrame SET frame_id = NULL
                    WHERE user_id = ?;
                "#,
                user_id
            )
            .execute(mysql)
            .await;

            let _ = sqlx::query!(
                r#"
                    UPDATE EditingPositionFrame SET frame_id = NULL
                    WHERE user_id = ?;
                "#,
                user_id
            )
            .execute(mysql)
            .await;

            let _ = sqlx::query!(
                r#"
                    UPDATE EditingLEDEffect SET led_effect_id = NULL
                    WHERE user_id = ?;
                "#,
                user_id
            )
            .execute(mysql)
            .await;
        });
    }
}
