use crate::schema::clusters;
use diesel::{PgConnection};
use chrono::{ DateTime, Utc};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct ClusterList(pub Vec<Cluster>);

#[derive(Queryable, Serialize, Deserialize)]
pub struct Cluster {
    pub id: i32,
    pub created_at: DateTime<Utc>,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="clusters"]
pub struct NewCluster {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl ClusterList {
    pub fn list(connection: &PgConnection) -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::clusters::dsl::*;

        let result = 
            clusters
                .limit(10)
                .load::<Cluster>(connection)
                .expect("Error loading clusters");

        ClusterList(result)
    }
}

impl NewCluster {
    pub fn create(&self, connection: &PgConnection) -> Result<Cluster, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(clusters::table)
            .values(self)
            .get_result(connection)
    }
}

impl Cluster {
    pub fn find(id: &i32, connection: &PgConnection) -> Result<Cluster, diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        clusters::table.find(id).first(connection)
    }

    pub fn destroy(id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::clusters::dsl;

        diesel::delete(dsl::clusters.find(id)).execute(connection)?;
        Ok(())
    }

    pub fn update(id: &i32, new_cluster: &NewCluster, connection: &PgConnection) ->
     Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::clusters::dsl;

        diesel::update(dsl::clusters.find(id))
            .set(new_cluster)
            .execute(connection)?;
        Ok(())
    }
}