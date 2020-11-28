use std::fmt::Write;

use crate::parser::Iter;
use crate::xdoc::error::Result;
use crate::PI;

/// https://www.w3.org/TR/xml/#NT-doctypedecl
/// > [28] doctypedecl ::= '<!DOCTYPE' S Name (S ExternalID)? S? ('[' intSubset ']' S?)? '>'
struct DocTypeDecl {
    space_before_name: Whitespace,
    name: DocTypeName,
    external_id: Option<DocTypeDeclSpaceExternalID>,
    space_before_int_subset: Option<Whitespace>,
    int_subsets: Vec<IntSubset>,
    space_after_int_subset: Option<Whitespace>,
}

struct DocTypeDeclSpaceExternalID {
    s: Whitespace,
    external_id: ExternalID,
}

/// Same as any name except without namespace alias prefixes.
struct DocTypeName {
    name: String,
}

/// `#x20` or `' '`
const SPACE: char = ' ';
/// `#x9` or `'\t'`
const TAB: char = '\t';
/// `#xD` or `'\r'`
const CARRIAGE_RETURN: char = '\r';
/// `#0A` pr `'\n'`
const NEWLINE: char = '\n';

struct Whitespace {
    inner: Vec<Space>,
}

enum Space {
    /// `#x20` or `' '`
    Space,
    /// `#x9` or `'\t'`
    Tab,
    /// `#xD` or `'\r'`
    CarriageReturn,
    /// `#0A` pr `'\n'`
    Newline,
}

/// ExternalID ::= 'SYSTEM' S SystemLiteral
///                | 'PUBLIC' S PubidLiteral S SystemLiteral
enum ExternalID {
    System(SystemExternalID),
    Public(PublicExternalID),
}

struct SystemExternalID {
    s: Whitespace,
    system_literal: SystemLiteral,
}

struct PublicExternalID {
    s_before_pub_id: Whitespace,
    pub_id_literal: PubIDLiteral,
    s_after_pub_id: Whitespace,
    system_literal: SystemLiteral,
}

enum Quote {
    /// Something that is quoted with single ('tick') quotation marks: `'`.
    Single,
    /// Something that is quoted with double quotation marks: `"`.
    Double,
}

impl Quote {
    fn parse(c: char) -> crate::parser::error::Result<Self> {
        match c {
            '\'' => Ok(Quote::Single),
            '"' => Ok(Quote::Double),
            _ => parse_err!(iter, "expected either ' or "),
        }
    }
}

/// > SystemLiteral ::= ('"' [^"]* '"') | ("'" [^']* "'")
struct SystemLiteral {
    quote: Quote,
    value: String,
}

/// https://www.w3.org/TR/xml/#NT-PubidLiteral
/// ```text
/// PubidLiteral ::= '"' PubidChar* '"' | "'" (PubidChar - "'")* "'"
/// PubidChar ::= #x20 | #xD | #xA | [a-zA-Z0-9] | [-'()+,./:=?;!*#@$_%]
/// ```
struct PubIDLiteral {
    quote: Quote,
    value: String,
}

/// > intSubset ::= (markupdecl | DeclSep)*
/// In the spec `intSubset` is a vec, however, I've defined the Vec at the usage site
/// ([`DocTypeDecl`]).
enum IntSubset {
    MarkupDecl(MarkupDeclValue),
    DeclSep(DeclSep),
}

/// > DeclSep ::= PEReference | S
enum DeclSep {
    PEReference(PEReferenceValue),
    S(Whitespace),
}

/// https://www.w3.org/TR/xml/#NT-Reference
/// ```text
/// [68] PEReference ::= '&' Name ';'
/// ```
struct ReferenceValue {
    value: DocTypeName,
}

/// https://www.w3.org/TR/xml/#NT-PEReference
/// ```text
/// [69] PEReference ::= '%' Name ';'
/// ```
struct PEReferenceValue {
    value: DocTypeName,
}

/// https://www.w3.org/TR/xml/#NT-markupdecl
/// > markupdecl ::= elementdecl | AttlistDecl | EntityDecl | NotationDecl | PI | Comment
enum MarkupDeclValue {
    ElementDecl(ElementDeclValue),
    AttlistDecl(AttlistDeclValue),
    EntityDecl(EntityDeclValue),
    NotationDecl(NotationDeclValue),
    PI(PI),
    Comment(String),
}

/// > elementdecl ::= '<!ELEMENT' S Name S contentspec S? '>'
struct ElementDeclValue {
    space_before_name: Whitespace,
    name: DocTypeName,
    space_after_name: Whitespace,
    content_spec: ContentSpec,
    space_after_content_spec: Option<Whitespace>,
}

