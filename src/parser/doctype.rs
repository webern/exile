use crate::parser::Iter;
use crate::xdoc::xdocv2::doctype::{
    AttDef, AttType, AttValue, AttValueData, AttlistDeclValue, CharRefValue, CharRefValueType,
    ChildrenType, ChildrenValue, ChoiceOrSeqContent, ChoiceValue, ContentSpec, CpType, CpValue,
    DeclSep, DefaultDecl, DefaultDeclAttValue, DelimitedListItem, DocTypeDecl,
    DocTypeDeclSpaceExternalID, DocTypeName, ElementDeclValue, EntityDeclValue, EntityDef,
    EntityDefExternal, EntityValue, EntityValueData, EnumeratedType, EnumerationValue, ExternalID,
    ExternalOrPublicID, FormattedCp, FormattedItem, GEDeclValue, IntSubset, MarkupDeclValue,
    MixedValue, NDataDecl, NotationDeclValue, NotationTypeValue, PEDeclValue, PEDef,
    PEReferenceValue, PubIDLiteral, PublicExternalID, PublicID, Quote, Reference, ReferenceValue,
    Repetitions, SeqValue, Space, SystemExternalID, SystemLiteral, TokenizedType, Whitespace,
    CHAR_CARRIAGE_RETURN, CHAR_NEWLINE, CHAR_SPACE, CHAR_TAB, STR_FIXED, STR_IMPLIED, STR_NOTATION,
    STR_REQUIRED,
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

impl FormattedCp {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl ChoiceOrSeqContent {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl ChoiceValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl SeqValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl AttlistDeclValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl AttDef {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl AttType {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl TokenizedType {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl EnumeratedType {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl NotationTypeValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl EnumerationValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl EntityDeclValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl GEDeclValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl PEDeclValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl EntityDef {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl EntityDefExternal {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl NDataDecl {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl PEDef {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl EntityValue {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
    }
}

impl EntityValueData {
    fn parse(iter: &mut Iter<'_>) -> Result<Self> {
        unimplemented!();
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
