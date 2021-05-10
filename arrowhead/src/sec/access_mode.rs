pub enum AccessMode {
    ByCertificate,
    ByToken,
    Insecure,
    Other(Box<str>),
}