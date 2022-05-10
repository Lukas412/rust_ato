pub struct GeneralReferences {
  references: Vec<GeneralReference>,
}

pub enum GeneralReference {
  Action(String),
  Boolean(String),
  Number(String),
  Path(String),
  String(String),
}