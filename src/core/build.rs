use std::rc::Rc;
use ::{Creation, PackProvider};
use core::error::BuildError;
use core::value::Value;

pub struct Build {
  pack_provider: Rc<PackProvider>,
  creation: Creation
}

impl Build {
  pub fn new(pack_provider: Rc<PackProvider>, creation: Creation) -> Self {
    Self { pack_provider, creation }
  }

  pub fn build(&mut self) -> Result<Value, BuildError> {
    todo!()
  }
}