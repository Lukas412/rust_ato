use std::fmt::Display;
use std::io;
use std::path::Path;
use std::rc::Rc;
use ::{CreationStack, PackProvider};
use core::error::BuildError;
use core::value::Value;
use Creation;

pub struct Builder {
  pack_provider: Rc<PackProvider>,
}

impl Builder {
  pub fn from_root<P: AsRef<Path> + ?Sized>(path: &P) -> io::Result<Self> {
    let pack_provider = Rc::new(PackProvider::from_root(path));
    Ok(Self::new(pack_provider))
  }

  pub fn new(pack_provider: Rc<PackProvider>) -> Self {
    Self { pack_provider }
  }

  pub fn build_creation<P: AsRef<Path> + ?Sized + Display>(self, path: &P) -> Result<Value, BuildError> {
    let mut requirements = CreationStack::default();
    let creation = Creation::rc_from_file(path)?;
    creation.build(&self.pack_provider, &mut requirements)
  }
}