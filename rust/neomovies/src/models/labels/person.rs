use crate::graph_ext::Transactable;
use async_trait::async_trait;
use eyre::{eyre, Result};
use neo4rs::*;

#[derive(Debug)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub born: i64,
}

impl Person {
    pub fn new(id: &str, name: &str, born: i64) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            born,
        }
    }
}

impl From<Node> for Person {
    fn from(s: Node) -> Self {
        Self {
            id: s.get::<String>("id").unwrap_or("".into()),
            name: s.get::<String>("name").unwrap_or("".into()),
            born: s.get::<i64>("born").unwrap_or(0),
        }
    }
}

#[async_trait]
impl Transactable for Person {
    async fn transact(&self, conn: &Graph) -> Result<()> {
        let create = "create (m:Person {id: $id, name: $name, born: $born}) return m";
        conn.run(
            query(create)
                .param("id", self.id.clone())
                .param("name", self.name.clone())
                .param("born", self.born.clone()),
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
        let person: Person = conn
            .find_one(query("match (p:Person {name:$name}) return p").param("name", "Tom Hanks"))
            .await
            .unwrap();
        assert_eq!(person.name, "Tom Hanks".to_string())
    }

    #[tokio::test]
    async fn find_many() {
        let conn = conn::new().await.unwrap();
        let actors = conn
            .find_many::<Person>(
                query("MATCH (m:Movie {title :$title})<-[:ACTED_IN]-(p) RETURN p")
                    .param("title", "The Matrix"),
            )
            .await
            .unwrap();
        dbg!(&actors);
        assert!(!actors.is_empty())
    }

    // #[tokio::test]
    // async fn create_new_movie() {
    //     let conn = conn::new().await.unwrap();
    //     conn.run(query("match (m:Movie {title: $title}) delete m").param("title", "The Batman"))
    //         .await
    //         .unwrap();

    //     Movie::new("The Batman", "...", 2021)
    //         .transact(&conn)
    //         .await
    //         .unwrap();

    //     let find_query =
    //         query("match (m:Movie {title: $title}) return m").param("title", "The Batman");

    //     assert_eq!(
    //         conn.find_one::<Movie>(find_query).await.unwrap().title,
    //         "The Batman".to_string()
    //     )
    // }
}
