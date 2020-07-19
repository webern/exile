# xdoc

Current version: 0.0.2

![build](https://github.com/webern/exile/workflows/exile%20ci/badge.svg)

`xdoc` presents the primitives on an XML DOM.
For example `Element` and `Attribute` are concepts in this library.
It is written in support of the `exile` crate, but kept separate from that crate due to dev-time
compilation dependencies.
Specifically, the `xtest` crate uses `xdoc` to generate tests for `exile`.
The public concepts in `xdoc` are re-exported by `exile`.