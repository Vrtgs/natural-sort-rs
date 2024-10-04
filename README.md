[![Crates.io](https://img.shields.io/crates/v/natural-sort-rs.svg)](https://crates.io/crates/natural-sort-rs)
[![docs.rs](https://docs.rs/natural-sort-rs/badge.svg)](https://docs.rs/natural-sort-rs)

A `#![no_std]` implementation of [natural sort order](https://en.wikipedia.org/wiki/Natural_sort_order)

# Example

```rust
use natural_sort_rs::{Natural, NaturalSort};

fn main() {
    let mut files = ["file2.txt", "file11.txt", "file1.txt"];
    files.sort();
    assert_eq!(files, ["file1.txt", "file11.txt", "file2.txt"]);

    assert!(Natural::str("file0002.txt") > Natural::str("file1B.txt"));
    assert!(Natural::str("file0002.txt") < Natural::str("file11.txt"));

    let mut files = [
        "file1.txt",
        "file1B.txt",
        "file00.txt",
        "file11.txt",
        "file0002.txt",
    ];

    files.natural_sort::<str>();


    // Here, "file11.txt" comes last because `natural_sort` saw that there was a
    // number inside the string, and did a numerical, rather than lexical,
    // comparison.
    assert_eq!(
        files,
        [
            "file00.txt",
            "file1.txt",
            "file1B.txt",
            "file0002.txt",
            "file11.txt"
        ]
    );
}
```