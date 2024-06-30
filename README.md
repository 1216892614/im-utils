# im_utils.rs

Extends [im](https://docs.rs/im/latest/im) and [im_rc](https://crates.io/crates/im-rc) containers with immutable methods.

## Usage

Get this crate by `cargo add im_utils`.

```rs
use im_utils::*; // just use them all.
use im::Vector;

let im_v: Vector<i32> = Vector::new().to_pushed_back(12);

assert_eq!(Vector::from_iter([12]), im_v);
```

## License

`im_utils` is dual licensed under the MIT license and the Apache License (Version 2.0).
