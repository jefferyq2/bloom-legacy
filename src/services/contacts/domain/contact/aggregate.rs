use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use crate::{
    db::schema::contacts_contacts,
};
use diesel_as_jsonb::AsJsonb;



    // addresses JSONB,
    // birthday TIMESTAMP WITH TIME ZONE,
    // company TEXT,
    // emails JSONB,
    // first_name TEXT,
    // last_name TEXT,
    // notes TEXT,
    // occupation TEXT,
    // organizations JSONB,
    // phones JSONB,
    // websites JSONB,

    // owner_id UUID NOT NULL REFERENCES account_accounts (id),

#[derive(AsChangeset, Clone, Debug, Deserialize, Identifiable, Insertable, Queryable, Serialize)]
#[table_name = "contacts_contacts"]
#[changeset_options(treat_none_as_null = "true")]
pub struct Contact {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,

    pub addresses: Vec<Address>,
    pub birthday: Option<chrono::DateTime<chrono::Utc>>,
    pub company: Option<String>,
    pub emails: Vec<Email>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub notes: Option<String>,
    pub occupation: Option<String>,
    pub organizations: Vec<Organization>,
    pub phones: Vec<Phone>,
    pub websites: Vec<Website>,

    pub owner_id: uuid::Uuid,
}


#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub struct Address {
    pub city: Option<String>,
    pub country: Option<String>,
    pub label: AddressLabel,
    pub postal_code: Option<String>,
    pub street_address: Option<String>,
    pub street_address2: Option<String>,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum AddressLabel {
    Home,
    Work,
    Other,
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub struct Email {
    pub email: String,
    pub label: EmailLabel,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum EmailLabel {
    Home,
    Work,
    Other,
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub struct Organization {
    pub name: Option<String>,
    pub title: Option<String>,
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub struct Phone {
    pub phone: String,
    pub label: PhoneLabel,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum PhoneLabel {
    Home,
    Work,
    Mobile,
    Main,
    HomeFax,
    WorkFax,
    Other,
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub struct Website {
    pub website: String,
    pub label: WebsiteLabel,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub enum WebsiteLabel {
    Profile,
    Blog,
    HomePage,
    Work,
    Other,
}


impl Contact {
    // create a new, unitialized note
    pub fn new() -> Self {
        let now = chrono::Utc::now();
        return Contact{
            id: uuid::Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            deleted_at: None,
            version: 0,

            addresses: Vec::new(),
            birthday: None,
            company: None,
            emails: Vec::new(),
            first_name: None,
            last_name: None,
            notes: None,
            occupation: None,
            organizations: Vec::new(),
            phones: Vec::new(),
            websites: Vec::new(),

            owner_id: uuid::Uuid::new_v4(),
        };
    }
}

impl eventsourcing::Aggregate for Contact {
    fn increment_version(&mut self) {
        self.version += 1;
    }

    fn update_updated_at(&mut self, timestamp: chrono::DateTime<chrono::Utc>) {
        self.updated_at = timestamp;
    }
}
