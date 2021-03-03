#![allow(dead_code, non_camel_case_types, non_snake_case)]
pub mod Colors;
pub mod colors;
mod colors_array;

pub enum color {
    antiquewhite = 0,
    aliceblue = 1,
    aqua = 2,
    aquamarine = 3,
    azure = 4,
    beige = 5,
    bisque = 6,
    black = 7,
    blanchedalmond = 8,
    blue = 9,
    blueviolet = 10,
    brown = 11,
    burlywood = 12,
    cadetblue = 13,
    chartreuse = 14,
    chocolate = 15,
    coral = 16,
    cornflowerblue = 17,
    cornsilk = 18,
    crimson = 19,
    cyan = 20,
    darkblue = 21,
    darkcyan = 22,
    darkgoldenrod = 23,
    darkgray = 24,
    darkgreen = 25,
    darkgrey = 26,
    darkkhaki = 27,
    darkmagenta = 28,
    darkolivegreen = 29,
    darkorange = 30,
    darkorchid = 31,
    darkred = 32,
    darksalmon = 33,
    darkseagreen = 34,
    darkslateblue = 35,
    darkslategray = 36,
    darkslategrey = 37,
    darkturquoise = 38,
    darkviolet = 39,
    deeppink = 40,
    deepskyblue = 41,
    dimgray = 42,
    dimgrey = 43,
    dodgerblue = 44,
    firebrick = 45,
    floralwhite = 46,
    forestgreen = 47,
    fuchsia = 48,
    gainsboro = 49,
    ghostwhite = 50,
    gold = 51,
    goldenrod = 52,
    gray = 53,
    green = 54,
    greenyellow = 55,
    grey = 56,
    honeydew = 57,
    hotpink = 58,
    indianred = 59,
    indigo = 60,
    ivory = 61,
    khaki = 62,
    lavender = 63,
    lavenderblush = 64,
    lawngreen = 65,
    lemonchiffon = 66,
    lightblue = 67,
    lightcoral = 68,
    lightcyan = 69,
    lightgoldenrodyellow = 70,
    lightgray = 71,
    lightgreen = 72,
    lightgrey = 73,
    lightpink = 74,
    lightsalmon = 75,
    lightseagreen = 76,
    lightskyblue = 77,
    lightslategray = 78,
    lightslategrey = 79,
    lightsteelblue = 80,
    lightyellow = 81,
    lime = 82,
    limegreen = 83,
    linen = 84,
    magenta = 85,
    maroon = 86,
    mediumaquamarine = 87,
    mediumblue = 88,
    mediumorchid = 89,
    mediumpurple = 90,
    mediumseagreen = 91,
    mediumslateblue = 92,
    mediumspringgreen = 93,
    mediumturquoise = 94,
    mediumvioletred = 95,
    midnightblue = 96,
    mintcream = 97,
    mistyrose = 98,
    moccasin = 99,
    navajowhite = 100,
    navy = 101,
    oldlace = 102,
    olive = 103,
    olivedrab = 104,
    orange = 105,
    orangered = 106,
    orchid = 107,
    palegoldenrod = 108,
    palegreen = 109,
    paleturquoise = 110,
    palevioletred = 111,
    papayawhip = 112,
    peachpuff = 113,
    peru = 114,
    pink = 115,
    plum = 116,
    powderblue = 117,
    purple = 118,
    rebeccapurple = 119,
    red = 120,
    rosybrown = 121,
    royalblue = 122,
    saddlebrown = 123,
    salmon = 124,
    sandybrown = 125,
    seagreen = 126,
    seashell = 127,
    sienna = 128,
    silver = 129,
    skyblue = 130,
    slateblue = 131,
    slategray = 132,
    slategrey = 133,
    snow = 134,
    springgreen = 135,
    steelblue = 136,
    tan = 137,
    teal = 138,
    thistle = 139,
    tomato = 140,
    turquoise = 141,
    violet = 142,
    wheat = 143,
    white = 144,
    whitesmoke = 145,
    yellow = 146,
    yellowgreen = 147,
}

