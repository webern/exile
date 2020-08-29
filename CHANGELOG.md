# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

Currently we are using v0.0.x where every version can and will contain breaking changes.

## [Unreleased]
### Added
- Support for single-quoted attributes [#58]
- `exile::load` for loading files [#58]

[#58]: https://github.com/webern/exile/pull/58

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
[Unreleased]: https://github.com/webern/exile/compare/v0.0.1...HEAD
[v0.0.1]: https://github.com/webern/exile/compare/v0.0.0...v0.0.1
[v0.0.0]: https://github.com/webern/exile/releases/tag/v0.0.0
[30175b0]: https://github.com/webern/exile/compare/dd000e2..30175b0
[dd000e2]: https://github.com/webern/exile/tree/dd000e2
