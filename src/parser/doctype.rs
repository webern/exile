use crate::parser::bang::parse_comment;
use crate::parser::pi::parse_pi;
use crate::parser::Iter;
use crate::xdoc::xdocv2::doctype::{
    AttDef, AttType, AttValue, AttValueData, AttlistDeclValue, CharRefValue, CharRefValueType,
    ChildrenType, ChildrenValue, ChoiceValue, ContentSpec, CpItem, CpValue, DeclSep, DefaultDecl,
    DefaultDeclAttValue, DelimitedListItem, DocTypeDecl, DocTypeDeclSpaceExternalID, DocTypeName,
    ElementDeclValue, EntityDeclValue, EntityDef, EntityDefExternal, EntityValue, EntityValueData,
    EnumeratedType, EnumerationValue, ExternalID, ExternalOrPublicID, GEDeclValue, IntSubset,
    MarkupDeclValue, MixedValue, NDataDecl, NmToken, NotationDeclValue, NotationTypeValue,
    PEDeclValue, PEDef, PEReferenceValue, PubIDLiteral, PublicExternalID, PublicID, Quote,
    Reference, ReferenceValue, Repetitions, SeqValue, Space, SystemExternalID, SystemLiteral,
    Whitespace, CHAR_CARRIAGE_RETURN, CHAR_NEWLINE, CHAR_SPACE, CHAR_TAB, STR_ANY, STR_ATTLIST,
    STR_CDATA, STR_ELEMENT, STR_EMPTY, STR_ENTITY, STR_FIXED, STR_IMPLIED, STR_NDATA, STR_NMTOKEN,
    STR_NOTATION, STR_PCDATA, STR_REQUIRED,
};

use super::error::Result;

impl DocTypeDecl {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl DocTypeDeclSpaceExternalID {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl DocTypeName {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl Whitespace {
    pub(crate) fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        Self::parse_optional(iter)
            .ok_or_else(|| create_parser_error!(&iter.st, "expected whitespace"))
    }

    pub(crate) fn parse_optional(iter: &mut Iter<'_>) -> Option<Self> {
        let mut w = Vec::new();
        if !iter.is_whitespace() {
            return None;
        }
        loop {
            match iter.st.c {
                CHAR_SPACE => w.push(Space::Space),
                CHAR_TAB => w.push(Space::Tab),
                CHAR_NEWLINE => w.push(Space::Newline),
                CHAR_CARRIAGE_RETURN => w.push(Space::CarriageReturn),
                _ => break,
            }
            if !iter.advance() {
                break;
            }
        }
        Some(Self { inner: w })
    }
}

impl Space {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl ExternalID {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl SystemExternalID {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl PublicExternalID {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl Quote {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl SystemLiteral {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl PubIDLiteral {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        let q = Quote::new(iter.st.c).map_err(|e| from_xe!(iter, e))?;
        iter.advance_or_die()?;
        let mut s = String::new();
        loop {
            if Self::forbidden(iter.st.c, q) {
                return parse_err!(iter, "forbidden character in pubid literal");
            } else if iter.st.c == q.char() {
                break;
            }
            s.push(iter.st.c);
            iter.advance_or_die()?
        }
        iter.advance();
        Ok(Self {
            quote: Quote::Single,
            value: s,
        })
    }
}

impl IntSubset {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        if iter.is('<') {
            Ok(IntSubset::MarkupDecl(MarkupDeclValue::parse(iter)?))
        } else {
            Ok(IntSubset::DeclSep(DeclSep::parse(iter)?))
        }
    }
}

impl DeclSep {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        if iter.is_whitespace() {
            Ok(DeclSep::Space(Whitespace::parse(iter)?))
        } else {
            Ok(DeclSep::PEReference(PEReferenceValue::parse(iter)?))
        }
    }
}

impl ReferenceValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        debug_assert!(iter.is('&'));
        iter.advance_or_die()?;
        let name = DocTypeName::parse(iter)?;
        expect!(iter, ';');
        iter.advance();
        Ok(Self { value: name })
    }
}

impl PEReferenceValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        debug_assert!(iter.is('%'));
        iter.advance_or_die()?;
        let name = DocTypeName::parse(iter)?;
        expect!(iter, ';');
        iter.advance();
        Ok(Self { value: name })
    }
}

impl MarkupDeclValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        debug_assert!(iter.is('<'));
        iter.advance_or_die()?;
        match iter.st.c {
            '!' => match iter.peek_or_die()? {
                'A' => Ok(MarkupDeclValue::AttlistDecl(AttlistDeclValue::parse(iter)?)),
                'E' => {
                    iter.advance_or_die()?;
                    match iter.peek_or_die()? {
                        'L' => {
                            iter.consume(STR_ELEMENT)?;
                            Ok(MarkupDeclValue::ElementDecl(ElementDeclValue::parse(iter)?))
                        }
                        'N' => {
                            iter.consume(STR_ENTITY)?;
                            Ok(MarkupDeclValue::EntityDecl(EntityDeclValue::parse(iter)?))
                        }
                        _ => parse_err!(iter, "expected {} or {}", STR_ELEMENT, STR_ENTITY),
                    }
                }
                'N' => Ok(MarkupDeclValue::NotationDecl(NotationDeclValue::parse(
                    iter,
                )?)),
                '-' => Ok(MarkupDeclValue::Comment(parse_comment(iter)?)),
                _ => parse_err!(
                    iter,
                    "expected {}, {}, {}, {} or comment",
                    STR_ATTLIST,
                    STR_ELEMENT,
                    STR_ENTITY,
                    STR_NOTATION
                ),
            },
            '?' => Ok(MarkupDeclValue::PI(parse_pi(iter)?)),
            _ => parse_err!(
                iter,
                "expected {}, {}, {}, {}, processing instruction or comment",
                STR_ATTLIST,
                STR_ELEMENT,
                STR_ENTITY,
                STR_NOTATION
            ),
        }
    }
}

