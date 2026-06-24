# sort-keys

[![All Contributors](https://img.shields.io/badge/all_contributors-1-orange.svg?style=flat-square)](#contributors-)

[![crates.io](https://img.shields.io/crates/v/sort-keys.svg)](https://crates.io/crates/sort-keys)
[![docs.rs](https://docs.rs/sort-keys/badge.svg)](https://docs.rs/sort-keys)
[![CI](https://github.com/trananhtung/sort-keys/actions/workflows/ci.yml/badge.svg)](https://github.com/trananhtung/sort-keys/actions/workflows/ci.yml)
[![license](https://img.shields.io/crates/l/sort-keys.svg)](#license)

**Recursively sort the keys of a JSON object** — for deterministic, stable output.

A faithful Rust port of the widely-used
[`sort-keys`](https://www.npmjs.com/package/sort-keys) npm package, operating on
[`serde_json::Value`].

- **Zero dependencies** beyond `serde_json`
- `deep`, `ignore_keys`, and a custom comparator
- Differential-tested against the reference `sort-keys` implementation (60k cases)

## Install

```toml
[dependencies]
sort-keys = "0.1"
serde_json = "1"
```

## Usage

```rust
use serde_json::json;
use sort_keys::{sort_keys, sort_keys_with, sort_keys_by, Options};

assert_eq!(sort_keys(&json!({ "c": 0, "a": 0, "b": 0 })), json!({ "a": 0, "b": 0, "c": 0 }));

// Recurse into nested objects and arrays:
assert_eq!(
    sort_keys_with(&json!({ "b": { "d": 0, "c": 0 }, "a": 0 }), &Options::new().deep(true)),
    json!({ "a": 0, "b": { "c": 0, "d": 0 } })
);

// Custom comparator (reverse):
assert_eq!(
    sort_keys_by(&json!({ "a": 0, "b": 0 }), &Options::new(), |a, b| b.cmp(a)),
    json!({ "b": 0, "a": 0 })
);

// Keep some keys unsorted (they come first, in original order):
let _ = sort_keys_with(&json!({ "z": 0, "a": 0 }), &Options::new().ignore_keys(["z"]));
```

## Notes

- The default comparator orders keys by UTF-16 code units, matching JavaScript's default
  string sort. Use [`sort_keys_by`] for any other order.
- This crate sorts **all** keys, including numeric-looking ones, by the comparator. (The
  JavaScript engine additionally reorders canonical integer keys first when serializing — a
  runtime quirk that does not apply to Rust; enable `serde_json`'s `preserve_order` feature to
  keep this crate's sorted order on output.)

## Contributors ✨

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind are welcome — code, docs, bug reports, ideas, reviews! See the [emoji key](https://allcontributors.org/docs/en/emoji-key) for how each contribution is recognized, and open a PR or issue to get involved.

Thanks goes to these wonderful people:

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/trananhtung"><img src="https://avatars.githubusercontent.com/u/30992229?v=4?s=100" width="100px;" alt="Tung Tran"/><br /><sub><b>Tung Tran</b></sub></a><br /><a href="https://github.com/trananhtung/./commits?author=trananhtung" title="Code">💻</a> <a href="#maintenance-trananhtung" title="Maintenance">🚧</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
