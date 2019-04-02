use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use kernel::{
    db::schema::drive_files,
};


#[derive(AsChangeset, Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "drive_files"]
#[changeset_options(treat_none_as_null = "true")]
pub struct File {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,

    pub name: String,
    pub parent_id: Option<uuid::Uuid>,
    pub removed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub size: i64,
     #[serde(rename = "type")]
    pub type_: String, // MIME type

    pub owner_id: uuid::Uuid,
}

impl File {
    // create a new, unitialized File
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return File{
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            version: 0,

            name: String::new(),
            parent_id: None,
            removed_at: None,
            size: 0,
            type_: String::new(),

            owner_id: uuid::Uuid::new_v4(),
        };
    }
}

impl eventsourcing::Aggregate for File {
    fn increment_version(&mut self) {
        self.version += 1;
    }

    fn update_updated_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>) {
        self.updated_at = timestamp;
    }
}
