use std::sync::Arc;
use redis::Client;
use sqlx::{MySql, Pool};

#[derive(Clone, Debug)]
pub struct AppState {
    pub mysql_pool: Arc<Pool<MySql>>,
    pub redis_client: Arc<Client>,
}