impl ElementDeclValue {
    /// > elementdecl ::= '<!ELEMENT' S Name S contentspec S? '>'
    /// expects iter at the first space following `<!ELEMENT`
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        iter.consume(STR_ELEMENT)?;
        Ok(Self {
            space_before_name: Whitespace::parse(iter)?,
            name: DocTypeName::parse(iter)?,
            space_after_name: Whitespace::parse(iter)?,
            content_spec: ContentSpec::parse(iter)?,
            space_after_content_spec: Whitespace::parse_optional(iter),
        })
    }
}

impl ContentSpec {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        match iter.st.c {
            'E' => {
                iter.consume(STR_EMPTY)?;
                Ok(ContentSpec::Empty)
            }
            'A' => {
                iter.consume(STR_ANY)?;
                Ok(ContentSpec::Any)
            }
            '(' => {
                let mystery = parse_mystery(iter)?;
                match mystery {
                    ParsedMystery::ChoiceOrSeq(c_or_s) => match c_or_s.t {
                        ParsedChoiceOrSeqType::Choice => Ok(ContentSpec::Children(ChildrenValue {
                            children_type: ChildrenType::Choice(ChoiceValue {
                                cps: c_or_s.cps,
                                space_before_close: c_or_s.ws_before_close,
                            }),
                            repetitions: Repetitions::parse(iter),
                        })),
                        ParsedChoiceOrSeqType::Seq => Ok(ContentSpec::Children(ChildrenValue {
                            children_type: ChildrenType::Seq(SeqValue {
                                cps: c_or_s.cps,
                                space_before_close: c_or_s.ws_before_close,
                            }),
                            repetitions: Repetitions::parse(iter),
                        })),
                    },
                    ParsedMystery::Mixed(m) => Ok(ContentSpec::Mixed(m)),
                }
            }
            _ => parse_err!(iter, "unable to parse ContentSpec"),
        }
    }
}

