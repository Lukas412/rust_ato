use core::str::Split;
use std::iter;

pub fn compare_namespace(namespaces: (&String, &String)) -> bool {
  let first_namespace_parts = split_namespace(namespaces.0);
  let second_namespace_parts = split_namespace(namespaces.1);
  iter::zip(first_namespace_parts, second_namespace_parts).all(|n| { compare_namespace_part(n) })
}

fn split_namespace(namespace: &String) -> Split<&str> {
  namespace.split("::")
}

fn compare_namespace_part(parts: (&str, &str)) -> bool {
  parts.0 == "*" || parts.1 == "*" || parts.0 == parts.1
}