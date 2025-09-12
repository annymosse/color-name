mod constants;

use crate::utils::euclidean_distance;
pub use constants::{color, colors, Colors, COLORS_DATA};

pub struct Color;

impl Color {
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
        (COLORS_DATA)[_clr].1
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
    /// // NOTE: the string can be at any case, it will be converted into Title-case
    /// assert_eq!(Color::val().by_string("inDigo".to_string()).expect("Not found"), [75, 0, 130]);
    /// assert_eq!(Color::val().by_string("IndiGo ".to_string()).expect("Not found"), [75, 0, 130]);
    /// ```
    ///
    /// **NOTE:** Return `Result<[u8;3],u16>` as color data `[u8;3]` or Not found (`404`).
    pub fn by_string(&self, color: String) -> Result<[u8; 3], u16> {
        // if there ends up being a color whose name is the empty string, this will need changing
        if color.len() == 0 {
            Err(404)
        } else {
            let mut got = false;
            let mut col = [0, 0, 0];
            for c in COLORS_DATA.iter() {
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
    }

    /// Get value of a color by string OR enum.
    pub fn val() -> Color {
        Color
    }

    /// Get exact colour name if there's no data it return "404".
    pub fn name(rgb: [u8; 3]) -> String {
        for (name, value) in COLORS_DATA.iter() {
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

            for (name, value) in COLORS_DATA.iter() {
                // Compute comparative distance
                let distance = euclidean_distance(rgb, *value);

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
    fn test_color_by_enum_fn() {
        assert_eq!(Color::val().by_enum(color::yellow), [255, 255, 0]);
        assert_eq!(Color::val().by_enum(color::aqua), [0, 255, 255]);
        assert_eq!(Color::val().by_enum(color::red), [255, 0, 0]);
        assert_eq!(Color::val().by_enum(color::black), [0, 0, 0]);
    }

    #[test]
    fn test_color_by_string_fn() {
        assert_eq!(Color::val().by_string(String::from("")), Err(404));
        assert_eq!(Color::val().by_string(String::from("A")), Err(404));
        assert_eq!(
            Color::val().by_string(String::from("Azure")).unwrap(),
            [240, 255, 255]
        );
        assert_eq!(
            Color::val().by_string(String::from("Chocolate")).unwrap(),
            [210, 105, 30]
        );
        assert_eq!(
            Color::val().by_string(String::from("Red")).unwrap(),
            [255, 0, 0]
        );
        assert_eq!(
            Color::val().by_string(String::from("Black")).unwrap(),
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

    #[test]
    fn test_not_found_cases() {
        // Color::name returns type `String` thus the 404 in type of `String`
        assert_eq!(Color::name([0, 195, 0]), "404");

        // Color::val().by_string returns a type of `Result<[u8; {const}], u16>` thus the 404 in type of `Err(u16)`
        assert_eq!(
            Color::val().by_string(String::from("MayMoonsley")),
            Err(404)
        );
    }
}
