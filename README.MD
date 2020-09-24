[![Build Status](annymosse/color-name.svg)](annymosse/color-name.svg?branch=master)
[![API](https://docs.rs/color-name/badge.svg)](https://docs.rs/color-name)

A crate with color names and its values and usefull functions inluded to get similar colour name by RGB data. Based on these [named-colors](http://dev.w3.org/csswg/css-color/#named-colors); Inspired heavely by https://github.com/colorjs/color-name.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
color-name = "1.0.0"
```

Add this to your `main.rs` or any other rust file:

```rust
extern crate color_name;
use color_name::{
    // main function to trait colours
    Color,
    // enum colour names use it with Color::value(color:color)
    // Ex: Color::value(color::red);
    color,
    // for lower case colour names
    colors,
    // for snake case colour names
    Colors,
};

assert_eq!(Color::name([0, 0, 0]), "Black");
// In case there's no color match that rgb param color it return "404"
assert_eq!(Color::name([0, 1, 1]), "404");
assert_eq!(Color::name([95, 4, 1]), "404");
// to get the closest similar colour use similar() or close_to()
//NOTE: close_to() is a proxy to similar()
assert_eq!(Color::similar([0, 1, 1]), "Black");
assert_eq!(Color::similar([95, 4, 1]), "Maroon");
assert_eq!(Color::close_to([95, 4, 1]), "Maroon");

// Deprecated use val().by_enum()
assert_eq!(Color::value(color::white), [255, 255, 255]);
assert_eq!(Color::val().by_enum(color::white), [255, 255, 255]);
assert_eq!(Color::value(color::indigo), [75, 0, 130]);
assert_eq!(Color::val().by_enum(color::indigo), [75, 0, 130]);

assert_eq!(Color::val().by_string("InDigo".to_string()), [75, 0, 130]);
// NOTE: the string can be at any case it will convert it into Title-case
assert_eq!(Color::val().by_string("inDigo".to_string()), [75, 0, 130]);
assert_eq!(Color::val().by_string("IndiGo".to_string()), [75, 0, 130]);


// for lower-case colors
use color_name::colors;
assert_eq!(colors::red,[255,0,0]);
assert_eq!(colors::blue,[0,0,255]);
assert_eq!(colors::yellow,[255,255,0]);
// for Snake-case colors
use color_name::colors;
assert_eq!(Colors::Red,[255,0,0]);
assert_eq!(Colors::Blue,[0,0,255]);
assert_eq!(Colors::Yellow,[255,255,0]);
...

// Enjoy it !!
```

#### Notes:

- Currently (as 2020/09) there is **148** colour name.
- All colours values are `[u8;3]` type (`[r, g, b]` in rang 0..255).

## Todo

- [ ] Add script to auto generate the following files from csv file:
  - [ ] `Colors.rs`.
  - [ ] `colors.rs`.
  - [ ] `colors.rs`.
  - [ ] `colors_array.rs`.
- [ ] Improve the whole crate to be able to eliminate these above files.
- [ ] Improve the whole crate to be more dynamic and able to extend it with other crates.

## Licence

[MIT License](./LICENSE.txt)
