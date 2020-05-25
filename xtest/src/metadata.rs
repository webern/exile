use crate::Syntax;

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq, Default)]
pub struct Metadata<'a> {
    pub description: String,
    pub syntax: Syntax,
    pub expected: Option<xdoc::Document<'a>>,
}
