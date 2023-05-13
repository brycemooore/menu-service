use actix::{Actor, SyncContext};
use crate::database::PostgresPool;

mod menu;

pub struct DbActor(pub PostgresPool);

impl Actor for DbActor {
    type Context = SyncContext<Self>;
}