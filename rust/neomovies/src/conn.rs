use eyre::{eyre, Result};
use neo4rs::{config, Graph};

pub async fn new() -> Result<Graph> {
    let config = config()
        .uri("127.0.0.1:7687")
        .user("my")
        .password("123")
        .db("neo4j")
        .fetch_size(500)
        .max_connections(10)
        .build()
        .map_err(|e| eyre!("Fail to create custom configuration: {:#?}", e))?;

    let graph = Graph::connect(config.clone())
        .await
        .map_err(|e| eyre!("Fail to connect using {:#?}. Error: {:#?}", config, e))?;

    Ok(graph)
}