fn comparative_distance(x: [u8; 3], y: [u8; 3]) -> u128 {
    /*See https://en.m.wikipedia.org/wiki/Euclidean_distance#Squared_Euclidean_distance*/
    let r: u128 = ((x[0] as i16 - y[0] as i16) as isize).pow(2) as u128;
    let g: u128 = ((x[1] as i16 - y[1] as i16) as isize).pow(2) as u128;
    let b: u128 = ((x[2] as i16 - y[2] as i16) as isize).pow(2) as u128;

    r + g + b
}

pub struct Color;

impl Color {
    /// Get colour rgb array by enum Colors param.
    ///
    /// #### Example:
    ///
    /// ```rust
    /// use color_name::{
    ///     // main function to trait colours
    ///     Color,
    ///     // enum colour names use it with Color::value(color:color)
    ///     // Ex: Color::value(color::red);
    ///     color
    /// };
    /// assert_eq!(Color::value(color::white), [255, 255, 255]);
    /// ```
    #[deprecated(
        since = "1.1.0",
        note = "Please use val().by_enum() function instead, this function will no longer exist above version 1.1.0 ."
    )]
    pub fn value(color: color) -> [u8; 3] {
        Self.by_enum(color)
    }

    /// Get colour rgb array by enum color param.
    ///
    /// #### Example
    ///
    /// ```rust
    /// use color_name::{
    ///     Color,
    ///     color
    /// };
    /// assert_eq!(Color::val().by_enum(color::white), [255, 255, 255]);
    ///
    /// ```
    pub fn by_enum(&self, color: color) -> [u8; 3] {
        let _clr = color as usize;
        (colors_array::COLORS_DATA)[_clr].1
    }

    /// Get colour rgb array by String param.
    ///
    /// #### Example:
    ///
    /// ```rust
    /// use color_name::{
    ///     Color
    /// };
    /// assert_eq!(Color::val().by_string("InDigo".to_string()).expect("Not found"), [75, 0, 130]);
    ///
    /// // NOTE: the string can be at any case it will convert it into Title-case
    /// assert_eq!(Color::val().by_string("inDigo".to_string()).expect("Not found"), [75, 0, 130]);
    /// assert_eq!(Color::val().by_string("IndiGo ".to_string()).expect("Not found"), [75, 0, 130]);
    /// ```
    ///
    /// **NOTE:** Return `Result<[u8;3],u16>` as color data `[u8;3]` or Not found (`404`).
    pub fn by_string(&self, color: String) -> Result<[u8; 3], u16> {
        let mut got = false;
        let mut col = [0, 0, 0];
        for c in colors_array::COLORS_DATA.iter() {
            if String::from(c.0)
                == format!(
                    "{}{}",
                    &color.trim().to_uppercase()[0..1],
                    &color.trim().to_lowercase()[1..]
                )
            {
                col = (c.1).to_owned();
                got = true;
                break;
            }
        }
        if got {
            Ok(col)
        } else {
            Err(404)
        }
    }

    /// Get value of a color by string OR enum.
    pub fn val() -> Color {
        Color
    }

    /// Get exact colour name if there's no data it return "404".
    pub fn name(rgb: [u8; 3]) -> String {
        for (name, value) in colors_array::COLORS_DATA.iter() {
            if value == &rgb {
                return format!("{}", name);
            }
        }

        format!("404")
    }

    /// Get closest colour name match the provided rgb data array.
    pub fn similar(rgb: [u8; 3]) -> String {
        let c = Self::name(rgb);
        if c == "404" {
            let mut current_closest_distance = u128::MAX;
            let mut current_closest_keyword = "";

            for (name, value) in colors_array::COLORS_DATA.iter() {
                // Compute comparative distance
                let distance = comparative_distance(rgb, *value);

                if value == &rgb {
                    return format!("{}", name);
                }
                // Check if its less, if so set as closest
                #[allow(unused_parens)]
                if (distance < current_closest_distance) {
                    current_closest_distance = distance;
                    current_closest_keyword = name;
                }
            }
            format!("{}", current_closest_keyword)
        } else {
            c
        }
    }

    /// Sweet proxy to simlar(rgb)
    pub fn close_to(rgb: [u8; 3]) -> String {
        Self::similar(rgb)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lower_const_colors() {
        assert_eq!(colors::yellow, [255, 255, 0]);
        assert_eq!(colors::aqua, [0, 255, 255]);
    }

    #[test]
    fn test_upper_const_colors() {
        assert_eq!(Colors::Yellow, [255, 255, 0]);
        assert_eq!(Colors::Aqua, [0, 255, 255]);
    }

    #[test]
    fn test_color__by_enum_fn() {
        assert_eq!(Color::val().by_enum(color::yellow), [255, 255, 0]);
        assert_eq!(Color::val().by_enum(color::aqua), [0, 255, 255]);
        assert_eq!(Color::val().by_enum(color::red), [255, 0, 0]);
        assert_eq!(Color::val().by_enum(color::black), [0, 0, 0]);
    }

    #[test]
    fn test_color__by_string_fn() {
        assert_eq!(Color::val().by_string(String::from("")), Err(404));
        assert_eq!(Color::val().by_string(String::from("A")), Err(404));
        assert_eq!(
            Color::val()
                .by_string(String::from("Azure"))
                .expect("Not Found"),
            [240, 255, 255]
        );
        assert_eq!(
            Color::val()
                .by_string(String::from("Chocolate"))
                .expect("Not Found"),
            [210, 105, 30]
        );
        assert_eq!(
            Color::val()
                .by_string(String::from("Red"))
                .expect("Not Found"),
            [255, 0, 0]
        );
        assert_eq!(
            Color::val()
                .by_string(String::from("Black"))
                .expect("Not Found"),
            [0, 0, 0]
        );
        assert_eq!(Color::val().by_string(String::from("annymosse")), Err(404));
    }

    #[test]
    fn test_color_name_fn() {
        assert_eq!(Color::name([0, 0, 0]), "Black");
        assert_eq!(Color::name([0, 1, 1]), "404");
        assert_eq!(Color::name([0, 195, 0]), "404");
    }

    #[test]
    fn test_compare() {
        assert_eq!(comparative_distance([195, 40, 10], [0, 0, 0]), 39725);
        assert_eq!(comparative_distance([10, 40, 255], [195, 40, 10]), 94250);
    }

    #[test]
    fn test_color_similar_fn() {
        assert_eq!(Color::similar([195, 40, 10]), "Firebrick");
        assert_eq!(Color::similar([123, 45, 67]), "Brown");
        assert_eq!(Color::similar([213, 69, 87]), "Indianred");
        assert_eq!(Color::similar([87, 33, 26]), "Maroon");
        assert_eq!(Color::similar([0, 195, 0]), "Lime");
    }

    #[test]
    fn test_color_close_to() {
        assert_eq!(Color::close_to([195, 40, 10]), "Firebrick");
        assert_eq!(Color::close_to([123, 45, 67]), "Brown");
        assert_eq!(Color::close_to([213, 69, 87]), "Indianred");
        assert_eq!(Color::close_to([87, 33, 26]), "Maroon");
        assert_eq!(Color::close_to([0, 195, 0]), "Lime");
    }

    #[test]
    fn test_enum_colors() {
        assert_eq!(Color::val().by_enum(color::yellow), [255, 255, 0]);
        assert_eq!(Color::val().by_enum(color::aqua), [0, 255, 255]);
        assert_eq!(Color::val().by_enum(color::red), [255, 0, 0]);
        assert_eq!(Color::val().by_enum(color::black), [0, 0, 0]);
    }
}