enum ContentSpec {
    Empty,
    Any,
    Mixed(MixedValue),
    Children(ChildrenValue),
}

struct FormattedItem<T> {
    space_before_item: Option<Whitespace>,
    item: T,
}

struct DelimitedListItem<T> {
    space_before_delimiter: Option<Whitespace>,
    item: T,
}

/// > Mixed ::= '(' S? '#PCDATA' (S? '|' S? Name)* S? ')*'
/// >           | '(' S? '#PCDATA' S? ')'
struct MixedValue {
    space_after_open_parenthesis: Option<Whitespace>,
    element_names: Vec<DelimitedListItem<DocTypeName>>,
    space_before_close_parenthesis: Option<Whitespace>,
}

enum Repetitions {
    /// Something may appear once, i.e. it is suffixed with `?`.
    Optional,
    /// Something must appear exactly once, i.e. it does not have a `?`, `*`, or `*`.
    Once,
    /// Something may appear any numer of times, or not at all, i.e. it is suffixed with `*`.
    ZeroOrMany,
    /// Something may appear once or more than once, i.e. it is suffixed with `+`.
    OneOrMany,
}

enum ChildrenType {
    Choice(ChoiceValue),
    Seq(SeqValue),
}

struct ChildrenValue {
    child_type: ChildrenType,
    repetitions: Repetitions,
}

enum CpType {
    Name(DocTypeName),
    Choice(ChoiceValue),
    Seq(SeqValue),
}

struct CpValue {
    cp_type: CpType,
    repetitions: Repetitions,
}

struct FormattedCp {
    space_before_pipe: Option<Whitespace>,
    space_after_pipe: Option<Whitespace>,
    cp: CpValue,
}

struct ChoiceOrSeqContent {
    space_after_open: Option<Whitespace>,
    /// There must be at least 2 in this vec. The first should not have any values for
    /// `space_before_delim` and `space_after_delim`.
    cps: Vec<FormattedCp>,
    space_before_close: Option<Whitespace>,
}

struct ChoiceValue {
    content: ChoiceOrSeqContent,
}

struct SeqValue {
    content: ChoiceOrSeqContent,
}

/// https://www.w3.org/TR/xml/#NT-AttlistDecl
/// AttlistDecl ::= '<!ATTLIST' S Name AttDef* S? '>'
struct AttlistDeclValue {
    space_before_name: Whitespace,
    name: DocTypeName,
    att_defs: Vec<AttDef>,
    space_before_close: Option<Whitespace>,
}

/// https://www.w3.org/TR/xml/#NT-AttDef
/// AttDef ::= S Name S AttType S DefaultDecl
struct AttDef {
    space_before_name: Whitespace,
    name: DocTypeName,
    space_before_att_type: Whitespace,
    att_type: AttType,
    space_before_default_decl: Whitespace,
    default_decl: DefaultDecl,
}

