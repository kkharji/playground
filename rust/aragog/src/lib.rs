use aragog::schema::{CollectionSchema, DatabaseSchema, GraphSchema, IndexSchema};
use arangors_lite::graph::{EdgeDefinition, Graph};
use arangors_lite::index::IndexSettings;

// DatabaseSchema Wrapper to ease declaring schema a pit (should Open PR)
// For example:
// DatabaseSchemaBuilder {
//         version: None,
//         collections: &[
//             ("Actor", false),
//             ("Movie", false),
//             ("Director", false),
//             ("ActedIn", true),
//             ("Directed", true),
//         ],
//         indexes: &[],
//         graphs: &[
//             ("ActorsInMovies", &[("ActedIn", &["Actor"], &["Movie"])]),
//             ("MovieDirectors", &[("Directed", &["Director"], &["Movie"])]),
//         ],
//     }
//     .into()
pub struct DatabaseSchemaBuilder<'a> {
    /// Schema version
    pub version: Option<u64>,
    /// Database collections
    pub collections: &'a [(&'a str, bool)],
    /// Database Collection Indexes (Name, CollectionName, Fields, IndexSettings)
    pub indexes: &'a [(&'a str, &'a str, &'a [&'a str], IndexSettings)],
    /// Database named graphs (Name, Edges(Name, From, To))
    pub graphs: &'a [(&'a str, &'a [(&'a str, &'a [&'a str], &'a [&'a str])])],
}

impl<'a> Into<DatabaseSchema> for DatabaseSchemaBuilder<'a> {
    fn into(self) -> DatabaseSchema {
        DatabaseSchema {
            version: self.version,
            collections: self
                .collections
                .to_vec()
                .into_iter()
                .map(|col| CollectionSchema {
                    name: col.0.to_string(),
                    is_edge_collection: col.1,
                    wait_for_sync: None,
                })
                .collect(),
            indexes: self
                .indexes
                .into_iter()
                .map(|idx| IndexSchema {
                    name: idx.0.to_string(),
                    collection: idx.1.to_string(),
                    fields: idx
                        .2
                        .to_vec()
                        .into_iter()
                        .map(ToString::to_string)
                        .collect(),
                    settings: idx.3.clone(),
                })
                .collect(),
            graphs: self
                .graphs
                .to_vec()
                .into_iter()
                .map(|g| {
                    GraphSchema(Graph {
                        name: g.0.to_string(),
                        edge_definitions: g
                            .1
                            .to_vec()
                            .into_iter()
                            .map(|o| -> EdgeDefinition {
                                EdgeDefinition {
                                    collection: o.0.to_string(),
                                    from: o
                                        .1
                                        .to_vec()
                                        .into_iter()
                                        .map(ToString::to_string)
                                        .collect(),
                                    to: o.2.to_vec().into_iter().map(ToString::to_string).collect(),
                                }
                            })
                            .collect(),
                        orphan_collections: vec![],
                        is_smart: None,
                        is_disjoint: None,
                        options: None,
                    })
                })
                .collect(),
        }
    }
}
