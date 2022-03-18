use async_trait::async_trait;
use eyre::{bail, eyre, Result};
use neo4rs::{Graph, Node, Query};

#[async_trait]
pub trait GraphExt {
    async fn find_one<T: From<Node> + Send>(&self, query: Query) -> Result<T>;
    async fn find_many<T: From<Node> + Send>(&self, query: Query) -> Result<Vec<T>>;
}

#[async_trait]
impl GraphExt for Graph {
    async fn find_one<T: From<Node> + Send>(&self, query: Query) -> Result<T> {
        if let Ok(Some(row)) = self
            .execute(query)
            .await
            .map_err(|e| eyre!("Fail to execute find_one:\n    {:#?}", e))?
            .next()
            .await
        {
            if let Some(node) = row.get::<Node>("m") {
                return Ok(node.into());
            };
        }
        bail!("find_one query return None")
    }

    async fn find_many<T: From<Node> + Send>(&self, query: Query) -> Result<Vec<T>> {
        let mut list: Vec<T> = vec![];
        let mut result = self
            .execute(query)
            .await
            .map_err(|e| eyre!("Fail to execute find_many:\n    {:#?}", e))?;

        while let Ok(Some(row)) = result.next().await {
            if let Some(n) = row.get::<Node>("m") {
                list.push(n.into());
            }
        }

        Ok(list)
    }
}

#[async_trait]
pub trait Transactable {
    async fn transact(&self, conn: &Graph) -> Result<()>;
}
