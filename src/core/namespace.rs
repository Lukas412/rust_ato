use std::fmt::{Display, Formatter};
use std::io::Read;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Namespace {
    parts: Vec<Name>,
}

impl Namespace {
    pub(crate) fn new(parts: Vec<Name>) -> Self {
        Self { parts }
    }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut namespace = self.parts.iter();
        if let Some(part) = namespace.next() {
            write!(f, "{}", part)?;
            for part in namespace {
                write!(f, "::{}", part)?;
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct Name {
    value: String,
}

impl Name {
    pub(crate) fn new(value: String) -> Self {
        Self { value }
    }

    fn inner(&self) -> &str {
        &self.value
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct ParameterName {
    namespace: Namespace,
    name: Name,
}

impl ParameterName {
    pub(crate) fn new(namespace: Namespace, name: Name) -> Self {
        Self { namespace, name }
    }

    pub(crate) fn get_namespace(&self) -> &Namespace {
        &self.namespace
    }

    pub(crate) fn get_name(&self) -> &Name {
        &self.name
    }
}

impl Display for ParameterName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.namespace, self.name)
    }
}
