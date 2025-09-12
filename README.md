[![Build Status](https://travis-ci.org/annymosse/color-name.svg?branch=master)](https://travis-ci.org/annymosse/color-name)
[![API](https://docs.rs/color-name/badge.svg)](https://docs.rs/color-name)

A crate with color names and its values and usefull functions inluded to get similar colour name by RGB data. Based on these [named-colors](http://dev.w3.org/csswg/css-color/#named-colors); Inspired heavely by https://github.com/colorjs/color-name.

## Usage

```rust
extern crate color_name; // 👈 Before Rust 2018 Edition, Not necessary in Rust 2018 and later

use color_name::css::{
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

assert_eq!(Color::val().by_enum(color::white), [255, 255, 255]);
assert_eq!(Color::val().by_enum(color::indigo), [75, 0, 130]);

assert_eq!(Color::val().by_string("Indigo".to_string()), [75, 0, 130]);
// NOTE: the string can be at any case, it will be converted into Camel case
assert_eq!(Color::val().by_string("InDigo".to_string()), [75, 0, 130]);
assert_eq!(Color::val().by_string("inDigo".to_string()), [75, 0, 130]);
assert_eq!(Color::val().by_string("IndiGo".to_string()), [75, 0, 130]);
assert_eq!(Color::val().by_string("IndiGO".to_string()), [75, 0, 130]);


// for lower-case colors
use color_name::css::colors;
assert_eq!(colors::red,[255,0,0]);
assert_eq!(colors::blue,[0,0,255]);
assert_eq!(colors::yellow,[255,255,0]);
// for Snake-case colors
use color_name::css::colors;
assert_eq!(Colors::Red,[255,0,0]);
assert_eq!(Colors::Blue,[0,0,255]);
assert_eq!(Colors::Yellow,[255,255,0]);

// Enjoy it !!
```

#### Notes:

- Currently (as 2025/09) there is **148** colour name.
- All colours values are `[u8;3]` type (`[r, g, b]` in rang 0..255).
- Not found Cases:
  - The `Color::name` function returns a type of `String`; thus, the 404 (Not found code) will be in a type of `String` example: `assert_eq!(Color::name([0, 195, 0]), "404");`.
  - The `Color::val().by_string` function returns a type of `Result<[u8; {const}], u16>`; thus, the 404 (Not found code) will be in a type of `Err(u16)`, example:
    `assert_eq!(
Color::val().by_string(String::from("MayMoonsley")),
Err(404)
);`
- In the upcoming versions we will unify the 'Not found' cases and make both return `Result`; thus, it will return `Err(Type)` for unfound colors and be easy & performant to catch 404 cases as an Err.

### Migration

```rust
use color_name::{
  //XXXXXXXXXXXXXXXXXXXX DEPRECATION XXXXXXXXXXXXXXXXXXXX
  // The following comment block will be removed in the
  // upcommin versions of the crate; Thus,  It's hightly
  // recommanded to migrate your code to the new API
  // especially it's very easy and smooth migration.
  // main function to trait colours
  Color,
  // enum colour names use it with Color::value(color:color)
  // Ex: Color::value(color::red);
  color,
  // for lower case colour names
  colors,
  // for snake case colour names
  Colors,
  //XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXxXXXXXXXXXXXXX

  //==================== NEW API 1.2 ====================
  css::{
    // main function to trait colours
    Color,
    // enum colour names use it with Color::value(color:color)
    // Ex: Color::value(color::red);
    color,
    // for lower case colour names
    colors,
    // for snake case colour names
    Colors,
  },
};


// Enjoy the NO CHANGES on your code it will just works !!
```

## Todo

- [ ] Add script to auto generate the following files from csv file:
  - [ ] `css/constants.rs`.
- [ ] Improve the whole crate to be able to eliminate these above files.
- [ ] Improve the whole crate to be more dynamic and able to extend it with other crates.

## Licence

[MIT Or Apache 2 Licenses](./LICENSE.txt)
