use std::rc::Rc;
use crate::{Creation, PackProvider};
use crate::core::error::BuildError;
use crate::core::value::Value;

pub(crate) struct Build {
  pack_provider: Rc<PackProvider>,
  creation: Creation
}

impl Build {
  pub(crate) fn new(pack_provider: Rc<PackProvider>, creation: Creation) -> Self {
    Self { pack_provider, creation }
  }

  pub(crate) fn build(&mut self) -> Result<Value, BuildError> {
    todo!()
  }
}