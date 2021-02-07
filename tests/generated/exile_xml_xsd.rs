// generated file, do not edit

use crate::test_utils::run_parse_test;
use exile::{Declaration, Document, Version};

const INPUT_FILE: &str = "exile_xml_xsd.xml";

#[test]
/// a file from https://www.w3.org/2001/03/xml.xsd
fn xml_xsd_parse() {
    run_parse_test(INPUT_FILE, &expected());
}

fn expected() -> Document {
    let mut doc = Document::new();
    doc.set_declaration(Declaration {
        version: Some(Version::V10),
        encoding: None,
    });
    // TODO - support doctype https://github.com/webern/exile/issues/22
    doc.set_doctype(
        r#"<!DOCTYPE xs:schema PUBLIC "-//W3C//DTD XMLSCHEMA 200102//EN" "XMLSchema.dtd" >"#,
    )
    .unwrap();
    let root = doc.root_mut();
    root.set_name(r#"xs:schema"#);
    root.add_attribute(
        r#"targetNamespace"#,
        r#"http://www.w3.org/XML/1998/namespace"#,
    );
    root.add_attribute(r#"xml:lang"#, r#"en"#);
    root.add_attribute(r#"xmlns:xs"#, r#"http://www.w3.org/2001/XMLSchema"#);
    let gen1n1 = root.add_new_child().unwrap();
    gen1n1.set_name(r#"xs:annotation"#);
    let gen2n1 = gen1n1.add_new_child().unwrap();
    gen2n1.set_name(r#"xs:documentation"#);
    gen2n1.add_text(r#"See http://www.w3.org/XML/1998/namespace.html and http://www.w3.org/TR/REC-xml for information about this namespace. This schema document describes the XML namespace, in a form suitable for import by other schema documents. Note that local names in this namespace are intended to be defined only by the World Wide Web Consortium or its subgroups. The following names are currently defined in this namespace and should not be used with conflicting semantics by any Working Group, specification, or document instance: base (as an attribute name): denotes an attribute whose value provides a URI to be used as the base for interpreting any relative URIs in the scope of the element on which it appears; its value is inherited. This name is reserved by virtue of its definition in the XML Base specification. lang (as an attribute name): denotes an attribute whose value is a language code for the natural language of the content of any element; its value is inherited. This name is reserved by virtue of its definition in the XML specification. space (as an attribute name): denotes an attribute whose value is a keyword indicating what whitespace processing discipline is intended for the content of the element; its value is inherited. This name is reserved by virtue of its definition in the XML specification. Father (in any context at all): denotes Jon Bosak, the chair of the original XML Working Group. This name is reserved by the following decision of the W3C XML Plenary and XML Coordination groups: In appreciation for his vision, leadership and dedication the W3C XML Plenary on this 10th day of February, 2000 reserves for Jon Bosak in perpetuity the XML name xml:Father"#);
    let gen1n3 = root.add_new_child().unwrap();
    gen1n3.set_name(r#"xs:annotation"#);
    let gen2n1 = gen1n3.add_new_child().unwrap();
    gen2n1.set_name(r#"xs:documentation"#);
    gen2n1.add_text(r#"This schema defines attributes and an attribute group suitable for use by schemas wishing to allow xml:base, xml:lang or xml:space attributes on elements they define. To enable this, such a schema must import this schema for the XML namespace, e.g. as follows: <schema . . .> . . . <import namespace="http://www.w3.org/XML/1998/namespace" schemaLocation="http://www.w3.org/2001/03/xml.xsd"/> Subsequently, qualified reference to any of the attributes or the group defined below will have the desired effect, e.g. <type . . .> . . . <attributeGroup ref="xml:specialAttrs"/> will define a type which will schema-validate an instance element with any of those attributes"#);
    let gen1n5 = root.add_new_child().unwrap();
    gen1n5.set_name(r#"xs:annotation"#);
    let gen2n1 = gen1n5.add_new_child().unwrap();
    gen2n1.set_name(r#"xs:documentation"#);
    gen2n1.add_text(r#"In keeping with the XML Schema WG's standard versioning policy, this schema document will persist at http://www.w3.org/2001/03/xml.xsd. At the date of issue it can also be found at http://www.w3.org/2001/xml.xsd. The schema document at that URI may however change in the future, in order to remain compatible with the latest version of XML Schema itself. In other words, if the XML Schema namespace changes, the version of this document at http://www.w3.org/2001/xml.xsd will change accordingly; the version at http://www.w3.org/2001/03/xml.xsd will not change."#);
    let gen1n7 = root.add_new_child().unwrap();
    gen1n7.set_name(r#"xs:attribute"#);
    gen1n7.add_attribute(r#"name"#, r#"lang"#);
    gen1n7.add_attribute(r#"type"#, r#"xs:language"#);
    let gen2n1 = gen1n7.add_new_child().unwrap();
    gen2n1.set_name(r#"xs:annotation"#);
    let gen3n1 = gen2n1.add_new_child().unwrap();
    gen3n1.set_name(r#"xs:documentation"#);
    gen3n1.add_text(r#"In due course, we should install the relevant ISO 2- and 3-letter codes as the enumerated possible values . . ."#);
    let gen1n9 = root.add_new_child().unwrap();
    gen1n9.set_name(r#"xs:attribute"#);
    gen1n9.add_attribute(r#"default"#, r#"preserve"#);
    gen1n9.add_attribute(r#"name"#, r#"space"#);
    let gen2n1 = gen1n9.add_new_child().unwrap();
    gen2n1.set_name(r#"xs:simpleType"#);
    let gen3n1 = gen2n1.add_new_child().unwrap();
    gen3n1.set_name(r#"xs:restriction"#);
    gen3n1.add_attribute(r#"base"#, r#"xs:NCName"#);
    let gen4n1 = gen3n1.add_new_child().unwrap();
    gen4n1.set_name(r#"xs:enumeration"#);
    gen4n1.add_attribute(r#"value"#, r#"default"#);
    let gen4n3 = gen3n1.add_new_child().unwrap();
    gen4n3.set_name(r#"xs:enumeration"#);
    gen4n3.add_attribute(r#"value"#, r#"preserve"#);
    let gen1n11 = root.add_new_child().unwrap();
    gen1n11.set_name(r#"xs:attribute"#);
    gen1n11.add_attribute(r#"name"#, r#"base"#);
    gen1n11.add_attribute(r#"type"#, r#"xs:anyURI"#);
    let gen2n1 = gen1n11.add_new_child().unwrap();
    gen2n1.set_name(r#"xs:annotation"#);
    let gen3n1 = gen2n1.add_new_child().unwrap();
    gen3n1.set_name(r#"xs:documentation"#);
    gen3n1.add_text(r#"See http://www.w3.org/TR/xmlbase/ for information about this attribute."#);
    let gen1n13 = root.add_new_child().unwrap();
    gen1n13.set_name(r#"xs:attributeGroup"#);
    gen1n13.add_attribute(r#"name"#, r#"specialAttrs"#);
    let gen2n1 = gen1n13.add_new_child().unwrap();
    gen2n1.set_name(r#"xs:attribute"#);
    gen2n1.add_attribute(r#"ref"#, r#"xml:base"#);
    let gen2n3 = gen1n13.add_new_child().unwrap();
    gen2n3.set_name(r#"xs:attribute"#);
    gen2n3.add_attribute(r#"ref"#, r#"xml:lang"#);
    let gen2n5 = gen1n13.add_new_child().unwrap();
    gen2n5.set_name(r#"xs:attribute"#);
    gen2n5.add_attribute(r#"ref"#, r#"xml:space"#);
    doc
}
