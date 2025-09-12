#![allow(non_upper_case_globals, non_snake_case, non_camel_case_types)]
pub mod colors {
    pub const antiquewhite: [u8; 3] = [250, 235, 215];
    pub const aliceblue: [u8; 3] = [240, 248, 255];
    pub const aqua: [u8; 3] = [0, 255, 255];
    pub const aquamarine: [u8; 3] = [127, 255, 212];
    pub const azure: [u8; 3] = [240, 255, 255];
    pub const beige: [u8; 3] = [245, 245, 220];
    pub const bisque: [u8; 3] = [255, 228, 196];
    pub const black: [u8; 3] = [0, 0, 0];
    pub const blanchedalmond: [u8; 3] = [255, 235, 205];
    pub const blue: [u8; 3] = [0, 0, 255];
    pub const blueviolet: [u8; 3] = [138, 43, 226];
    pub const brown: [u8; 3] = [165, 42, 42];
    pub const burlywood: [u8; 3] = [222, 184, 135];
    pub const cadetblue: [u8; 3] = [95, 158, 160];
    pub const chartreuse: [u8; 3] = [127, 255, 0];
    pub const chocolate: [u8; 3] = [210, 105, 30];
    pub const coral: [u8; 3] = [255, 127, 80];
    pub const cornflowerblue: [u8; 3] = [100, 149, 237];
    pub const cornsilk: [u8; 3] = [255, 248, 220];
    pub const crimson: [u8; 3] = [220, 20, 60];
    pub const cyan: [u8; 3] = [0, 255, 255];
    pub const darkblue: [u8; 3] = [0, 0, 139];
    pub const darkcyan: [u8; 3] = [0, 139, 139];
    pub const darkgoldenrod: [u8; 3] = [184, 134, 11];
    pub const darkgray: [u8; 3] = [169, 169, 169];
    pub const darkgreen: [u8; 3] = [0, 100, 0];
    pub const darkgrey: [u8; 3] = [169, 169, 169];
    pub const darkkhaki: [u8; 3] = [189, 183, 107];
    pub const darkmagenta: [u8; 3] = [139, 0, 139];
    pub const darkolivegreen: [u8; 3] = [85, 107, 47];
    pub const darkorange: [u8; 3] = [255, 140, 0];
    pub const darkorchid: [u8; 3] = [153, 50, 204];
    pub const darkred: [u8; 3] = [139, 0, 0];
    pub const darksalmon: [u8; 3] = [233, 150, 122];
    pub const darkseagreen: [u8; 3] = [143, 188, 143];
    pub const darkslateblue: [u8; 3] = [72, 61, 139];
    pub const darkslategray: [u8; 3] = [47, 79, 79];
    pub const darkslategrey: [u8; 3] = [47, 79, 79];
    pub const darkturquoise: [u8; 3] = [0, 206, 209];
    pub const darkviolet: [u8; 3] = [148, 0, 211];
    pub const deeppink: [u8; 3] = [255, 20, 147];
    pub const deepskyblue: [u8; 3] = [0, 191, 255];
    pub const dimgray: [u8; 3] = [105, 105, 105];
    pub const dimgrey: [u8; 3] = [105, 105, 105];
    pub const dodgerblue: [u8; 3] = [30, 144, 255];
    pub const firebrick: [u8; 3] = [178, 34, 34];
    pub const floralwhite: [u8; 3] = [255, 250, 240];
    pub const forestgreen: [u8; 3] = [34, 139, 34];
    pub const fuchsia: [u8; 3] = [255, 0, 255];
    pub const gainsboro: [u8; 3] = [220, 220, 220];
    pub const ghostwhite: [u8; 3] = [248, 248, 255];
    pub const gold: [u8; 3] = [255, 215, 0];
    pub const goldenrod: [u8; 3] = [218, 165, 32];
    pub const gray: [u8; 3] = [128, 128, 128];
    pub const green: [u8; 3] = [0, 128, 0];
    pub const greenyellow: [u8; 3] = [173, 255, 47];
    pub const grey: [u8; 3] = [128, 128, 128];
    pub const honeydew: [u8; 3] = [240, 255, 240];
    pub const hotpink: [u8; 3] = [255, 105, 180];
    pub const indianred: [u8; 3] = [205, 92, 92];
    pub const indigo: [u8; 3] = [75, 0, 130];
    pub const ivory: [u8; 3] = [255, 255, 240];
    pub const khaki: [u8; 3] = [240, 230, 140];
    pub const lavender: [u8; 3] = [230, 230, 250];
    pub const lavenderblush: [u8; 3] = [255, 240, 245];
    pub const lawngreen: [u8; 3] = [124, 252, 0];
    pub const lemonchiffon: [u8; 3] = [255, 250, 205];
    pub const lightblue: [u8; 3] = [173, 216, 230];
    pub const lightcoral: [u8; 3] = [240, 128, 128];
    pub const lightcyan: [u8; 3] = [224, 255, 255];
    pub const lightgoldenrodyellow: [u8; 3] = [250, 250, 210];
    pub const lightgray: [u8; 3] = [211, 211, 211];
    pub const lightgreen: [u8; 3] = [144, 238, 144];
    pub const lightgrey: [u8; 3] = [211, 211, 211];
    pub const lightpink: [u8; 3] = [255, 182, 193];
    pub const lightsalmon: [u8; 3] = [255, 160, 122];
    pub const lightseagreen: [u8; 3] = [32, 178, 170];
    pub const lightskyblue: [u8; 3] = [135, 206, 250];
    pub const lightslategray: [u8; 3] = [119, 136, 153];
    pub const lightslategrey: [u8; 3] = [119, 136, 153];
    pub const lightsteelblue: [u8; 3] = [176, 196, 222];
    pub const lightyellow: [u8; 3] = [255, 255, 224];
    pub const lime: [u8; 3] = [0, 255, 0];
    pub const limegreen: [u8; 3] = [50, 205, 50];
    pub const linen: [u8; 3] = [250, 240, 230];
    pub const magenta: [u8; 3] = [255, 0, 255];
    pub const maroon: [u8; 3] = [128, 0, 0];
    pub const mediumaquamarine: [u8; 3] = [102, 205, 170];
    pub const mediumblue: [u8; 3] = [0, 0, 205];
    pub const mediumorchid: [u8; 3] = [186, 85, 211];
    pub const mediumpurple: [u8; 3] = [147, 112, 219];
    pub const mediumseagreen: [u8; 3] = [60, 179, 113];
    pub const mediumslateblue: [u8; 3] = [123, 104, 238];
    pub const mediumspringgreen: [u8; 3] = [0, 250, 154];
    pub const mediumturquoise: [u8; 3] = [72, 209, 204];
    pub const mediumvioletred: [u8; 3] = [199, 21, 133];
    pub const midnightblue: [u8; 3] = [25, 25, 112];
    pub const mintcream: [u8; 3] = [245, 255, 250];
    pub const mistyrose: [u8; 3] = [255, 228, 225];
    pub const moccasin: [u8; 3] = [255, 228, 181];
    pub const navajowhite: [u8; 3] = [255, 222, 173];
    pub const navy: [u8; 3] = [0, 0, 128];
    pub const oldlace: [u8; 3] = [253, 245, 230];
    pub const olive: [u8; 3] = [128, 128, 0];
    pub const olivedrab: [u8; 3] = [107, 142, 35];
    pub const orange: [u8; 3] = [255, 165, 0];
    pub const orangered: [u8; 3] = [255, 69, 0];
    pub const orchid: [u8; 3] = [218, 112, 214];
    pub const palegoldenrod: [u8; 3] = [238, 232, 170];
    pub const palegreen: [u8; 3] = [152, 251, 152];
    pub const paleturquoise: [u8; 3] = [175, 238, 238];
    pub const palevioletred: [u8; 3] = [219, 112, 147];
    pub const papayawhip: [u8; 3] = [255, 239, 213];
    pub const peachpuff: [u8; 3] = [255, 218, 185];
    pub const peru: [u8; 3] = [205, 133, 63];
    pub const pink: [u8; 3] = [255, 192, 203];
    pub const plum: [u8; 3] = [221, 160, 221];
    pub const powderblue: [u8; 3] = [176, 224, 230];
    pub const purple: [u8; 3] = [128, 0, 128];
    pub const rebeccapurple: [u8; 3] = [102, 51, 153];
    pub const red: [u8; 3] = [255, 0, 0];
    pub const rosybrown: [u8; 3] = [188, 143, 143];
    pub const royalblue: [u8; 3] = [65, 105, 225];
    pub const saddlebrown: [u8; 3] = [139, 69, 19];
    pub const salmon: [u8; 3] = [250, 128, 114];
    pub const sandybrown: [u8; 3] = [244, 164, 96];
    pub const seagreen: [u8; 3] = [46, 139, 87];
    pub const seashell: [u8; 3] = [255, 245, 238];
    pub const sienna: [u8; 3] = [160, 82, 45];
    pub const silver: [u8; 3] = [192, 192, 192];
    pub const skyblue: [u8; 3] = [135, 206, 235];
    pub const slateblue: [u8; 3] = [106, 90, 205];
    pub const slategray: [u8; 3] = [112, 128, 144];
    pub const slategrey: [u8; 3] = [112, 128, 144];
    pub const snow: [u8; 3] = [255, 250, 250];
    pub const springgreen: [u8; 3] = [0, 255, 127];
    pub const steelblue: [u8; 3] = [70, 130, 180];
    pub const tan: [u8; 3] = [210, 180, 140];
    pub const teal: [u8; 3] = [0, 128, 128];
    pub const thistle: [u8; 3] = [216, 191, 216];
    pub const tomato: [u8; 3] = [255, 99, 71];
    pub const turquoise: [u8; 3] = [64, 224, 208];
    pub const violet: [u8; 3] = [238, 130, 238];
    pub const wheat: [u8; 3] = [245, 222, 179];
    pub const white: [u8; 3] = [255, 255, 255];
    pub const whitesmoke: [u8; 3] = [245, 245, 245];
    pub const yellow: [u8; 3] = [255, 255, 0];
}

