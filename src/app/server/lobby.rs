use std::collections::HashMap;
use actix::{Actor, Context};

pub struct Lobby {
    chats: HashMap<String, String>
}

impl Actor for Lobby {
    type Context = Context<Self>;
}