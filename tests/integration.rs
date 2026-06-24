//! Integration tests exercising the public API of `sort-keys`.

use serde_json::json;
use sort_keys::{sort_keys, sort_keys_by, sort_keys_with, Options};

#[test]
fn deterministic_config() {
    let config = json!({ "name": "app", "deps": { "zlib": "1", "acme": "2" }, "version": "1.0" });
    let sorted = sort_keys_with(&config, &Options::new().deep(true));
    // Re-serializing yields a stable, sorted form.
    assert_eq!(
        serde_json::to_string(&sorted).unwrap(),
        r#"{"deps":{"acme":"2","zlib":"1"},"name":"app","version":"1.0"}"#
    );
}

#[test]
fn shallow_leaves_nested_alone() {
    assert_eq!(
        sort_keys(&json!({ "b": 0, "a": { "y": 0, "x": 0 } })),
        json!({ "a": { "y": 0, "x": 0 }, "b": 0 })
    );
}

#[test]
fn reverse_and_ignore() {
    let reversed = sort_keys_by(&json!({ "a": 0, "c": 0, "b": 0 }), &Options::new(), |a, b| b.cmp(a));
    assert_eq!(reversed, json!({ "c": 0, "b": 0, "a": 0 }));

    let ignored = sort_keys_with(&json!({ "id": 0, "c": 0, "a": 0 }), &Options::new().ignore_keys(["id"]));
    assert_eq!(ignored, json!({ "id": 0, "a": 0, "c": 0 }));
}

#[test]
fn arrays_of_objects() {
    assert_eq!(
        sort_keys_with(&json!({ "list": [{ "b": 0, "a": 0 }] }), &Options::new().deep(true)),
        json!({ "list": [{ "a": 0, "b": 0 }] })
    );
}