pub mod Colors {
    pub const Antiquewhite: [u8; 3] = [250, 235, 215];
    pub const Aliceblue: [u8; 3] = [240, 248, 255];
    pub const Aqua: [u8; 3] = [0, 255, 255];
    pub const Aquamarine: [u8; 3] = [127, 255, 212];
    pub const Azure: [u8; 3] = [240, 255, 255];
    pub const Beige: [u8; 3] = [245, 245, 220];
    pub const Bisque: [u8; 3] = [255, 228, 196];
    pub const Black: [u8; 3] = [0, 0, 0];
    pub const Blanchedalmond: [u8; 3] = [255, 235, 205];
    pub const Blue: [u8; 3] = [0, 0, 255];
    pub const Blueviolet: [u8; 3] = [138, 43, 226];
    pub const Brown: [u8; 3] = [165, 42, 42];
    pub const Burlywood: [u8; 3] = [222, 184, 135];
    pub const Cadetblue: [u8; 3] = [95, 158, 160];
    pub const Chartreuse: [u8; 3] = [127, 255, 0];
    pub const Chocolate: [u8; 3] = [210, 105, 30];
    pub const Coral: [u8; 3] = [255, 127, 80];
    pub const Cornflowerblue: [u8; 3] = [100, 149, 237];
    pub const Cornsilk: [u8; 3] = [255, 248, 220];
    pub const Crimson: [u8; 3] = [220, 20, 60];
    pub const Cyan: [u8; 3] = [0, 255, 255];
    pub const Darkblue: [u8; 3] = [0, 0, 139];
    pub const Darkcyan: [u8; 3] = [0, 139, 139];
    pub const Darkgoldenrod: [u8; 3] = [184, 134, 11];
    pub const Darkgray: [u8; 3] = [169, 169, 169];
    pub const Darkgreen: [u8; 3] = [0, 100, 0];
    pub const Darkgrey: [u8; 3] = [169, 169, 169];
    pub const Darkkhaki: [u8; 3] = [189, 183, 107];
    pub const Darkmagenta: [u8; 3] = [139, 0, 139];
    pub const Darkolivegreen: [u8; 3] = [85, 107, 47];
    pub const Darkorange: [u8; 3] = [255, 140, 0];
    pub const Darkorchid: [u8; 3] = [153, 50, 204];
    pub const Darkred: [u8; 3] = [139, 0, 0];
    pub const Darksalmon: [u8; 3] = [233, 150, 122];
    pub const Darkseagreen: [u8; 3] = [143, 188, 143];
    pub const Darkslateblue: [u8; 3] = [72, 61, 139];
    pub const Darkslategray: [u8; 3] = [47, 79, 79];
    pub const Darkslategrey: [u8; 3] = [47, 79, 79];
    pub const Darkturquoise: [u8; 3] = [0, 206, 209];
    pub const Darkviolet: [u8; 3] = [148, 0, 211];
    pub const Deeppink: [u8; 3] = [255, 20, 147];
    pub const Deepskyblue: [u8; 3] = [0, 191, 255];
    pub const Dimgray: [u8; 3] = [105, 105, 105];
    pub const Dimgrey: [u8; 3] = [105, 105, 105];
    pub const Dodgerblue: [u8; 3] = [30, 144, 255];
    pub const Firebrick: [u8; 3] = [178, 34, 34];
    pub const Floralwhite: [u8; 3] = [255, 250, 240];
    pub const Forestgreen: [u8; 3] = [34, 139, 34];
    pub const Fuchsia: [u8; 3] = [255, 0, 255];
    pub const Gainsboro: [u8; 3] = [220, 220, 220];
    pub const Ghostwhite: [u8; 3] = [248, 248, 255];
    pub const Gold: [u8; 3] = [255, 215, 0];
    pub const Goldenrod: [u8; 3] = [218, 165, 32];
    pub const Gray: [u8; 3] = [128, 128, 128];
    pub const Green: [u8; 3] = [0, 128, 0];
    pub const Greenyellow: [u8; 3] = [173, 255, 47];
    pub const Grey: [u8; 3] = [128, 128, 128];
    pub const Honeydew: [u8; 3] = [240, 255, 240];
    pub const Hotpink: [u8; 3] = [255, 105, 180];
    pub const Indianred: [u8; 3] = [205, 92, 92];
    pub const Indigo: [u8; 3] = [75, 0, 130];
    pub const Ivory: [u8; 3] = [255, 255, 240];
    pub const Khaki: [u8; 3] = [240, 230, 140];
    pub const Lavender: [u8; 3] = [230, 230, 250];
    pub const Lavenderblush: [u8; 3] = [255, 240, 245];
    pub const Lawngreen: [u8; 3] = [124, 252, 0];
    pub const Lemonchiffon: [u8; 3] = [255, 250, 205];
    pub const Lightblue: [u8; 3] = [173, 216, 230];
    pub const Lightcoral: [u8; 3] = [240, 128, 128];
    pub const Lightcyan: [u8; 3] = [224, 255, 255];
    pub const Lightgoldenrodyellow: [u8; 3] = [250, 250, 210];
    pub const Lightgray: [u8; 3] = [211, 211, 211];
    pub const Lightgreen: [u8; 3] = [144, 238, 144];
    pub const Lightgrey: [u8; 3] = [211, 211, 211];
    pub const Lightpink: [u8; 3] = [255, 182, 193];
    pub const Lightsalmon: [u8; 3] = [255, 160, 122];
    pub const Lightseagreen: [u8; 3] = [32, 178, 170];
    pub const Lightskyblue: [u8; 3] = [135, 206, 250];
    pub const Lightslategray: [u8; 3] = [119, 136, 153];
    pub const Lightslategrey: [u8; 3] = [119, 136, 153];
    pub const Lightsteelblue: [u8; 3] = [176, 196, 222];
    pub const Lightyellow: [u8; 3] = [255, 255, 224];
    pub const Lime: [u8; 3] = [0, 255, 0];
    pub const Limegreen: [u8; 3] = [50, 205, 50];
    pub const Linen: [u8; 3] = [250, 240, 230];
    pub const Magenta: [u8; 3] = [255, 0, 255];
    pub const Maroon: [u8; 3] = [128, 0, 0];
    pub const Mediumaquamarine: [u8; 3] = [102, 205, 170];
    pub const Mediumblue: [u8; 3] = [0, 0, 205];
    pub const Mediumorchid: [u8; 3] = [186, 85, 211];
    pub const Mediumpurple: [u8; 3] = [147, 112, 219];
    pub const Mediumseagreen: [u8; 3] = [60, 179, 113];
    pub const Mediumslateblue: [u8; 3] = [123, 104, 238];
    pub const Mediumspringgreen: [u8; 3] = [0, 250, 154];
    pub const Mediumturquoise: [u8; 3] = [72, 209, 204];
    pub const Mediumvioletred: [u8; 3] = [199, 21, 133];
    pub const Midnightblue: [u8; 3] = [25, 25, 112];
    pub const Mintcream: [u8; 3] = [245, 255, 250];
    pub const Mistyrose: [u8; 3] = [255, 228, 225];
    pub const Moccasin: [u8; 3] = [255, 228, 181];
    pub const Navajowhite: [u8; 3] = [255, 222, 173];
    pub const Navy: [u8; 3] = [0, 0, 128];
    pub const Oldlace: [u8; 3] = [253, 245, 230];
    pub const Olive: [u8; 3] = [128, 128, 0];
    pub const Olivedrab: [u8; 3] = [107, 142, 35];
    pub const Orange: [u8; 3] = [255, 165, 0];
    pub const Orangered: [u8; 3] = [255, 69, 0];
    pub const Orchid: [u8; 3] = [218, 112, 214];
    pub const Palegoldenrod: [u8; 3] = [238, 232, 170];
    pub const Palegreen: [u8; 3] = [152, 251, 152];
    pub const Paleturquoise: [u8; 3] = [175, 238, 238];
    pub const Palevioletred: [u8; 3] = [219, 112, 147];
    pub const Papayawhip: [u8; 3] = [255, 239, 213];
    pub const Peachpuff: [u8; 3] = [255, 218, 185];
    pub const Peru: [u8; 3] = [205, 133, 63];
    pub const Pink: [u8; 3] = [255, 192, 203];
    pub const Plum: [u8; 3] = [221, 160, 221];
    pub const Powderblue: [u8; 3] = [176, 224, 230];
    pub const Purple: [u8; 3] = [128, 0, 128];
    pub const Rebeccapurple: [u8; 3] = [102, 51, 153];
    pub const Red: [u8; 3] = [255, 0, 0];
    pub const Rosybrown: [u8; 3] = [188, 143, 143];
    pub const Royalblue: [u8; 3] = [65, 105, 225];
    pub const Saddlebrown: [u8; 3] = [139, 69, 19];
    pub const Salmon: [u8; 3] = [250, 128, 114];
    pub const Sandybrown: [u8; 3] = [244, 164, 96];
    pub const Seagreen: [u8; 3] = [46, 139, 87];
    pub const Seashell: [u8; 3] = [255, 245, 238];
    pub const Sienna: [u8; 3] = [160, 82, 45];
    pub const Silver: [u8; 3] = [192, 192, 192];
    pub const Skyblue: [u8; 3] = [135, 206, 235];
    pub const Slateblue: [u8; 3] = [106, 90, 205];
    pub const Slategray: [u8; 3] = [112, 128, 144];
    pub const Slategrey: [u8; 3] = [112, 128, 144];
    pub const Snow: [u8; 3] = [255, 250, 250];
    pub const Springgreen: [u8; 3] = [0, 255, 127];
    pub const Steelblue: [u8; 3] = [70, 130, 180];
    pub const Tan: [u8; 3] = [210, 180, 140];
    pub const Teal: [u8; 3] = [0, 128, 128];
    pub const Thistle: [u8; 3] = [216, 191, 216];
    pub const Tomato: [u8; 3] = [255, 99, 71];
    pub const Turquoise: [u8; 3] = [64, 224, 208];
    pub const Violet: [u8; 3] = [238, 130, 238];
    pub const Wheat: [u8; 3] = [245, 222, 179];
    pub const White: [u8; 3] = [255, 255, 255];
    pub const Whitesmoke: [u8; 3] = [245, 245, 245];
    pub const Yellow: [u8; 3] = [255, 255, 0];
}

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

