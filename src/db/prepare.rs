use surrealdb::{
    Surreal,
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
};

use crate::config::Config;

pub async fn prepare_surrealdb(config: Config) -> Surreal<Client> {
    let db: Surreal<Client> = Surreal::<Client>::init();
    db.connect::<Ws>(config.db.surrealdb_host).await.unwrap();
    db.signin(Root {
        username: &config.db.surrealdb_root_name,
        password: &config.db.surrealdb_root_password,
    })
    .await
    .unwrap();
    db.use_ns(config.db.surrealdb_namespace)
        .use_db(config.db.surrealdb_database)
        .await
        .unwrap();
    db
}
