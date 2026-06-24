//! # sort-keys — recursively sort the keys of a JSON object
//!
//! Produce a copy of a [`serde_json::Value`] with object keys sorted — useful for
//! deterministic, stable output. A faithful Rust port of the widely-used
//! [`sort-keys`](https://www.npmjs.com/package/sort-keys) npm package.
//!
//! ```
//! use serde_json::json;
//! use sort_keys::{sort_keys, sort_keys_with, Options};
//!
//! assert_eq!(sort_keys(&json!({ "c": 0, "a": 0, "b": 0 })), json!({ "a": 0, "b": 0, "c": 0 }));
//!
//! // Recurse into nested objects and arrays with `deep`:
//! assert_eq!(
//!     sort_keys_with(&json!({ "b": { "d": 0, "c": 0 }, "a": 0 }), &Options::new().deep(true)),
//!     json!({ "a": 0, "b": { "c": 0, "d": 0 } })
//! );
//! ```
//!
//! By default keys are compared by UTF-16 code units (matching JavaScript's default sort).
//! Use [`sort_keys_by`] for a custom comparator.

#![forbid(unsafe_code)]
#![doc(html_root_url = "https://docs.rs/sort-keys/0.1.0")]

use core::cmp::Ordering;
use serde_json::{Map, Value};

// Compile-test the README's examples as part of `cargo test`.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

/// Options controlling [`sort_keys_with`] / [`sort_keys_by`].
#[derive(Debug, Clone, Default)]
pub struct Options {
    deep: bool,
    ignore_keys: Vec<String>,
}

impl Options {
    /// Default options (shallow, no ignored keys).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Recurse into nested objects and arrays.
    #[must_use]
    pub fn deep(mut self, value: bool) -> Self {
        self.deep = value;
        self
    }

    /// Keys to leave unsorted: they keep their original order and are placed before the
    /// sorted keys.
    #[must_use]
    pub fn ignore_keys<I, S>(mut self, keys: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.ignore_keys = keys.into_iter().map(Into::into).collect();
        self
    }
}

/// Compare two strings by their UTF-16 code units (JavaScript's default string ordering).
#[must_use]
pub fn compare_utf16(a: &str, b: &str) -> Ordering {
    a.encode_utf16().cmp(b.encode_utf16())
}

/// Sort the keys of `value` (shallow) using the default UTF-16 ordering.
///
/// ```
/// # use serde_json::json;
/// # use sort_keys::sort_keys;
/// assert_eq!(sort_keys(&json!({ "b": 1, "a": 2 })), json!({ "a": 2, "b": 1 }));
/// ```
#[must_use]
pub fn sort_keys(value: &Value) -> Value {
    sort_keys_with(value, &Options::new())
}

/// Sort the keys of `value` with the given [`Options`], using the default UTF-16 ordering.
#[must_use]
pub fn sort_keys_with(value: &Value, options: &Options) -> Value {
    sort_keys_by(value, options, compare_utf16)
}

/// Sort the keys of `value` with the given [`Options`] and a custom comparator.
///
/// ```
/// # use serde_json::json;
/// # use sort_keys::{sort_keys_by, Options};
/// // Reverse order:
/// let sorted = sort_keys_by(&json!({ "a": 1, "b": 2 }), &Options::new(), |a, b| b.cmp(a));
/// assert_eq!(sorted, json!({ "b": 2, "a": 1 }));
/// ```
#[must_use]
pub fn sort_keys_by<F>(value: &Value, options: &Options, compare: F) -> Value
where
    F: Fn(&str, &str) -> Ordering,
{
    sort_value(value, options, &compare)
}

fn sort_value<F>(value: &Value, options: &Options, compare: &F) -> Value
where
    F: Fn(&str, &str) -> Ordering,
{
    match value {
        Value::Object(map) => Value::Object(sort_object(map, options, compare)),
        Value::Array(array) => Value::Array(sort_array(array, options, compare)),
        other => other.clone(),
    }
}

