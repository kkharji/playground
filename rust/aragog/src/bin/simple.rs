use aragog::schema::DatabaseSchema;
use aragog::{DatabaseConnection, DatabaseRecord, Record};
use aragog_playground::DatabaseSchemaBuilder;
use serde::{Deserialize, Serialize};

/// Actor DataShape
#[derive(Serialize, Deserialize, Clone, Record)]
pub struct Actor {
    pub name: String,
    pub date_of_birth: String,
}
// --------------------------------

/// Director DataShape
#[derive(Serialize, Deserialize, Clone, Record)]
pub struct Director {
    pub name: String,
    pub date_of_birth: String,
}
// --------------------------------

/// Director DataShape
#[derive(Serialize, Deserialize, Clone, Record)]
pub struct Movie {
    pub title: String,
    pub release_year: String,
    pub genre: String,
}
// --------------------------------

/// ActedIn DataShape
#[derive(Serialize, Deserialize, Clone, Record)]
pub struct ActedIn {
    pub stage_name: String,
}
/// ActedIn DataShape
#[derive(Serialize, Deserialize, Clone, Record)]
pub struct Directed {}
// --------------------------------

/// Create database connection
async fn new_connection(cred: &[(&str, &str)], schema: DatabaseSchema) -> DatabaseConnection {
    let cred = cred
        .into_iter()
        .map(|c| std::env::var(c.0).unwrap_or_else(|_| c.1.to_string()))
        .collect::<Vec<String>>();

    // Build Database Connection
    DatabaseConnection::builder()
        .with_credentials(&cred[0], &cred[1], &cred[2], &cred[3])
        .with_auth_mode(aragog::AuthMode::Basic)
        .with_schema(schema)
        .apply_schema()
        .build()
        .await
        .unwrap()
}

#[tokio::main]
async fn main() {
    // Construct schema using csutom helper
    let schema = DatabaseSchemaBuilder {
        version: None,
        collections: &[
            ("Actor", false),
            ("Movie", false),
            ("Director", false),
            ("ActedIn", true),
            ("Directed", true),
        ],
        indexes: &[],
        graphs: &[
            ("ActorsInMovies", &[("ActedIn", &["Actor"], &["Movie"])]),
            ("MovieDirectors", &[("Directed", &["Director"], &["Movie"])]),
        ],
    };

    // NOTE: db and user needs to be manually created using `arangosh`.
    let cred = &[
        ("DB_HOST", "http://localhost:8529"),
        ("DB_NAME", "playground_db"),
        ("DB_USER", "playground_user"),
        ("DB_PASSWORD", "playground_password"),
    ];

    // Create database connection
    let connection = new_connection(cred, schema.into()).await;

    // Remove everything
    // connection.truncate().await;

    // Insert Test Data
    insert_test_data(&connection).await.unwrap();
}

async fn insert_test_data(conn: &DatabaseConnection) -> Result<(), aragog::Error> {
    let mut matt_reeves = DatabaseRecord::create(
        Director {
            name: "Matt Reeves".into(),
            date_of_birth: "April 27, 1966".into(),
        },
        conn,
    )
    .await?;
    let mut the_batman = DatabaseRecord::create(
        Movie {
            title: "The Batman".into(),
            release_year: "2022".into(),
            genre: "Action".into(),
        },
        conn,
    )
    .await?;
    let mut cloverfield = DatabaseRecord::create(
        Movie {
            title: "Cloverfield".into(),
            release_year: "2008".into(),
            genre: "Action".into(),
        },
        conn,
    )
    .await?;

    let mut good_time = DatabaseRecord::create(
        Movie {
            title: "Good Time".into(),
            release_year: "2017".into(),
            genre: "Crime".into(),
        },
        conn,
    )
    .await?;
    let mut benny_safdie = DatabaseRecord::create(
        Director {
            name: "Benny Safdie".into(),
            date_of_birth: "Jun 15, 1986".into(),
        },
        conn,
    )
    .await?;

    let mut mad_max = DatabaseRecord::create(
        Movie {
            title: "Mad Max: Fury Road".into(),
            release_year: "2010".into(),
            genre: "Action".into(),
        },
        conn,
    )
    .await?;

    let mut george_miller = DatabaseRecord::create(
        Director {
            name: "George Miller".into(),
            date_of_birth: "Mar 3, 1945".into(),
        },
        conn,
    )
    .await?;

    let mut robert_pattinson = DatabaseRecord::create(
        Actor {
            name: "Robert Pattinson".into(),
            date_of_birth: "May 1, 1986".into(),
        },
        conn,
    )
    .await?;

    let mut robert_pattinson_goodtime_link = DatabaseRecord::link(
        &robert_pattinson,
        &good_time,
        conn,
        ActedIn {
            stage_name: "Connie Nikas".into(),
        },
    )
    .await?;
    let mut robert_pattinson_batman_link = DatabaseRecord::link(
        &robert_pattinson,
        &the_batman,
        conn,
        ActedIn {
            stage_name: "Bruce Wayne".into(),
        },
    )
    .await?;

    let mut zoe_kravitz = DatabaseRecord::create(
        Actor {
            name: "Zoë Kravitz".into(),
            date_of_birth: "Dec 13, 1988".into(),
        },
        conn,
    )
    .await?;
    let mut zoe_batman_link = DatabaseRecord::link(
        &zoe_kravitz,
        &the_batman,
        conn,
        ActedIn {
            stage_name: "Selina Kyle".into(),
        },
    )
    .await?;
    let mut zoe_mad_max_link = DatabaseRecord::link(
        &zoe_kravitz,
        &mad_max,
        conn,
        ActedIn {
            stage_name: "Toast the Knowing".into(),
        },
    )
    .await?;

    // let mut matt_reeves_the_batman_link =
    //     DatabaseRecord::link(&matt_reeves, &the_batman, conn, Directed {}).await?;
    // let mut matt_reeves_the_cloverfield_link =
    //     DatabaseRecord::link(&matt_reeves, &cloverfield, conn, Directed {}).await?;
    // let mut george_miller_max_max_link =
    //     DatabaseRecord::link(&george_miller, &mad_max, conn, Directed {}).await?;
    // let mut benny_safdie_good_time_link =
    //     DatabaseRecord::link(&benny_safdie, &good_time, conn, Directed {}).await?;

    // zoe_mad_max_link.save(conn).await?;
    // zoe_batman_link.save(conn).await?;

    // robert_pattinson_batman_link.save(conn).await?;
    // robert_pattinson_goodtime_link.save(conn).await?;

    // george_miller_max_max_link.save(conn).await?;
    // benny_safdie_good_time_link.save(conn).await?;

    // matt_reeves_the_batman_link.save(conn).await?;
    // matt_reeves_the_cloverfield_link.save(conn).await?;

    // robert_pattinson.save(conn).await?;
    // zoe_kravitz.save(conn).await?;
    // matt_reeves.save(conn).await?;
    // the_batman.save(conn).await?;
    // cloverfield.save(conn).await?;
    // good_time.save(conn).await?;
    // benny_safdie.save(conn).await?;
    // mad_max.save(conn).await?;
    // george_miller.save(conn).await?;

    Ok(())
}
