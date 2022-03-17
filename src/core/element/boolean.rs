mod operation;

pub struct BooleanElement {
  value: bool,
}

impl BooleanElement {
  pub fn new(value: bool) -> BooleanElement {
    BooleanElement { value }
  }
}