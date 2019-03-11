use actix::{Actor, SyncContext};
use diesel::{
    prelude::PgConnection,
    r2d2::{ Pool, ConnectionManager },
};

#[derive(Clone)]
pub struct DbActor{
    pub pool: Pool<ConnectionManager<PgConnection>>,
}


impl Actor for DbActor {
    type Context = SyncContext<Self>;
}
