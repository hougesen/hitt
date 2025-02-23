# hitt-parser

<a href="https://crates.io/crates/hitt-parser"><img src="https://img.shields.io/crates/v/hitt-parser.svg"></a>

A HTTP parser with support for some extra features, built for the HTTP testing tool [hitt](https://github.com/hougesen/hitt).

## Supported features

- [x] HTTP Method
- [x] HTTP URI
- [x] HTTP Version
- [x] Single line comment (`#`)
- [ ] Multi line comment
- [x] Multiple requests in single file (`###` on a blank line between each request)
- [x] Variable declaration (`@variable_name=value` on a blank line)
- [x] Variable usage (`{{ variable_name }}`)
