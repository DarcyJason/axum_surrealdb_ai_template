use redis::aio::MultiplexedConnection;
use surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};

use crate::{config::Config, errors::Result};

pub async fn prepare_surrealdb(config: Config) -> Result<Surreal<Client>> {
    let db: Surreal<Client> = Surreal::<Client>::init();
    db.connect::<Ws>(config.db.surrealdb_host).await?;
    db.signin(Root {
        username: &config.db.surrealdb_root_name,
        password: &config.db.surrealdb_root_password,
    })
    .await?;
    db.use_ns(config.db.surrealdb_namespace)
        .use_db(config.db.surrealdb_database)
        .await?;
    Ok(db)
}

pub async fn prepare_redis(config: Config) -> redis::RedisResult<MultiplexedConnection> {
    let redis_client = redis::Client::open(config.db.redis_host)?;
    let connection = redis_client.get_multiplexed_async_connection().await?;
    Ok(connection)
}
