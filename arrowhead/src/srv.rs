use std::time::Instant;

use crate::{sec, sys};

pub struct Cache {}

pub struct Consumer {}

pub struct Interfaces {}

pub struct Interface {
    pub name: Box<str>,
    pub token: Option<Box<[u8]>>,
}

pub struct Metadata {}

pub struct Provider {}

pub struct Records(Vec<Record>);

pub struct Record {
    pub name: String,
    pub provider: sys::Record,
    pub uri: String,
    pub created_at: Instant,
    pub expires_at: Instant,
    pub access_mode: sec::AccessMode,
    pub metadata: Metadata,
    pub version: usize,
    pub interfaces: Interfaces,
}
