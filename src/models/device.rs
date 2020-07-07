use crate::schema::devices;
use diesel::PgConnection;

#[derive(Serialize, Deserialize)]
pub struct DeviceList(pub Vec<Device>);

#[derive(Queryable, Serialize, Deserialize)]
pub struct Device {
    pub id: i32,
    pub name: String,
    pub owner: String,
    pub description: Option<String>
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="devices"]
pub struct NewDevice {
    pub name: Option<String>,
    pub owner: Option<String>,
    pub description: Option<String>
}

impl DeviceList {
    pub fn list(connection: &PgConnection) -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::devices::dsl::*;

        let result = 
            devices
                .limit(10)
                .load::<Device>(connection)
                .expect("Error loading devices");

        DeviceList(result)
    }
}

impl NewDevice {
    pub fn create(&self, connection: &PgConnection) -> Result<Device, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(devices::table)
            .values(self)
            .get_result(connection)
    }
}

impl Device {
    pub fn find(id: &i32, connection: &PgConnection) -> Result<Device, diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        devices::table.find(id).first(connection)
    }

    pub fn destroy(id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::devices::dsl;

        diesel::delete(dsl::devices.find(id)).execute(connection)?;
        Ok(())
    }

    pub fn update(id: &i32, new_device: &NewDevice, connection: &PgConnection) ->
     Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::devices::dsl;

        diesel::update(dsl::devices.find(id))
            .set(new_device)
            .execute(connection)?;
        Ok(())
    }
}