impl MixedValue {
    /// Expects iter pointing at `#PCDATA` and takes whatever whitespace proceeded it as an argument.
    fn parse(
        iter: &mut Iter<'_>,
        space_after_open_parenthesis: Option<Whitespace>,
    ) -> Result<Self> {
        iter.consume(STR_PCDATA)?;
        let mut space_before_delim = None;
        let mut element_names = Vec::new();
        loop {
            space_before_delim = Whitespace::parse_optional(iter);
            if iter.is(')') {
                break;
            }
            expect!(iter, '|')?;
            let space_after_delim = Whitespace::parse_optional(iter);
            element_names.push(DocTypeName::parse(iter)?);
        }
        iter.advance();
        Ok(Self {
            space_after_open_parenthesis,
            element_names: Vec::new(),
            space_before_close_parenthesis: space_before_delim,
        })
    }
}

impl Repetitions {
    fn parse(iter: &mut Iter<'_>) -> Self {
        let r = match iter.st.c {
            '?' => Repetitions::Optional,
            '+' => Repetitions::OneOrMore,
            '*' => Repetitions::ZeroOrMore,
            _ => return Repetitions::Once,
        };
        iter.advance();
        r
    }
}

impl ChildrenType {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        let choice_or_seq = match parse_mystery(iter)? {
            ParsedMystery::ChoiceOrSeq(val) => val,
            ParsedMystery::Mixed(_) => return parse_err!(iter, "#PCDATA cannot exist here"),
        };
        match choice_or_seq.t {
            ParsedChoiceOrSeqType::Choice => Ok(ChildrenType::Choice(ChoiceValue {
                cps: choice_or_seq.cps,
                space_before_close: choice_or_seq.ws_before_close,
            })),
            ParsedChoiceOrSeqType::Seq => Ok(ChildrenType::Seq(SeqValue {
                cps: choice_or_seq.cps,
                space_before_close: choice_or_seq.ws_before_close,
            })),
        }
    }
}

impl ChildrenValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        Ok(ChildrenValue {
            children_type: ChildrenType::parse(iter)?,
            repetitions: Repetitions::parse(iter),
        })
    }
}

impl CpItem {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        if iter.is('(') {
            let choice_or_seq = match parse_mystery(iter)? {
                ParsedMystery::ChoiceOrSeq(val) => val,
                ParsedMystery::Mixed(_) => return parse_err!(iter, "#PCDATA cannot exist here"),
            };
            match choice_or_seq.t {
                ParsedChoiceOrSeqType::Choice => Ok(CpItem::Choice(ChoiceValue {
                    cps: choice_or_seq.cps,
                    space_before_close: choice_or_seq.ws_before_close,
                })),
                ParsedChoiceOrSeqType::Seq => Ok(CpItem::Seq(SeqValue {
                    cps: choice_or_seq.cps,
                    space_before_close: choice_or_seq.ws_before_close,
                })),
            }
        } else {
            Ok(CpItem::Name(DocTypeName::parse(iter)?))
        }
    }
}

