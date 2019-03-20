use crate::{
    services::account::domain::account,
    services::common::events::EventMetadata,
    services::account::validators,
    error::KernelError,
};


#[derive(Clone, Debug)]
pub struct Create {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub username: String,
    pub avatar_url: String,
}

impl eventsourcing::Command for Create {
    type Aggregate = account::Account;
    type Event = account::Event;
    type DbConn = diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;
    type Error = KernelError;

    fn validate(&self, _conn: &Self::DbConn, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        validators::first_name(&self.first_name)?;
        validators::last_name(&self.last_name)?;
        validators::password(&self.password)?;
        // TODO: validate email

        // verify that an email isn't already in use
        // verify that username isn't already in use
        return Ok(());
    }

    fn build_event(&self, _conn: &Self::DbConn, aggregate: &Self::Aggregate) -> Result<Self::Event, Self::Error> {
        let now = chrono::Utc::now();
        // TODO: metdata
        let data = account::EventData::CreatedV1(account::CreatedV1{
            id: uuid::Uuid::new_v4(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            password: self.password.clone(),
            avatar_url: self.avatar_url.clone(),
            username: self.username.clone(),
            is_admin: false,
        });
        return  Ok(account::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: now,
            data,
            aggregate_id: aggregate.id,
            metadata: EventMetadata{
                actor_id: None,
                request_id: None,
            },
        });
    }
}
