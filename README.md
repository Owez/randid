# Randid

Randid (pronounced like random but with `-id` instead of `-om`) is a minimalistic, web-safe library for generating customisable IDs for use in primarily web applications. The generated IDs are not guarenteed to be unique however!

Currently, this library has 2 main functions: `randid_str()` and `randid_i32()`. The former generates a random [BASE62](https://www.wikidata.org/wiki/Q809817) (web-safe) string of a specified length and the latter creates a padded random integer of the specified length (like `00012` for a length of 5).

## Examples

A random BASE62 string embedded as a url:

```rust
use randid::randid_str;

fn main() {
    let my_id = randid_str(5);

    println!("https://example.com/safeid/{}", my_id); // will provide a url-safe id like `bWk9D`, `yWvm3` or `POf3R`
}
```

Two padded random integers of 12 and 24 characters long respectively:

```rust
use randid::randid_i32;

fn main() {
    let padded_num_12 = randid_i32(12);
    let padded_num_24 = randid_i32(24);

    println!(
        "Guarenteed length of 12: {}, Guarenteed length of 24: {}",
        padded_num_12,
        padded_num_24
    );
}
```