/// https://www.w3.org/TR/xml/#NT-AttType
/// StringType | TokenizedType | EnumeratedType
enum AttType {
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

enum TokenizedType {
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
enum EnumeratedType {
    NotationType(NotationTypeValue),
    Enumeration(EnumerationValue),
}

/// https://www.w3.org/TR/xml/#NT-NotationType
/// NotationType ::= 'NOTATION' S '(' S? Name (S? '|' S? Name)* S? ')'
struct NotationTypeValue {
    space_before_open: Whitespace,
    names: Vec<DelimitedListItem<DocTypeName>>,
    space_before_close: Option<Whitespace>,
}

/// https://www.w3.org/TR/xml/#NT-Enumeration
/// Enumeration ::= '(' S? Nmtoken (S? '|' S? Nmtoken)* S? ')'
struct EnumerationValue {
    names: Vec<DelimitedListItem<DocTypeName>>,
    space_before_close: Option<Whitespace>,
}

/// https://www.w3.org/TR/xml/#NT-EntityDecl
/// EntityDecl ::=  GEDecl | PEDecl
enum EntityDeclValue {
    GEDecl(GEDeclValue),
    PEDecl(PEDeclValue),
}

/// https://www.w3.org/TR/xml/#NT-GEDecl
/// GEDecl ::= '<!ENTITY' S Name S EntityDef S? '>'
struct GEDeclValue {
    space_before_name: Whitespace,
    name: DocTypeName,
    space_before_entity_def: Whitespace,
    entity_def: EntityDef,
    space_before_close: Option<Whitespace>,
}

/// https://www.w3.org/TR/xml/#NT-PEDecl
/// PEDecl ::=  '<!ENTITY' S '%' S Name S PEDef S? '>'
struct PEDeclValue {
    space_before_name: Whitespace,
    name: DocTypeName,
    space_before_pe_def: Whitespace,
    pe_def: PEDef,
    space_before_close: Option<Whitespace>,
}

/// https://www.w3.org/TR/xml/#NT-EntityDef
/// EntityDef ::= EntityValue | (ExternalID NDataDecl?)
enum EntityDef {
    Entity(EntityValue),
    External(EntityDefExternal),
}

/// https://www.w3.org/TR/xml/#NT-EntityDef\
/// (ExternalID NDataDecl?)
struct EntityDefExternal {
    external_id: ExternalID,
    ndata_decl: Option<NDataDecl>,
}

/// https://www.w3.org/TR/xml/#NT-NDataDecl
/// NDataDecl ::= S 'NDATA' S Name
struct NDataDecl {
    space_before_ndata: Whitespace,
    space_before_name: Whitespace,
    name: DocTypeName,
}

/// https://www.w3.org/TR/xml/#NT-PEDef
/// PEDef ::= EntityValue | ExternalID
enum PEDef {
    Entity(EntityValue),
    External(ExternalID),
}

/// https://www.w3.org/TR/xml/#NT-EntityValue
/// ```text
/// [9] EntityValue ::= '"' ([^%&"] | PEReference | Reference)* '"'
///                         | "'" ([^%&'] | PEReference | Reference)* "'"
/// ```
struct EntityValue {
    quote: Quote,
    data: Vec<EntityValueData>,
}

/// https://www.w3.org/TR/xml/#NT-EntityValue
/// ```text
/// [9] EntityValue ::= '"' ([^%&"] | PEReference | Reference)* '"'
///                         | "'" ([^%&'] | PEReference | Reference)* "'"
/// ```
enum EntityValueData {
    String(String),
    PEReference(PEReferenceValue),
    Reference(ReferenceValue),
}

/// https://www.w3.org/TR/xml/#NT-Reference
/// ```text
/// [67] Reference ::= EntityRef | CharRef
/// ```
enum Reference {
    EntityRef(ReferenceValue),
    CharRef(CharRefValue),
}

/// https://www.w3.org/TR/xml/#NT-CharRef
/// ```text
/// [66] CharRef ::= '&#' [0-9]+ ';'
///                   | '&#x' [0-9a-fA-F]+ ';'
/// ```
struct CharRefValue {
    char_ref_type: CharRefValueType,
    value: u64,
}

enum CharRefValueType {
    Decimal,
    Hex,
}

/// https://www.w3.org/TR/xml/#NT-NotationDecl
/// ```text
/// [82] NotationDecl ::= '<!NOTATION' S Name S (ExternalID | PublicID) S? '>'
/// ```
struct NotationDeclValue {
    space_before_name: Whitespace,
    space_before_id: Whitespace,
    id: ExternalOrPublicID,
    space_before_close: Option<Whitespace>,
}

/// https://www.w3.org/TR/xml/#NT-NotationDecl
/// ```text
/// (ExternalID | PublicID)
/// ```
enum ExternalOrPublicID {
    External(ExternalID),
    Public(PublicID),
}

/// https://www.w3.org/TR/xml/#NT-PublicID
/// ```text
/// [83] PublicID ::= 'PUBLIC' S PubidLiteral
/// ```
struct PublicID {
    space_before_name: Whitespace,
    public_id_literal: PubIDLiteral,
}

/// https://www.w3.org/TR/xml/#NT-DefaultDecl
/// ```text
/// [60] DefaultDecl ::= '#REQUIRED' | '#IMPLIED'
///                      | (('#FIXED' S)? AttValue)
/// ```
enum DefaultDecl {
    Required,
    Implied,
}

/// https://www.w3.org/TR/xml/#NT-AttValue
/// ```text
/// [10] AttValue ::= '"' ([^<&"] | Reference)* '"'
///                   | "'" ([^<&'] | Reference)* "'"
/// ```
struct AttValue {
    quote: Quote,
    data: AttValueData,
}

impl AttValue {
    fn parse(&mut iter: Iter) -> crate::parser::error::Result<()> {
        let q = Quote::new(iter.st.c)?;
        iter.advance_or_die()?;
        unimplemented!();
    }

    fn write_opts<W: Write>(&self, w: W, _o: Wst) -> Result<()> {
        unimplemented!();
    }
}

/// https://www.w3.org/TR/xml/#NT-AttValue
/// ```text
/// [10] AttValue ::= '"' ([^<&"] | Reference)* '"'
///                   | "'" ([^<&'] | Reference)* "'"
/// ```
enum AttValueData {
    Text(String),
    Reference(ReferenceValue),
}

/// The state of the writer, including any user-specified formatting options.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Wst {}
