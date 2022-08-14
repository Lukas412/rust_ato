use std::fmt::{Debug, Display};
use std::io;
use std::path::Path;
use std::rc::Rc;
use error_stack::ResultExt;
use crate::{Build, Creation, PackProvider};
use crate::errors::build::BuildError;
use crate::helpers::ser::from::from_file;

pub(crate) struct Builder {
  pack_provider: Rc<PackProvider>,
}

impl Builder {
  pub(crate) fn from_root<P: AsRef<Path> + ?Sized>(path: &P) -> io::Result<Self> {
    let pack_provider = Rc::new(PackProvider::from_root(path));
    Ok(Self::new(pack_provider))
  }

  pub(crate) fn new(pack_provider: Rc<PackProvider>) -> Self {
    Self { pack_provider }
  }

  pub(crate) fn create_build<P: AsRef<Path> + ?Sized>(self, path: &P) -> error_stack::Result<Build, BuildError> {
    let pack_provider = self.pack_provider.clone();

    let creation = from_file(path)
      .change_context(BuildError::default())?;

    Ok(Build::new(pack_provider, creation))
  }
}