use crate::schema::device_to_cluster;
use crate::models::device::{Device, DeviceList};
use crate::models::device;

use diesel::PgConnection;


#[derive(Queryable, Serialize, Deserialize)]
pub struct DeviceClusterPair {
    pub id: i32,
    pub device_id: i32,
    pub cluster_id: i32
}
#[derive(Serialize, Deserialize)]
pub struct DeletedPairReturn {
    success: bool
}
#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="device_to_cluster"]
pub struct NewDeviceClusterPair {
    pub device_id: Option<i32>,
    pub cluster_id: Option<i32>
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

    pub fn delete_by_pair(pair: &NewDeviceClusterPair, connection: &PgConnection) -> Result<DeletedPairReturn, diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::diesel::ExpressionMethods;

        let  num_affected = diesel::delete(device_to_cluster::table.filter(device_to_cluster::device_id.eq(pair.device_id.unwrap()))
                                        .filter(device_to_cluster::cluster_id.eq(pair.cluster_id.unwrap())))
                                        .execute(connection).unwrap();

        return Ok(DeletedPairReturn{success: num_affected > 0})
    }

    pub fn find_device_relationships(c_id: &i32, connection: &PgConnection) -> Result<DeviceList, diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::diesel::ExpressionMethods;


        let device_pairs: Vec<DeviceClusterPair> = device_to_cluster::table.filter(device_to_cluster::cluster_id.eq(c_id)).get_results(connection).unwrap();

        let mut devices: Vec<Device>= Vec::new();

        for device_pair in device_pairs{
            let device: Device  = Device::find(&device_pair.device_id, &connection).unwrap();
            devices.push(device);
        }
        Ok(device::DeviceList(devices))
    }

    pub fn destroy(id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::device_to_cluster::dsl;

        diesel::delete(dsl::device_to_cluster.find(id)).execute(connection)?;
        Ok(())
    }

}