# Turtle (Terse RDF Triple Language) parser

A turtle parser written in Rust

This is a learning project, infact learning Rust as I go!

The goal is to write a parser that can parse at least basic turtle syntax (no (nested) blank nodes, collections).

> Work In Progress

## Current capabilities
The parser is currently able to parse the following things:

 - [x] `@base` | `BASE`
 - [x] `@prefix` | `PREFIX`
 - [x] `<http://example/iri>`
 - [x] `prefix:iri`
 - [x] `a`
 - [x] `"literal"`
 - [x] `"literal@en"`
 - [x] `"literal"^^xsd:string`
 - [x] `"literal"^^<http://www.w3.org/2001/XMLSchema#string>`
 - [x] object lists `"literal@en", <http://example/iri>, ...`
 - [x] predicate lists
 - [x] integer `2`
 - [x] decimal `4.002602`
 - [x] double `1.663E-4`
 - [x] boolean
 - [ ] blank nodes
 - [x] multi line literals
 - [ ] collections