impl CpValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        Ok(CpValue {
            cp_item: CpItem::parse(iter)?,
            repetitions: Repetitions::parse(iter),
        })
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum ParsedChoiceOrSeqType {
    Choice,
    Seq,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct ParsedChoiceOrSeq {
    t: ParsedChoiceOrSeqType,
    cps: Vec<DelimitedListItem<CpValue>>,
    ws_before_close: Option<Whitespace>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
enum ParsedMystery {
    ChoiceOrSeq(ParsedChoiceOrSeq),
    Mixed(MixedValue),
}

/// Choice, Seq and Mixed cannot be determined until we have started parsing.
fn parse_mystery(iter: &mut Iter<'_>) -> Result<ParsedMystery> {
    debug_assert!(iter.is('('));
    iter.advance_or_die()?;
    let space_after_open = Whitespace::parse_optional(iter);
    if iter.is('#') {
        // must be '#PCDATA' which indicates Mixed
        Ok(ParsedMystery::Mixed(MixedValue::parse(
            iter,
            space_after_open,
        )?))
    } else {
        Ok(ParsedMystery::ChoiceOrSeq(parse_choice_or_seq(
            iter,
            space_after_open,
        )?))
    }
}

/// Takes the iter after `(` and first `Whitespace`.
fn parse_choice_or_seq(
    iter: &mut Iter<'_>,
    space_after_open: Option<Whitespace>,
) -> Result<ParsedChoiceOrSeq> {
    let first_cp_value = CpValue::parse(iter)?;
    let mut cps = Vec::new();
    cps.push(DelimitedListItem {
        space_before_delimiter: None,
        space_after_delimiter: space_after_open,
        item: first_cp_value,
    });
    let mut ws_before_delimiter = Whitespace::parse_optional(iter);
    let mut choice_or_seq = None;
    loop {
        if iter.is(')') {
            iter.advance();
            break;
        }
        if let Some(t) = choice_or_seq {
            match t {
                ParsedChoiceOrSeqType::Choice => expect!(iter, ',')?,
                ParsedChoiceOrSeqType::Seq => expect!(iter, '|')?,
            }
        } else {
            match iter.st.c {
                ',' => choice_or_seq = Some(ParsedChoiceOrSeqType::Seq),
                '|' => choice_or_seq = Some(ParsedChoiceOrSeqType::Choice),
                _ => return parse_err!(iter, "unexpected char when parsing choice or seq"),
            }
        }
        let ws_after_delimiter = Whitespace::parse_optional(iter);
        let cp = CpValue::parse(iter)?;
        cps.push(DelimitedListItem {
            space_before_delimiter: ws_before_delimiter,
            space_after_delimiter: ws_after_delimiter,
            item: cp,
        });
        ws_before_delimiter = Whitespace::parse_optional(iter);
    }
    let t = choice_or_seq
        .ok_or_else(|| create_parser_error!(&iter.st, "unable to determine choice or seq"))?;
    match t {
        ParsedChoiceOrSeqType::Choice => {
            if cps.len() < 2 {
                return parse_err!(iter, "choice must have at least two members");
            }
        }
        ParsedChoiceOrSeqType::Seq => {
            if cps.is_empty() {
                return parse_err!(iter, "seq must have at least one member");
            }
        }
    }
    Ok(ParsedChoiceOrSeq {
        t,
        cps,
        ws_before_close: ws_before_delimiter,
    })
}

impl AttlistDeclValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        debug_assert!(iter.is('!'));
        iter.advance_or_die()?;
        iter.consume(STR_ATTLIST)?;
        let space_before_name = Whitespace::parse(iter)?;
        let name = DocTypeName::parse(iter)?;
        let mut att_defs = Vec::new();
        let mut leftover_ws = None;
        loop {
            if iter.is('>') {
                break;
            }
            let lead_ws = Whitespace::parse(iter)?;
            if iter.is('>') {
                leftover_ws = Some(lead_ws);
                break;
            }
            att_defs.push(AttDef {
                space_before_name: lead_ws,
                name: DocTypeName::parse(iter)?,
                space_before_att_type: Whitespace::parse(iter)?,
                att_type: AttType::parse(iter)?,
                space_before_default_decl: Whitespace::parse(iter)?,
                default_decl: DefaultDecl::parse(iter)?,
            })
        }
        Ok(Self {
            space_before_name,
            name,
            att_defs,
            space_before_close: leftover_ws,
        })
    }
}

impl AttDef {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        Ok(Self {
            space_before_name: Whitespace::parse(iter)?,
            name: DocTypeName::parse(iter)?,
            space_before_att_type: Whitespace::parse(iter)?,
            att_type: AttType::parse(iter)?,
            space_before_default_decl: Whitespace::parse(iter)?,
            default_decl: DefaultDecl::parse(iter)?,
        })
    }
}

