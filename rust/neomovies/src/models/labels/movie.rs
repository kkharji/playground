use crate::graph_ext::Transactable;
use async_trait::async_trait;
use eyre::{eyre, Result};
use neo4rs::*;

#[derive(Debug)]
pub struct Movie {
    pub id: String,
    pub released: i64,
    pub title: String,
    pub tagline: String,
}

impl Movie {
    pub fn new(title: &str, tagline: &str, released: i64) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            released,
            title: title.into(),
            tagline: tagline.into(),
        }
    }
}

impl From<Node> for Movie {
    fn from(s: Node) -> Self {
        Movie {
            id: s.get::<String>("id").unwrap_or("".into()),
            released: s.get::<i64>("released").unwrap_or(0),
            title: s.get::<String>("title").unwrap_or("".into()),
            tagline: s.get::<String>("tagline").unwrap_or("".into()),
        }
    }
}

#[async_trait]
impl Transactable for Movie {
    async fn transact(&self, conn: &Graph) -> Result<()> {
        let create =
            "create (m:Movie {id: $id, title: $title, tagline: $tagline, released: $released}) return m";
        conn.run(
            query(create)
                .param("id", self.id.clone())
                .param("title", self.title.clone())
                .param("tagline", self.tagline.clone())
                .param("released", self.released),
        )
        .await
        .map_err(|e| eyre!("Fail to transact {:#?} {:#?}", self, e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::conn;
    use crate::graph_ext::GraphExt;

    #[tokio::test]
    async fn find_one() {
        let conn = conn::new().await.unwrap();
        let movie: Movie = conn
            .find_one(
                query("match (m:Movie {title: $title}) return m").param("title", "The Matrix"),
            )
            .await
            .unwrap();
        assert_eq!(movie.title, "The Matrix".to_string())
    }

    #[tokio::test]
    async fn find_many() {
        let conn = conn::new().await.unwrap();
        let movies = conn
            .find_many::<Movie>(
                query("MATCH (tom:Person {name:$name})-[:ACTED_IN]->(m) RETURN m")
                    .param("name", "Tom Hanks"),
            )
            .await
            .unwrap();
        assert!(!movies.is_empty())
    }

    #[tokio::test]
    async fn create_new_movie() {
        let conn = conn::new().await.unwrap();
        conn.run(query("match (m:Movie {title: $title}) delete m").param("title", "The Batman"))
            .await
            .unwrap();

        Movie::new("The Batman", "...", 2021)
            .transact(&conn)
            .await
            .unwrap();

        let find_query =
            query("match (m:Movie {title: $title}) return m").param("title", "The Batman");

        assert_eq!(
            conn.find_one::<Movie>(find_query).await.unwrap().title,
            "The Batman".to_string()
        )
    }
}
