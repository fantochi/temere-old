use actix::{Actor, Context};

pub struct Chat {

}

impl Actor for Chat {
    type Context = Context<Self>;
}