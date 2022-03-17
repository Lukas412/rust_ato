mod operation;

struct BooleanElement {
  value: bool,
}

impl BooleanElement {
  fn new(value: bool) -> BooleanElement {
    BooleanElement { value }
  }
}