use actix::Addr;
use uuid::Uuid;

use crate::app::client::Client;

#[derive(Debug, Clone)]
pub enum SessionState {
    InChat(Uuid),
    Looking,
    Waiting
}

#[derive(Clone)]
pub struct Session {
    addr: Addr<Client>,
    state: SessionState
}

impl Session {
    pub fn new(client_addr: Addr<Client>) -> Self {
        Self {
            addr: client_addr.clone(),
            state: SessionState::Waiting
        }
    }

    pub fn get_state(&self) -> SessionState {
        self.state.clone()
    }

    pub fn set_state(&mut self, state: SessionState) {
        self.state = state;
    }

    pub fn get_addr(&self) -> Addr<Client> {
        self.addr.clone()
    }
}