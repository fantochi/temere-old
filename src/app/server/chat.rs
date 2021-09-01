use std::collections::HashMap;

use actix::{Actor, Context};
use uuid::Uuid;

pub struct Chat {
    id: Uuid,
    members: HashMap<String, String>
}

impl Actor for Chat {
    type Context = Context<Self>;
}