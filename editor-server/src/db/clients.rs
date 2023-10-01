use dotenv::var;
use redis::Client;
use sqlx::{MySql, MySqlPool, Pool};
use std::sync::Arc;

pub async fn build_mysql_pool() -> Arc<Pool<MySql>> {
    let mysql_host = var("MYSQL_URL").expect("MYSQL_URL is not set");
    let mysql_pool = MySqlPool::connect(mysql_host.as_str())
        .await
        .expect("Failed to create mysql pool");

    Arc::new(mysql_pool)
}

pub async fn build_redis_client() -> Arc<Client> {
    let redis_host = var("REDIS_HOST").expect("REDIS_HOST is not set");
    let redis_port = var("REDIS_PORT").expect("REDIS_PORT is not set");

    let redis_client = Client::open(format!("redis://{}:{}", redis_host, redis_port))
        .expect("Failed to create redis client");

    Arc::new(redis_client)
}

