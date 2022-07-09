use std::io::Read;

use yaserde::__xml::attribute::OwnedAttribute;
use yaserde::__xml::name::OwnedName;
use yaserde::__xml::namespace::Namespace;
use yaserde::__xml::reader::XmlEvent;
use yaserde::de::Deserializer;

pub fn peek_start_element<R: Read>(reader: &mut Deserializer<R>) -> Result<(&OwnedName, &Namespace, &Vec<OwnedAttribute>), String> {
  let peek = reader.peek()?;
  match peek {
    XmlEvent::StartElement { name, namespace, attributes } =>
      Ok((name, namespace, attributes)),
    _ => Err(format!("Expect StartElement: {:?}", peek))
  }
}

pub fn next_start_element<R: Read>(reader: &mut Deserializer<R>) -> Result<(OwnedName, Namespace, Vec<OwnedAttribute>), String> {
  let next = reader.next()?;
  match next {
    XmlEvent::StartElement { name, namespace, attributes } =>
      Ok((name, namespace, attributes)),
    _ => Err(format!("Expect StartElement: {:?}", next))
  }
}