impl AttType {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        match iter.st.c {
            '(' => Ok(AttType::EnumeratedTypes(EnumeratedType::parse(iter)?)),
            'C' => {
                iter.consume(STR_CDATA)?;
                Ok(AttType::CData)
            }
            'E' => {
                iter.consume("ENTIT")?;
                if iter.is('Y') {
                    iter.advance_or_die()?;
                    Ok(AttType::Entity)
                } else {
                    iter.consume("IES")?;
                    Ok(AttType::Entities)
                }
            }
            'I' => {
                iter.consume("ID")?;
                if !iter.is('R') {
                    Ok(AttType::ID)
                } else {
                    iter.consume("REF")?;
                    if iter.is('S') {
                        Ok(AttType::IDRefs)
                    } else {
                        Ok(AttType::IDRef)
                    }
                }
            }
            'N' => {
                if iter.peek_or_die()? == 'O' {
                    Ok(AttType::EnumeratedTypes(EnumeratedType::parse(iter)?))
                } else {
                    iter.consume(STR_NMTOKEN)?;
                    if iter.is('S') {
                        Ok(AttType::NMTokens)
                    } else {
                        Ok(AttType::NMToken)
                    }
                }
            }
            _ => parse_err!(iter, "unable to parse AttType"),
        }
    }
}

impl EnumeratedType {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        if iter.is('N') {
            Ok(EnumeratedType::NotationType(NotationTypeValue::parse(
                iter,
            )?))
        } else if iter.is('(') {
            Ok(EnumeratedType::Enumeration(EnumerationValue::parse(iter)?))
        } else {
            parse_err!(iter, "expected N or )")
        }
    }
}

impl NotationTypeValue {
    /// NotationType ::= 'NOTATION' S '(' S? Name (S? '|' S? Name)* S? ')'
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        iter.consume(STR_NOTATION)?;
        let ws_before_paren = Whitespace::parse(iter)?;
        expect!(iter, '(')?;
        let ws_after_paren = Whitespace::parse_optional(iter);
        let first_name = DocTypeName::parse(iter)?;
        let mut names = Vec::new();
        names.push(DelimitedListItem {
            space_before_delimiter: None,
            space_after_delimiter: ws_after_paren,
            item: first_name,
        });
        let mut ws = None;
        loop {
            ws = Whitespace::parse_optional(iter);
            if iter.is(')') {
                break;
            } else if !iter.is('|') {
                return parse_err!(iter, "unexpected char in enumeration list");
            }
            iter.advance_or_die()?;
            let ws2 = Whitespace::parse_optional(iter);
            let name = DocTypeName::parse(iter)?;
            names.push(DelimitedListItem {
                space_before_delimiter: ws,
                space_after_delimiter: ws2,
                item: name,
            });
        }
        Ok(Self {
            space_before_open: ws_before_paren,
            names,
            space_before_close: ws,
        })
    }
}

impl EnumerationValue {
    /// Enumeration ::= '(' S? Nmtoken (S? '|' S? Nmtoken)* S? ')'
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        debug_assert!(iter.is('('));
        iter.advance_or_die()?;
        let mut nmtokens = Vec::new();
        nmtokens.push(DelimitedListItem {
            space_before_delimiter: None,
            space_after_delimiter: Whitespace::parse_optional(iter),
            item: NmToken::parse(iter, None)?,
        });
        let mut ws = None;
        loop {
            ws = Whitespace::parse_optional(iter);
            if iter.is(')') {
                break;
            } else if !iter.is('|') {
                return parse_err!(iter, "unexpected char in enumeration list");
            }
            iter.advance_or_die()?;
            let ws2 = Whitespace::parse_optional(iter);
            let nmtoken = NmToken::parse(iter, None)?;
            nmtokens.push(DelimitedListItem {
                space_before_delimiter: ws,
                space_after_delimiter: ws2,
                item: nmtoken,
            });
        }

        Ok(Self {
            names: nmtokens,
            space_before_close: ws,
        })
    }
}

impl NmToken {
    fn parse(iter: &mut Iter<'_>, end_marker: Option<char>) -> Result<Self> {
        let mut value = String::new();
        loop {
            if !iter.is_name_char() {
                return if let Some(end) = end_marker {
                    if iter.is(end) {
                        Ok(Self { value })
                    } else {
                        parse_err!(iter, "unexpected char before '{}' was reached", end)
                    }
                } else {
                    Ok(Self { value })
                };
            } else {
                value.push(iter.st.c);
                if end_marker.is_some() {
                    iter.advance_or_die()?
                } else {
                    if !iter.advance() {
                        return Ok(Self { value });
                    }
                }
            }
        }
    }
}

