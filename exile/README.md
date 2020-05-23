# exile

Current version: 0.0.0


The `exile` library is in its infancy and may never amount to much (because XML is hard).

The goal is to read XML files into structured data, and write them back.
Ultimately I am interested in generating types from XSD, but it's a long way between here and there.

## TODO

This initial list gets me to a sort of 'pre-mvp' that can handle only the simplest of XML documents.

 * [x] xdoc: create the ezfile in a test using structs
 * [x] xdoc: write assertions for the ezfile structs
 * [x] xdoc: serialize the ezfile to xml
 * [x] xdoc: assert serialized xml equals a string constant of the xml
 * [x] xdoc: serialize the ezfile to json
 * [x] xtest: add the serialized ezfile data to the metadata file as an assertion.
 * [x] exile: generate an assertion of the ezfile using build.rs
 * [x] exile: make the parser work so that the ezfile test passes
 * [ ] exile: remove these dependencies    Compiling snafu-derive v0.6.3, Compiling snafu v0.6.3