pub static COLORS_DATA: [(&str, [u8; 3]); 148] = [
    ("Antiquewhite", [250, 235, 215]),
    ("Aliceblue", [240, 248, 255]),
    ("Aqua", [0, 255, 255]),
    ("Aquamarine", [127, 255, 212]),
    ("Azure", [240, 255, 255]),
    ("Beige", [245, 245, 220]),
    ("Bisque", [255, 228, 196]),
    ("Black", [0, 0, 0]),
    ("Blanchedalmond", [255, 235, 205]),
    ("Blue", [0, 0, 255]),
    ("Blueviolet", [138, 43, 226]),
    ("Brown", [165, 42, 42]),
    ("Burlywood", [222, 184, 135]),
    ("Cadetblue", [95, 158, 160]),
    ("Chartreuse", [127, 255, 0]),
    ("Chocolate", [210, 105, 30]),
    ("Coral", [255, 127, 80]),
    ("Cornflowerblue", [100, 149, 237]),
    ("Cornsilk", [255, 248, 220]),
    ("Crimson", [220, 20, 60]),
    ("Cyan", [0, 255, 255]),
    ("Darkblue", [0, 0, 139]),
    ("Darkcyan", [0, 139, 139]),
    ("Darkgoldenrod", [184, 134, 11]),
    ("Darkgray", [169, 169, 169]),
    ("Darkgreen", [0, 100, 0]),
    ("Darkgrey", [169, 169, 169]),
    ("Darkkhaki", [189, 183, 107]),
    ("Darkmagenta", [139, 0, 139]),
    ("Darkolivegreen", [85, 107, 47]),
    ("Darkorange", [255, 140, 0]),
    ("Darkorchid", [153, 50, 204]),
    ("Darkred", [139, 0, 0]),
    ("Darksalmon", [233, 150, 122]),
    ("Darkseagreen", [143, 188, 143]),
    ("Darkslateblue", [72, 61, 139]),
    ("Darkslategray", [47, 79, 79]),
    ("Darkslategrey", [47, 79, 79]),
    ("Darkturquoise", [0, 206, 209]),
    ("Darkviolet", [148, 0, 211]),
    ("Deeppink", [255, 20, 147]),
    ("Deepskyblue", [0, 191, 255]),
    ("Dimgray", [105, 105, 105]),
    ("Dimgrey", [105, 105, 105]),
    ("Dodgerblue", [30, 144, 255]),
    ("Firebrick", [178, 34, 34]),
    ("Floralwhite", [255, 250, 240]),
    ("Forestgreen", [34, 139, 34]),
    ("Fuchsia", [255, 0, 255]),
    ("Gainsboro", [220, 220, 220]),
    ("Ghostwhite", [248, 248, 255]),
    ("Gold", [255, 215, 0]),
    ("Goldenrod", [218, 165, 32]),
    ("Gray", [128, 128, 128]),
    ("Green", [0, 128, 0]),
    ("Greenyellow", [173, 255, 47]),
    ("Grey", [128, 128, 128]),
    ("Honeydew", [240, 255, 240]),
    ("Hotpink", [255, 105, 180]),
    ("Indianred", [205, 92, 92]),
    ("Indigo", [75, 0, 130]),
    ("Ivory", [255, 255, 240]),
    ("Khaki", [240, 230, 140]),
    ("Lavender", [230, 230, 250]),
    ("Lavenderblush", [255, 240, 245]),
    ("Lawngreen", [124, 252, 0]),
    ("Lemonchiffon", [255, 250, 205]),
    ("Lightblue", [173, 216, 230]),
    ("Lightcoral", [240, 128, 128]),
    ("Lightcyan", [224, 255, 255]),
    ("Lightgoldenrodyellow", [250, 250, 210]),
    ("Lightgray", [211, 211, 211]),
    ("Lightgreen", [144, 238, 144]),
    ("Lightgrey", [211, 211, 211]),
    ("Lightpink", [255, 182, 193]),
    ("Lightsalmon", [255, 160, 122]),
    ("Lightseagreen", [32, 178, 170]),
    ("Lightskyblue", [135, 206, 250]),
    ("Lightslategray", [119, 136, 153]),
    ("Lightslategrey", [119, 136, 153]),
    ("Lightsteelblue", [176, 196, 222]),
    ("Lightyellow", [255, 255, 224]),
    ("Lime", [0, 255, 0]),
    ("Limegreen", [50, 205, 50]),
    ("Linen", [250, 240, 230]),
    ("Magenta", [255, 0, 255]),
    ("Maroon", [128, 0, 0]),
    ("Mediumaquamarine", [102, 205, 170]),
    ("Mediumblue", [0, 0, 205]),
    ("Mediumorchid", [186, 85, 211]),
    ("Mediumpurple", [147, 112, 219]),
    ("Mediumseagreen", [60, 179, 113]),
    ("Mediumslateblue", [123, 104, 238]),
    ("Mediumspringgreen", [0, 250, 154]),
    ("Mediumturquoise", [72, 209, 204]),
    ("Mediumvioletred", [199, 21, 133]),
    ("Midnightblue", [25, 25, 112]),
    ("Mintcream", [245, 255, 250]),
    ("Mistyrose", [255, 228, 225]),
    ("Moccasin", [255, 228, 181]),
    ("Navajowhite", [255, 222, 173]),
    ("Navy", [0, 0, 128]),
    ("Oldlace", [253, 245, 230]),
    ("Olive", [128, 128, 0]),
    ("Olivedrab", [107, 142, 35]),
    ("Orange", [255, 165, 0]),
    ("Orangered", [255, 69, 0]),
    ("Orchid", [218, 112, 214]),
    ("Palegoldenrod", [238, 232, 170]),
    ("Palegreen", [152, 251, 152]),
    ("Paleturquoise", [175, 238, 238]),
    ("Palevioletred", [219, 112, 147]),
    ("Papayawhip", [255, 239, 213]),
    ("Peachpuff", [255, 218, 185]),
    ("Peru", [205, 133, 63]),
    ("Pink", [255, 192, 203]),
    ("Plum", [221, 160, 221]),
    ("Powderblue", [176, 224, 230]),
    ("Purple", [128, 0, 128]),
    ("Rebeccapurple", [102, 51, 153]),
    ("Red", [255, 0, 0]),
    ("Rosybrown", [188, 143, 143]),
    ("Royalblue", [65, 105, 225]),
    ("Saddlebrown", [139, 69, 19]),
    ("Salmon", [250, 128, 114]),
    ("Sandybrown", [244, 164, 96]),
    ("Seagreen", [46, 139, 87]),
    ("Seashell", [255, 245, 238]),
    ("Sienna", [160, 82, 45]),
    ("Silver", [192, 192, 192]),
    ("Skyblue", [135, 206, 235]),
    ("Slateblue", [106, 90, 205]),
    ("Slategray", [112, 128, 144]),
    ("Slategrey", [112, 128, 144]),
    ("Snow", [255, 250, 250]),
    ("Springgreen", [0, 255, 127]),
    ("Steelblue", [70, 130, 180]),
    ("Tan", [210, 180, 140]),
    ("Teal", [0, 128, 128]),
    ("Thistle", [216, 191, 216]),
    ("Tomato", [255, 99, 71]),
    ("Turquoise", [64, 224, 208]),
    ("Violet", [238, 130, 238]),
    ("Wheat", [245, 222, 179]),
    ("White", [255, 255, 255]),
    ("Whitesmoke", [245, 245, 245]),
    ("Yellow", [255, 255, 0]),
    ("yellowgreen", [154, 205, 50]),
];