impl EntityDeclValue {
    /// Expects the iter pointing at the space following `<!ENTITY`.
    /// GEDecl ::= '<!ENTITY' S Name S EntityDef S? '>'
    /// PEDecl ::=  '<!ENTITY' S '%' S Name S PEDef S? '>'
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        let space_after_entity = Whitespace::parse(iter)?;
        if iter.is('%') {
            iter.advance_or_die()?;
            Ok(EntityDeclValue::PEDecl(PEDeclValue {
                space_before_percent: space_after_entity,
                space_before_name: Whitespace::parse(iter)?,
                name: DocTypeName::parse(iter)?,
                space_before_pe_def: Whitespace::parse(iter)?,
                pe_def: PEDef::parse(iter)?,
                space_before_close: Whitespace::parse_optional(iter),
            }))
        } else {
            Ok(EntityDeclValue::GEDecl(GEDeclValue {
                space_before_name: space_after_entity,
                name: DocTypeName::parse(iter)?,
                space_before_entity_def: Whitespace::parse(iter)?,
                entity_def: EntityDef::parse(iter)?,
                space_before_close: Whitespace::parse_optional(iter),
            }))
        }
    }
}

impl EntityDef {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        if iter.is('"') || iter.is('\'') {
            Ok(EntityDef::Entity(EntityValue::parse(iter)?))
        } else {
            Ok(EntityDef::External(EntityDefExternal::parse(iter)?))
        }
    }
}

impl EntityDefExternal {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        Ok(Self {
            external_id: ExternalID::parse(iter)?,
            ndata_decl: if iter.is_whitespace() {
                Some(NDataDecl::parse(iter)?)
            } else {
                None
            },
        })
    }
}

impl NDataDecl {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        let space_before_ndata = Whitespace::parse(iter)?;
        iter.consume(STR_NDATA)?;
        Ok(Self {
            space_before_ndata,
            space_before_name: Whitespace::parse(iter)?,
            name: DocTypeName::parse(iter)?,
        })
    }
}

impl PEDef {
    /// Iter should be pointing at SYSTEM, PUBLIC, ' or "
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        debug_assert!(matches!(iter.st.c, 'S' | 'P' | '\'' | '"'));
        match iter.st.c {
            'S' | 'P' => Ok(PEDef::External(ExternalID::parse(iter)?)),
            _ => Ok(PEDef::Entity(EntityValue::parse(iter)?)),
        }
    }
}

impl EntityValue {
    /// should be pointing at " or '
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        let mut data = Vec::new();
        let q = Quote::new(iter.st.c).map_err(|e| from_xe!(iter, e))?;
        iter.advance_or_die()?;
        loop {
            if iter.st.c == q.char() {
                break;
            }
            data.push(EntityValueData::parse(iter, q)?);
        }
        iter.advance();
        Ok(Self { quote: q, data })
    }
}

impl EntityValueData {
    /// Expects the iter pointing at `&` or `%` or any non-forbidden character.
    fn parse(iter: &mut Iter<'_>, q: Quote) -> Result<Self> {
        let c = iter.st.c;
        if c == '&' {
            return Ok(EntityValueData::Reference(ReferenceValue::parse(iter)?));
        } else if c == '%' {
            return Ok(EntityValueData::PEReference(PEReferenceValue::parse(iter)?));
        }
        // parse as text if it is not a reference
        let mut s = String::new();
        loop {
            if iter.st.c == q.char() {
                break;
            } else if iter.st.c == '&' {
                break;
            } else if iter.st.c == '%' {
                break;
            }
            s.push(c);
            iter.advance_or_die()?;
        }
        Ok(EntityValueData::String(s))
    }
}

