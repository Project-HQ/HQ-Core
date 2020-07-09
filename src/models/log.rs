use crate::schema::logs;
use diesel::{PgConnection};
use chrono::{ DateTime, Utc};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
pub struct LogList(pub Vec<Log>);

#[derive(Queryable, Serialize, Deserialize)]
pub struct Log {
    pub id: i32,
    pub device_id: i32,
    pub received_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub int_data: Option<i32>,
    pub str_data: Option<String>,
    pub float_data: Option<f64>,
    pub json_data: Option<Value>,
    pub is_file: Option<bool>
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name="logs"]
pub struct NewLog {
    pub int_data: Option<i32>,
    pub str_data: Option<String>,
    pub float_data: Option<f64>,
    pub json_data: Option<Value>,
    pub is_file: Option<bool>
}

impl LogList {
    pub fn list(connection: &PgConnection) -> Self {
        use diesel::RunQueryDsl;
        use diesel::QueryDsl;
        use crate::schema::logs::dsl::*;

        let result = 
            logs
                .limit(10)
                .load::<Log>(connection)
                .expect("Error loading logs");

        LogList(result)
    }
}

impl NewLog {
    pub fn create(&self, connection: &PgConnection) -> Result<Log, diesel::result::Error> {
        use diesel::RunQueryDsl;

        diesel::insert_into(logs::table)
            .values(self)
            .get_result(connection)
    }
}

impl Log {
    pub fn find(id: &i32, connection: &PgConnection) -> Result<Log, diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;

        logs::table.find(id).first(connection)
    }

    pub fn destroy(id: &i32, connection: &PgConnection) -> Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::logs::dsl;

        diesel::delete(dsl::logs.find(id)).execute(connection)?;
        Ok(())
    }

    pub fn update(id: &i32, new_log: &NewLog, connection: &PgConnection) ->
     Result<(), diesel::result::Error> {
        use diesel::QueryDsl;
        use diesel::RunQueryDsl;
        use crate::schema::logs::dsl;

        diesel::update(dsl::logs.find(id))
            .set(new_log)
            .execute(connection)?;
        Ok(())
    }
}