use crate::parser::Iter;
use crate::xdoc::xdocv2::doctype::{
    AttDef, AttType, AttValue, AttValueData, AttlistDeclValue, CharRefValue, CharRefValueType,
    ChildrenType, ChildrenValue, ChoiceOrSeqContent, ContentSpec, CpType, CpValue, DeclSep,
    DefaultDecl, DefaultDeclAttValue, DelimitedListItem, DocTypeDecl, DocTypeDeclSpaceExternalID,
    DocTypeName, ElementDeclValue, EntityDeclValue, EntityDef, EntityDefExternal, EntityValue,
    EntityValueData, EnumeratedType, EnumerationValue, ExternalID, ExternalOrPublicID,
    FormattedItem, GEDeclValue, IntSubset, MarkupDeclValue, MixedValue, NDataDecl, NmToken,
    NotationDeclValue, NotationTypeValue, PEDeclValue, PEDef, PEReferenceValue, PubIDLiteral,
    PublicExternalID, PublicID, Quote, Reference, ReferenceValue, Repetitions, Space,
    SystemExternalID, SystemLiteral, Whitespace, CHAR_CARRIAGE_RETURN, CHAR_NEWLINE, CHAR_SPACE,
    CHAR_TAB, STR_ATTLIST, STR_CDATA, STR_ENTITY, STR_FIXED, STR_IMPLIED, STR_NDATA, STR_NMTOKEN,
    STR_NOTATION, STR_REQUIRED,
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
        unimplemented!();
    }
}

impl DeclSep {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl ReferenceValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl PEReferenceValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl MarkupDeclValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl ElementDeclValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl ContentSpec {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl<T> FormattedItem<T> {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl<T> DelimitedListItem<T> {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl MixedValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl Repetitions {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl ChildrenType {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl ChildrenValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl CpType {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl CpValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl ChoiceOrSeqContent {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

// impl ChoiceValue {
//     fn parse(iter: &mut Iter<'_>) -> Result<Self> {
//         unimplemented!();
//     }
// }
//
// impl SeqValue {
//     fn parse(iter: &mut Iter<'_>) -> Result<Self> {
//         unimplemented!();
//     }
// }

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

// impl TokenizedType {
//     fn parse(iter: &mut Iter<'_>) -> Result<Self> {
//         unimplemented!();
//     }
// }

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
                if let Some(end) = end_marker {
                    if iter.is(end) {
                        return Ok(Self { value });
                    } else {
                        return parse_err!(iter, "unexpected char before '{}' was reached", end);
                    }
                } else {
                    return Ok(Self { value });
                }
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
    /// Expects the iter pointing at `!`.
    /// GEDecl ::= '<!ENTITY' S Name S EntityDef S? '>'
    /// PEDecl ::=  '<!ENTITY' S '%' S Name S PEDef S? '>'
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        debug_assert!(iter.is('!'));
        iter.advance_or_die()?;
        iter.consume(STR_ENTITY)?;
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
        expect!(iter, '#');
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
        iter.consume(STR_NOTATION);
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
