use crate::Syntax;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Default)]
pub struct Metadata {
    pub description: String,
    pub syntax: Syntax,
    pub expected: Option<xdoc::Document>,
}