impl Reference {
    /// Expects iter pointing to `&`.
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        match iter.peek_or_die()? {
            '#' => Ok(Reference::CharRef(CharRefValue::parse(iter)?)),
            _ => Ok(Reference::EntityRef(ReferenceValue::parse(iter)?)),
        }
    }
}

impl CharRefValue {
    /// Expects iter pointing to `&`.
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        debug_assert!(iter.is('&'));
        iter.advance_or_die()?;
        expect!(iter, '#')?;
        iter.advance_or_die()?;
        let next = iter.peek_or_die()?;
        let t = if next == 'x' {
            iter.advance_or_die()?;
            CharRefValueType::Hex
        } else if next.is_ascii_digit() {
            CharRefValueType::Decimal
        } else {
            return parse_err!(iter, "expected hex or decimal number");
        };
        let mut value = String::new();
        loop {
            if iter.is(';') {
                break;
            }
            value.push(iter.st.c);
            iter.advance_or_die()?;
        }
        iter.advance();
        let value = match t {
            CharRefValueType::Decimal => value
                .parse::<u64>()
                .map_err(|e| create_parser_error!(&iter.st, "{}", e))?,
            CharRefValueType::Hex => u64::from_str_radix(value.as_str(), 16)
                .map_err(|e| create_parser_error!(&iter.st, "{}", e))?,
        };
        Ok(Self {
            char_ref_type: t,
            value,
        })
    }
}

/// Expects iter pointing at `!NOTATION` (i.e. after the `<`).
impl NotationDeclValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        debug_assert!(iter.is('!'));
        iter.advance_or_die()?;
        iter.consume(STR_NOTATION)?;
        Ok(Self {
            space_before_name: Whitespace::parse(iter)?,
            name: DocTypeName::parse(iter)?,
            space_before_id: Whitespace::parse(iter)?,
            id: ExternalOrPublicID::parse(iter)?,
            space_before_close: Whitespace::parse_optional(iter),
        })
    }
}

impl ExternalOrPublicID {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        match iter.st.c {
            'S' => Ok(ExternalOrPublicID::External(ExternalID::parse(iter)?)),
            'P' => Ok(ExternalOrPublicID::Public(PublicID::parse(iter)?)),
            _ => parse_err!(iter, "expected SYSTEM or PUBLIC"),
        }
    }
}

impl PublicID {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        Ok(Self {
            space_before_name: Whitespace::parse(iter)?,
            public_id_literal: PubIDLiteral::parse(iter)?,
        })
    }
}

impl DefaultDecl {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        let next = iter.peek_or_die()?;
        match iter.st.c {
            '#' if next == 'R' => {
                iter.consume(STR_REQUIRED)?;
                Ok(DefaultDecl::Required)
            }
            '#' if next == 'I' => {
                iter.consume(STR_IMPLIED)?;
                Ok(DefaultDecl::Implied)
            }
            _ => Ok(DefaultDecl::Value(DefaultDeclAttValue::parse(iter)?)),
        }
    }
}

impl DefaultDeclAttValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        let fixed = if iter.is('#') {
            iter.consume(STR_FIXED)?;
            Some(Whitespace::parse(iter)?)
        } else {
            None
        };

        Ok(Self {
            fixed,
            att_value: AttValue::parse(iter)?,
        })
    }
}

impl AttValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        let q = Quote::new(iter.st.c).map_err(|e| create_parser_error!(&iter.st, "{}", e))?;
        iter.advance_or_die()?;
        Ok(Self {
            quote: q,
            data: AttValueData::parse(iter, q)?,
        })
    }
}

impl AttValueData {
    fn parse(iter: &mut Iter<'_>, q: Quote) -> Result<Self> {
        if iter.is('&') {
            Ok(AttValueData::Reference(ReferenceValue::parse(iter)?))
        } else {
            let mut s = String::new();
            loop {
                if Self::forbidden(iter.st.c, q) {
                    return parse_err!(iter, "forbidden character in attribute value");
                } else if iter.is(q.char()) {
                    iter.advance();
                    return Ok(AttValueData::Text(s));
                }
                iter.advance_or_die()?;
            }
        }
    }
}
