use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::errors::Result;

pub async fn init_surrealdb(db: Surreal<Client>) -> Result<Surreal<Client>> {
    let query = r#"
        DEFINE TABLE user SCHEMAFULL;
    "#;
    db.query(query).await?;
    Ok(db)
}
