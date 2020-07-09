use crate::schema::device_to_cluster;
use diesel::PgConnection;

#[derive(Serialize, Deserialize)]
pub struct DeviceClusterPairList(pub Vec<DeviceClusterPair>);

#[derive(Queryable, Serialize, Deserialize)]
pub struct DeviceClusterPair {
    pub id: i32,
    pub device_id: i32,
    pub cluster_id: i32
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="device_to_cluster"]
pub struct NewDeviceClusterPair {
    pub device_id: Option<i32>,
    pub cluster_id: Option<i32>
}

impl DeviceClusterPairList {
    pub fn list(connection: &PgConnection) -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::device_to_cluster::dsl::*;

        let result = 
            device_to_cluster
                .limit(10)
                .load::<DeviceClusterPair>(connection)
                .expect("Error loading devices");

        DeviceClusterPairList(result)
    }
}

impl NewDeviceClusterPair {
    pub fn create(&self, connection: &PgConnection) -> Result<DeviceClusterPair, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(device_to_cluster::table)
            .values(self)
            .get_result(connection)
    }
}

impl DeviceClusterPair {
    pub fn find(id: &i32, connection: &PgConnection) -> Result<DeviceClusterPair, diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        device_to_cluster::table.find(id).first(connection)
    }

    pub fn destroy(id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::device_to_cluster::dsl;

        diesel::delete(dsl::device_to_cluster.find(id)).execute(connection)?;
        Ok(())
    }

    pub fn update(id: &i32, new_device_cluster_pair: &NewDeviceClusterPair, connection: &PgConnection) ->
     Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::device_to_cluster::dsl;

        diesel::update(dsl::device_to_cluster.find(id))
            .set(new_device_cluster_pair)
            .execute(connection)?;
        Ok(())
    }
}