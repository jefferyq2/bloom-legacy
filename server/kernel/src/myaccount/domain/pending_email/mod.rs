mod commands;
mod events;
mod aggregate;


pub use aggregate::PendingEmail;
pub use commands::{
    Create,
    Verify,
    Delete,
};
pub use events::{
    Event,
    EventData,
    CreatedV1,
    VerificationFailedReason,
};