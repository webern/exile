use std::fmt::Write;

use crate::xdoc::error::Result;
use crate::PI;

/// https://www.w3.org/TR/xml/#NT-doctypedecl
/// > [28] doctypedecl ::= '<!DOCTYPE' S Name (S ExternalID)? S? ('[' intSubset ']' S?)? '>'
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DocTypeDecl {
    pub(crate) space_before_name: Whitespace,
    pub(crate) name: DocTypeName,
    pub(crate) external_id: Option<DocTypeDeclSpaceExternalID>,
    pub(crate) space_before_int_subset: Option<Whitespace>,
    pub(crate) int_subsets: Vec<IntSubset>,
    pub(crate) space_after_int_subset: Option<Whitespace>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DocTypeDeclSpaceExternalID {
    pub(crate) space_before_id: Whitespace,
    pub(crate) external_id: ExternalID,
}

/// Same as any name except without namespace alias prefixes.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DocTypeName {
    pub(crate) name: String,
}

/// `#x20` or `' '`
pub(crate) const CHAR_SPACE: char = ' ';
/// `#x9` or `'\t'`
pub(crate) const CHAR_TAB: char = '\t';
/// `#xD` or `'\r'`
pub(crate) const CHAR_CARRIAGE_RETURN: char = '\r';
/// `#0A` pr `'\n'`
pub(crate) const CHAR_NEWLINE: char = '\n';

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Whitespace {
    pub(crate) inner: Vec<Space>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Space {
    /// `#x20` or `' '`
    Space,
    /// `#x9` or `'\t'`
    Tab,
    /// `#xD` or `'\r'`
    CarriageReturn,
    /// `#0A` pr `'\n'`
    Newline,
}

pub(crate) const STR_SYSTEM: &str = "SYSTEM";
pub(crate) const STR_PUBLIC: &str = "PUBLIC";

/// ExternalID ::= 'SYSTEM' S SystemLiteral
///                | 'PUBLIC' S PubidLiteral S SystemLiteral
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ExternalID {
    System(SystemExternalID),
    Public(PublicExternalID),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SystemExternalID {
    pub(crate) space_before_literal: Whitespace,
    pub(crate) system_literal: SystemLiteral,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PublicExternalID {
    pub(crate) space_before_pub_id: Whitespace,
    pub(crate) pub_id_literal: PubIDLiteral,
    pub(crate) space_after_pub_id: Whitespace,
    pub(crate) system_literal: SystemLiteral,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Quote {
    /// Something that is quoted with single ('tick') quotation marks: `'`.
    Single,
    /// Something that is quoted with double quotation marks: `"`.
    Double,
}

impl Quote {
    pub(crate) fn new(c: char) -> Result<Self> {
        match c {
            '\'' => Ok(Quote::Single),
            '"' => Ok(Quote::Double),
            _ => raise!("expected either single or double quote character"),
        }
    }

    pub(crate) fn char(&self) -> char {
        match self {
            Quote::Single => '\'',
            Quote::Double => '"',
        }
    }
}

/// > SystemLiteral ::= ('"' [^"]* '"') | ("'" [^']* "'")
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SystemLiteral {
    pub(crate) quote: Quote,
    pub(crate) value: String,
}

/// https://www.w3.org/TR/xml/#NT-PubidLiteral
/// ```text
/// PubidLiteral ::= '"' PubidChar* '"' | "'" (PubidChar - "'")* "'"
/// PubidChar ::= #x20 | #xD | #xA | [a-zA-Z0-9] | [-'()+,./:=?;!*#@$_%]
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PubIDLiteral {
    pub(crate) quote: Quote,
    pub(crate) value: String,
}

impl PubIDLiteral {
    pub(crate) fn forbidden(c: char, q: Quote) -> bool {
        match q {
            Quote::Single if c == '\'' => return true,
            Quote::Double if c == '"' => return true,
            _ => {}
        }
        matches!(c,
           CHAR_SPACE |
           CHAR_CARRIAGE_RETURN |
           CHAR_NEWLINE |
           'a'..='z' |
           'A'..='Z' |
           '0'..='9' |
           '-' |
           '\'' |
           '(' |
           ')' |
           '+' |
           ',' |
           '.' |
           '/' |
           ':' |
           '?' |
           ';' |
           '*' |
           '#' |
           '@' |
           '$' |
           '_' |
           '%'
        )
    }
}

/// > intSubset ::= (markupdecl | DeclSep)*
/// In the spec `intSubset` is a vec, however, I've defined the Vec at the usage site
/// ([`DocTypeDecl`]).
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum IntSubset {
    MarkupDecl(MarkupDeclValue),
    DeclSep(DeclSep),
}

/// > DeclSep ::= PEReference | S
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DeclSep {
    PEReference(PEReferenceValue),
    Space(Whitespace),
}

/// https://www.w3.org/TR/xml/#NT-Reference
/// ```text
/// [68] PEReference ::= '&' Name ';'
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ReferenceValue {
    pub(crate) value: DocTypeName,
}

/// https://www.w3.org/TR/xml/#NT-PEReference
/// ```text
/// [69] PEReference ::= '%' Name ';'
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PEReferenceValue {
    pub(crate) value: DocTypeName,
}

/// https://www.w3.org/TR/xml/#NT-markupdecl
/// > markupdecl ::= elementdecl | AttlistDecl | EntityDecl | NotationDecl | PI | Comment
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum MarkupDeclValue {
    ElementDecl(ElementDeclValue),
    AttlistDecl(AttlistDeclValue),
    EntityDecl(EntityDeclValue),
    NotationDecl(NotationDeclValue),
    PI(PI),
    Comment(String),
}

/// > elementdecl ::= '<!ELEMENT' S Name S contentspec S? '>'
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ElementDeclValue {
    pub(crate) space_before_name: Whitespace,
    pub(crate) name: DocTypeName,
    pub(crate) space_after_name: Whitespace,
    pub(crate) content_spec: ContentSpec,
    pub(crate) space_after_content_spec: Option<Whitespace>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ContentSpec {
    Empty,
    Any,
    Mixed(MixedValue),
    Children(ChildrenValue),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FormattedItem<T> {
    pub(crate) space_before_item: Option<Whitespace>,
    pub(crate) item: T,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DelimitedListItem<T> {
    pub(crate) space_before_delimiter: Option<Whitespace>,
    pub(crate) item: T,
}

/// > Mixed ::= '(' S? '#PCDATA' (S? '|' S? Name)* S? ')*'
/// >           | '(' S? '#PCDATA' S? ')'
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct MixedValue {
    pub(crate) space_after_open_parenthesis: Option<Whitespace>,
    pub(crate) element_names: Vec<DelimitedListItem<DocTypeName>>,
    pub(crate) space_before_close_parenthesis: Option<Whitespace>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Repetitions {
    /// Something may appear once, i.e. it is suffixed with `?`.
    Optional,
    /// Something must appear exactly once, i.e. it does not have a `?`, `*`, or `*`.
    Once,
    /// Something may appear any numer of times, or not at all, i.e. it is suffixed with `*`.
    ZeroOrMany,
    /// Something may appear once or more than once, i.e. it is suffixed with `+`.
    OneOrMany,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ChildrenType {
    Choice(ChoiceValue),
    Seq(SeqValue),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ChildrenValue {
    pub(crate) child_type: ChildrenType,
    pub(crate) repetitions: Repetitions,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CpType {
    Name(DocTypeName),
    Choice(ChoiceValue),
    Seq(SeqValue),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CpValue {
    pub(crate) cp_type: CpType,
    pub(crate) repetitions: Repetitions,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FormattedCp {
    pub(crate) space_before_pipe: Option<Whitespace>,
    pub(crate) space_after_pipe: Option<Whitespace>,
    pub(crate) cp: CpValue,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ChoiceOrSeqContent {
    pub(crate) space_after_open: Option<Whitespace>,
    /// There must be at least 2 in this vec. The first should not have any values for
    /// `space_before_delim` and `space_after_delim`.
    pub(crate) cps: Vec<FormattedCp>,
    pub(crate) space_before_close: Option<Whitespace>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ChoiceValue {
    pub(crate) content: ChoiceOrSeqContent,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SeqValue {
    pub(crate) content: ChoiceOrSeqContent,
}

/// https://www.w3.org/TR/xml/#NT-AttlistDecl
/// AttlistDecl ::= '<!ATTLIST' S Name AttDef* S? '>'
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AttlistDeclValue {
    pub(crate) space_before_name: Whitespace,
    pub(crate) name: DocTypeName,
    pub(crate) att_defs: Vec<AttDef>,
    pub(crate) space_before_close: Option<Whitespace>,
}

/// https://www.w3.org/TR/xml/#NT-AttDef
/// AttDef ::= S Name S AttType S DefaultDecl
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AttDef {
    pub(crate) space_before_name: Whitespace,
    pub(crate) name: DocTypeName,
    pub(crate) space_before_att_type: Whitespace,
    pub(crate) att_type: AttType,
    pub(crate) space_before_default_decl: Whitespace,
    pub(crate) default_decl: DefaultDecl,
}

/// https://www.w3.org/TR/xml/#NT-AttType
/// StringType | TokenizedType | EnumeratedType
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AttType {
    StringType,
    TokenizedType,
    EnumeratedTypes,
}

const STR_ID: &str = "ID";
const STR_IDREF: &str = "IDREF";
const STR_IDREFS: &str = "IDREFS";
const STR_ENTITY: &str = "ENTITY";
const STR_ENTITIES: &str = "ENTITIES";
const STR_NMTOKEN: &str = "NMTOKEN";
const STR_NMTOKENS: &str = "NMTOKENS";

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum TokenizedType {
    ID,
    IDRef,
    IDRefs,
    Entity,
    Entities,
    NMToken,
    NMTokens,
}

/// https://www.w3.org/TR/xml/#NT-EnumeratedType
///
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EnumeratedType {
    NotationType(NotationTypeValue),
    Enumeration(EnumerationValue),
}

/// https://www.w3.org/TR/xml/#NT-NotationType
/// NotationType ::= 'NOTATION' S '(' S? Name (S? '|' S? Name)* S? ')'
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NotationTypeValue {
    pub(crate) space_before_open: Whitespace,
    pub(crate) names: Vec<DelimitedListItem<DocTypeName>>,
    pub(crate) space_before_close: Option<Whitespace>,
}

/// https://www.w3.org/TR/xml/#NT-Enumeration
/// Enumeration ::= '(' S? Nmtoken (S? '|' S? Nmtoken)* S? ')'
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EnumerationValue {
    pub(crate) names: Vec<DelimitedListItem<DocTypeName>>,
    pub(crate) space_before_close: Option<Whitespace>,
}

/// https://www.w3.org/TR/xml/#NT-EntityDecl
/// EntityDecl ::=  GEDecl | PEDecl
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EntityDeclValue {
    GEDecl(GEDeclValue),
    PEDecl(PEDeclValue),
}

/// https://www.w3.org/TR/xml/#NT-GEDecl
/// GEDecl ::= '<!ENTITY' S Name S EntityDef S? '>'
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct GEDeclValue {
    pub(crate) space_before_name: Whitespace,
    pub(crate) name: DocTypeName,
    pub(crate) space_before_entity_def: Whitespace,
    pub(crate) entity_def: EntityDef,
    pub(crate) space_before_close: Option<Whitespace>,
}

/// https://www.w3.org/TR/xml/#NT-PEDecl
/// PEDecl ::=  '<!ENTITY' S '%' S Name S PEDef S? '>'
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PEDeclValue {
    pub(crate) space_before_name: Whitespace,
    pub(crate) name: DocTypeName,
    pub(crate) space_before_pe_def: Whitespace,
    pub(crate) pe_def: PEDef,
    pub(crate) space_before_close: Option<Whitespace>,
}

/// https://www.w3.org/TR/xml/#NT-EntityDef
/// EntityDef ::= EntityValue | (ExternalID NDataDecl?)
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EntityDef {
    Entity(EntityValue),
    External(EntityDefExternal),
}

/// https://www.w3.org/TR/xml/#NT-EntityDef\
/// (ExternalID NDataDecl?)
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EntityDefExternal {
    pub(crate) external_id: ExternalID,
    pub(crate) ndata_decl: Option<NDataDecl>,
}

/// https://www.w3.org/TR/xml/#NT-NDataDecl
/// NDataDecl ::= S 'NDATA' S Name
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NDataDecl {
    pub(crate) space_before_ndata: Whitespace,
    pub(crate) space_before_name: Whitespace,
    pub(crate) name: DocTypeName,
}

/// https://www.w3.org/TR/xml/#NT-PEDef
/// PEDef ::= EntityValue | ExternalID
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PEDef {
    Entity(EntityValue),
    External(ExternalID),
}

/// https://www.w3.org/TR/xml/#NT-EntityValue
/// ```text
/// [9] EntityValue ::= '"' ([^%&"] | PEReference | Reference)* '"'
///                         | "'" ([^%&'] | PEReference | Reference)* "'"
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct EntityValue {
    pub(crate) quote: Quote,
    pub(crate) data: Vec<EntityValueData>,
}

/// https://www.w3.org/TR/xml/#NT-EntityValue
/// ```text
/// [9] EntityValue ::= '"' ([^%&"] | PEReference | Reference)* '"'
///                         | "'" ([^%&'] | PEReference | Reference)* "'"
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum EntityValueData {
    String(String),
    PEReference(PEReferenceValue),
    Reference(ReferenceValue),
}

/// https://www.w3.org/TR/xml/#NT-Reference
/// ```text
/// [67] Reference ::= EntityRef | CharRef
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Reference {
    EntityRef(ReferenceValue),
    CharRef(CharRefValue),
}

/// https://www.w3.org/TR/xml/#NT-CharRef
/// ```text
/// [66] CharRef ::= '&#' [0-9]+ ';'
///                   | '&#x' [0-9a-fA-F]+ ';'
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct CharRefValue {
    pub(crate) char_ref_type: CharRefValueType,
    pub(crate) value: u64,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CharRefValueType {
    Decimal,
    Hex,
}

pub(crate) const STR_NOTATION: &str = "NOTATION";

/// https://www.w3.org/TR/xml/#NT-NotationDecl
/// ```text
/// [82] NotationDecl ::= '<!NOTATION' S Name S (ExternalID | PublicID) S? '>'
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct NotationDeclValue {
    pub(crate) space_before_name: Whitespace,
    pub(crate) name: DocTypeName,
    pub(crate) space_before_id: Whitespace,
    pub(crate) id: ExternalOrPublicID,
    pub(crate) space_before_close: Option<Whitespace>,
}

/// https://www.w3.org/TR/xml/#NT-NotationDecl
/// ```text
/// (ExternalID | PublicID)
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ExternalOrPublicID {
    External(ExternalID),
    Public(PublicID),
}

/// https://www.w3.org/TR/xml/#NT-PublicID
/// ```text
/// [83] PublicID ::= 'PUBLIC' S PubidLiteral
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct PublicID {
    pub(crate) space_before_name: Whitespace,
    pub(crate) public_id_literal: PubIDLiteral,
}

pub(crate) const STR_REQUIRED: &str = "#REQUIRED";
pub(crate) const STR_IMPLIED: &str = "#IMPLIED";
pub(crate) const STR_FIXED: &str = "#FIXED";

/// https://www.w3.org/TR/xml/#NT-DefaultDecl
/// ```text
/// [60] DefaultDecl ::= '#REQUIRED' | '#IMPLIED'
///                      | (('#FIXED' S)? AttValue)
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum DefaultDecl {
    Required,
    Implied,
    Value(DefaultDeclAttValue),
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct DefaultDeclAttValue {
    pub(crate) fixed: Option<Whitespace>,
    pub(crate) att_value: AttValue,
}

/// https://www.w3.org/TR/xml/#NT-AttValue
/// ```text
/// [10] AttValue ::= '"' ([^<&"] | Reference)* '"'
///                   | "'" ([^<&'] | Reference)* "'"
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct AttValue {
    pub(crate) quote: Quote,
    pub(crate) data: AttValueData,
}

impl AttValue {
    fn write_opts<W: Write>(&self, w: W, _o: Wst) -> Result<()> {
        unimplemented!();
    }
}

/// https://www.w3.org/TR/xml/#NT-AttValue
/// ```text
/// [10] AttValue ::= '"' ([^<&"] | Reference)* '"'
///                   | "'" ([^<&'] | Reference)* "'"
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum AttValueData {
    Text(String),
    Reference(ReferenceValue),
}

impl AttValueData {
    pub(crate) fn forbidden(c: char, q: Quote) -> bool {
        match c {
            '\'' if q == Quote::Single => true,
            '"' if q == Quote::Double => true,
            '&' => true,
            '<' => true,
            _ => false,
        }
    }
}

/// The state of the writer, including any user-specified formatting options.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Wst {}
