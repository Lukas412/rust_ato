pub struct Creation {
  uses: CreationUses,
  element: CreationElement,
}

pub struct CreationUses {
  uses: Vec<String>,
}

pub struct CreationElement {
  namespace: String,
  values: CreationValue,
}

pub struct CreationValue {
  name: String,
  value: Option<String>,
  elements: Vec<CreationElement>,
}