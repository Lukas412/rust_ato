use std::fmt::Display;
use std::io;
use std::path::Path;
use std::rc::Rc;
use ::{CreationStack, PackProvider};
use core::build::Build;
use core::error::BuildError;
use core::value::Value;
use Creation;

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

  pub(crate) fn create_build<P: AsRef<Path> + ?Sized + Display>(self, path: &P) -> Result<Build, BuildError> {
    let pack_provider = self.pack_provider.clone();
    let creation = Creation::from_file(path)?;
    Ok(Build::new(pack_provider, creation))
  }
}