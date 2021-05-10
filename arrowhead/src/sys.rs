use std::net;

use crate::{sec, srv};

pub struct System {
    identity: Identity,
}

impl System {
    pub fn consume(&mut self) -> &mut srv::Consumer {
        todo!()
    }

    pub fn consumed_services(&self) -> &srv::Records {
        todo!()
    }

    pub fn identity(&self) -> &Identity {
        return &self.identity;
    }

    pub fn provide(&mut self) -> &mut srv::Provider {
        todo!()
    }

    pub fn provided_services(&self) -> &srv::Records {
        todo!()
    }
}

pub struct Identity {
    pub record_index: usize,
    pub certificate: sec::Certificate,
}

pub struct Record {
    pub name: String,
    pub socket_addr: net::SocketAddr,
    pub public_key: sec::PublicKey,
}

pub struct Records(Vec<Record>);
