use crate::core::main::general::reference::GeneralReferences;

#[derive(Debug, Default, YaDeserialize)]
#[yaserde(root, rename = "bundle", prefix = "general", namespace = "general: http://www.ato.net/xmlns/general")]
pub struct GeneralBundle {
  #[yaserde(attribute)]
  namespace: String,
  references: GeneralReferences,
}