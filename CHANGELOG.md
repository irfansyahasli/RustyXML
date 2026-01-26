# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-01-26

### Added

- **Purpose-built Rust NIF** for ultra-fast XML parsing in Elixir
- **SIMD-accelerated parsing** via `memchr` for fast delimiter detection
- **Arena-based DOM** with `NodeId` indices for cache-friendly traversal
- **Full XPath 1.0 support** with all 13 axes and 27+ functions
- **SweetXml-compatible API** with `~x` sigil and modifiers (`s`, `l`, `e`, `o`, `i`, `f`, `k`)

### Parsing Strategies

- **DOM Parser** (`parse/1` + `xpath/2`) - Parse once, query multiple times
- **Direct Query** (`xpath/2` on raw XML) - Single query, temporary DOM
- **Lazy XPath** (`Native.xpath_lazy/2`) - Keep results in Rust memory, 3x faster for large result sets
- **Streaming** (`stream_tags/3`) - Bounded memory for large files
- **Parallel XPath** (`xmap_parallel/2`) - Multi-threaded query evaluation

### XPath Features

- **All 13 axes**: child, descendant, descendant-or-self, parent, ancestor, ancestor-or-self, following, following-sibling, preceding, preceding-sibling, self, attribute, namespace
- **27+ functions**: position(), last(), count(), string(), concat(), contains(), substring(), normalize-space(), boolean(), not(), number(), sum(), floor(), ceiling(), round(), and more
- **LRU cache** for compiled XPath expressions (256 entries)
- **Fast-path predicates** for common patterns (`[@attr='value']`, `[n]`)

### Security

- **XXE immune** - External entities parsed but ignored
- **Billion Laughs immune** - Only 5 built-in entities expanded
- **DTD processing disabled** - No external DTD fetching
- **Panic-safe NIF** - No `.unwrap()` in NIF code paths
- **Atom table safe** - User-provided values use binary keys

### Conformance

- **100% OASIS/W3C XML Conformance** - 1089/1089 applicable tests pass
- **218/218** valid document tests
- **871/871** not-well-formed rejection tests

### Performance

- **8.9x faster parsing** than SweetXml
- **3x faster** lazy XPath for count-only queries
- **5x faster** combined parse + query workflows