fn sort_object<F>(map: &Map<String, Value>, options: &Options, compare: &F) -> Map<String, Value>
where
    F: Fn(&str, &str) -> Ordering,
{
    let mut ignored: Vec<&String> = Vec::new();
    let mut to_sort: Vec<&String> = Vec::new();
    for key in map.keys() {
        if options
            .ignore_keys
            .iter()
            .any(|ignored_key| ignored_key == key)
        {
            ignored.push(key);
        } else {
            to_sort.push(key);
        }
    }
    to_sort.sort_by(|a, b| compare(a, b));

    let mut result = Map::new();
    for key in ignored.into_iter().chain(to_sort) {
        let value = &map[key];
        let new_value = if options.deep {
            recurse_value(value, options, compare)
        } else {
            value.clone()
        };
        result.insert(key.clone(), new_value);
    }
    result
}

fn sort_array<F>(array: &[Value], options: &Options, compare: &F) -> Vec<Value>
where
    F: Fn(&str, &str) -> Ordering,
{
    array
        .iter()
        .map(|item| {
            if options.deep {
                recurse_value(item, options, compare)
            } else {
                item.clone()
            }
        })
        .collect()
}

/// Recurse into a value if it is a container (objects are key-sorted, arrays have their items
/// processed); other values are cloned.
fn recurse_value<F>(value: &Value, options: &Options, compare: &F) -> Value
where
    F: Fn(&str, &str) -> Ordering,
{
    match value {
        Value::Object(map) => Value::Object(sort_object(map, options, compare)),
        Value::Array(array) => Value::Array(sort_array(array, options, compare)),
        other => other.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn shallow() {
        assert_eq!(
            sort_keys(&json!({ "c": 0, "a": 0, "b": 0 })),
            json!({ "a": 0, "b": 0, "c": 0 })
        );
        // Default shallow: nested keys not sorted.
        assert_eq!(
            sort_keys(&json!({ "b": { "d": 0, "c": 0 }, "a": 0 })),
            json!({ "a": 0, "b": { "d": 0, "c": 0 } })
        );
    }

    #[test]
    fn deep() {
        assert_eq!(
            sort_keys_with(
                &json!({ "b": { "d": 0, "c": 0 }, "a": 0 }),
                &Options::new().deep(true)
            ),
            json!({ "a": 0, "b": { "c": 0, "d": 0 } })
        );
        assert_eq!(
            sort_keys_with(
                &json!({ "b": [{ "d": 0, "c": 0 }], "a": 0 }),
                &Options::new().deep(true)
            ),
            json!({ "a": 0, "b": [{ "c": 0, "d": 0 }] })
        );
        // Top-level array: object items sorted only when deep.
        assert_eq!(
            sort_keys_with(&json!([{ "c": 0, "a": 0 }]), &Options::new().deep(true)),
            json!([{ "a": 0, "c": 0 }])
        );
        assert_eq!(
            sort_keys(&json!([{ "c": 0, "a": 0 }])),
            json!([{ "c": 0, "a": 0 }])
        );
    }

    #[test]
    fn custom_compare() {
        let reversed = sort_keys_by(
            &json!({ "a": 0, "c": 0, "b": 0 }),
            &Options::new(),
            |a, b| b.cmp(a),
        );
        assert_eq!(reversed, json!({ "c": 0, "b": 0, "a": 0 }));
    }

    #[test]
    fn ignore_keys() {
        // Ignored keys keep their original order and come first.
        let result = sort_keys_with(
            &json!({ "z": 0, "b": 0, "a": 0 }),
            &Options::new().ignore_keys(["z"]),
        );
        assert_eq!(result, json!({ "z": 0, "a": 0, "b": 0 }));
    }

    #[test]
    fn scalars_passthrough() {
        assert_eq!(sort_keys(&json!(42)), json!(42));
        assert_eq!(sort_keys(&json!("s")), json!("s"));
    }
}
