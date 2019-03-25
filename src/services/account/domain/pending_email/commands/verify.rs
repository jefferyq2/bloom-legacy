use crate::{
    services::common::events::EventMetadata,
    services::account::domain::pending_email,
    services::account::validators,
    error::KernelError,
};
use serde::{Serialize, Deserialize};
use chrono::Utc;
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Verify {
    pub id: uuid::Uuid,
    pub code: String,
    pub email: String,
    pub metadata: EventMetadata,
}


impl<'a> eventsourcing::Command<'a> for Verify {
    type Aggregate = pending_email::PendingEmail;
    type Event = pending_email::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        use crate::db::schema::{
            account_accounts::dsl::*,
        };
        use diesel::prelude::*;

        validators::email(&self.email)?;

        // verify that an email isn't already in use
        let existing_email: i64 = account_accounts
            .filter(email.eq(&self.email))
            .filter(deleted_at.is_null())
            .count()
            .get_result(ctx)?;
        if existing_email != 0 {
            return Err(KernelError::Validation(format!("Email: {} is already in use.", &self.email)));
        }
        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let metadata = self.metadata.clone();
        let timestamp = Utc::now();
        let duration = aggregate.created_at.signed_duration_since(timestamp);

        let data = if aggregate.trials + 1 >= 8 {
            pending_email::EventData::VerificationFailedV1(pending_email::VerificationFailedV1{
                reason: "Maximum number of trials reached. Please create another account.".to_string(),
            })
        } else if !bcrypt::verify(&self.code, &aggregate.token).map_err(|_| KernelError::Bcrypt)? {
            // verify given code
            pending_email::EventData::VerificationFailedV1(pending_email::VerificationFailedV1{
                reason: "Code is not valid.".to_string(),
            })
        } else if duration.num_minutes() > 30 {
            // verify code expiration
            pending_email::EventData::VerificationFailedV1(pending_email::VerificationFailedV1{
                reason: "Code has expired. Please create another account.".to_string(),
            })
        } else {
            pending_email::EventData::VerificationSucceededV1
        };

        return  Ok((pending_email::Event{
            id: uuid::Uuid::new_v4(),
            timestamp,
            data,
            aggregate_id: aggregate.id,
            metadata,
        }, ()));
    }
}

