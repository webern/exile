# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Currently we are using v0.0.x where every version can and will contain breaking changes.

## [v0.0.9] 2021-05-29
### Added
- An index type for accessing elements and their parents [#85]
[#85]: https://github.com/webern/exile/pull/85

## [v0.0.8] 2021-05-02
### Changed
- Provide mutable access to element children [#84]
- Use the correct error type for XDoc::write [#84]
- Rename `PI` to `Pi` for clippy's happiness [#84]

[#84]: https://github.com/webern/exile/pull/84

## [v0.0.7] 2021-02-06
### Changed
- Allow '>' in strings [#83]

[#83]: https://github.com/webern/exile/pull/83

## [v0.0.6] 2021-01-23
### Added
- Implement display for `Element` [#81]
- Add some types for XML Namespaces [#81]

### Changed
- Make the fields of `PI` (processing instruction) private [#81]

[#81]: https://github.com/webern/exile/pull/81

## [v0.0.5] 2020-12-12
## Added
- Add a way to iterate attributes [#80]
- Began work on doctypes, not done, not compiled in release [#78]

### Changed
- Allow the string `utf-8` (lowercase) [#80]
- Change some return types to `Option<&String>` [#80]

[#78]: https://github.com/webern/exile/pull/78
[#80]: https://github.com/webern/exile/pull/80

## [v0.0.4] 2020-11-29
## Added
- Support comments [#77]

[#77]: https://github.com/webern/exile/pull/77

## [v0.0.3] 2020-11-25
## Added
- Whitespace normalization of text nodes [#75]
- Support CDATA sections [#76]

### Changed
- Make `Element` struct members private [#74]
- Improve processing instructions [#75]

[#74]: https://github.com/webern/exile/pull/74
[#75]: https://github.com/webern/exile/pull/75
[#76]: https://github.com/webern/exile/pull/76

## [v0.0.2] - 2020-11-15
### Added
- Support for single-quoted attributes [#58]
- `exile::load` for loading files [#58]
- A lot of work on generating test cases with Java [#67], [#70], [#72]

### Changed
- The `xdoc` `Version` and `Encoding` enums were weird, changed to remove `None` [#59]
- Added some mutating functions to `Document`, `Element`, and maybe others
- Eliminated the `xdoc` and `xtest` crates [#67], [#70], [#72]

[#58]: https://github.com/webern/exile/pull/58
[#59]: https://github.com/webern/exile/pull/59
[#67]: https://github.com/webern/exile/pull/67
[#70]: https://github.com/webern/exile/pull/70
[#72]: https://github.com/webern/exile/pull/72

## [v0.0.1] - 2020-07-18
### Added
- Support for processing instructions [#56]
- Readme improvements [#54]

[#56]: https://github.com/webern/exile/pull/56
[#54]: https://github.com/webern/exile/pull/54

## [v0.0.0] - 2020-05-27
### Added
- Additional serialization tests, updated readme and changelog [#52]
- An iterator that only visits elements [#49]
- Ignore comments, processing instructions and doctype nodes when parsing [#48]
- Handle XML escape sequences [#46]
- Home made error macros [#39]
- Use BTreeMap for attributes [#37]
- Setup GitHub actions for CI testing [#34]
- Parser basics [30175b0]
- Basic XML Serialization [dd000e2]

[#34]: https://github.com/webern/exile/pull/34
[#37]: https://github.com/webern/exile/pull/37
[#39]: https://github.com/webern/exile/pull/39
[#46]: https://github.com/webern/exile/pull/46
[#48]: https://github.com/webern/exile/pull/48
[#49]: https://github.com/webern/exile/pull/49
[#52]: https://github.com/webern/exile/pull/52

<!-- version diff links -->
[Unreleased]: https://github.com/webern/exile/compare/v0.0.9...HEAD
[v0.0.9]: https://github.com/webern/exile/compare/v0.0.8...v0.0.9
[v0.0.8]: https://github.com/webern/exile/compare/v0.0.7...v0.0.8
[v0.0.7]: https://github.com/webern/exile/compare/v0.0.6...v0.0.7
[v0.0.6]: https://github.com/webern/exile/compare/v0.0.5...v0.0.6
[v0.0.5]: https://github.com/webern/exile/compare/v0.0.4...v0.0.5
[v0.0.4]: https://github.com/webern/exile/compare/v0.0.3...v0.0.4
[v0.0.3]: https://github.com/webern/exile/compare/v0.0.2...v0.0.3
[v0.0.2]: https://github.com/webern/exile/compare/v0.0.1...v0.0.2
[v0.0.1]: https://github.com/webern/exile/compare/v0.0.0...v0.0.1
[v0.0.0]: https://github.com/webern/exile/releases/tag/v0.0.0
[30175b0]: https://github.com/webern/exile/compare/dd000e2..30175b0
[dd000e2]: https://github.com/webern/exile/tree/dd000e2
