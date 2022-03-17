mod operation;

struct StringElement {
  value: String,
}

impl StringElement {
  pub fn new(value: String) -> StringElement {
    StringElement { value }
  }
}