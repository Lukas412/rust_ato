use serde::Deserialize;
use std::io::Read;

#[derive(Debug, Default, Deserialize)]
pub(crate) struct GeneralReferences {
    references: Vec<GeneralReference>,
}

#[derive(Debug)]
pub(crate) enum GeneralReference {
    Action(String),
    Boolean(String),
    Number(String),
    Path(String),
    String(String),
}
