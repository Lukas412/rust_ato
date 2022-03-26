mod core;

fn main() {
  let test = "<boolean:parameter xmlns:boolean=\"http://www.ato.net/xmlns/element/boolean\" name=\"yes\"/>";
  println!("{:?}", test);
}
