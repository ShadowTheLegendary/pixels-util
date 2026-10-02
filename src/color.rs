use strum::EnumString;

pub type Color = [u8; 4];

pub const WHITE: Color = [0xFF; 4];
pub const TRANSPARENT: Color = [0x00; 4];

pub const GRAY8: Color = [63, 63, 63, 255];
pub const GRAY7: Color = [79, 79, 79, 255];
pub const GRAY6: Color = [95, 95, 95, 255];
pub const GRAY5: Color = [111, 111, 111, 255];
pub const GRAY4: Color = [127, 127, 127, 255];
pub const GRAY3: Color = [143, 143, 143, 255];
pub const GRAY2: Color = [159, 159, 159, 255];
pub const GRAY1: Color = [175, 175, 175, 255];
pub const GRAY0: Color = [191, 191, 191, 255];

pub const BLACK: Color = [0x00, 0x00, 0x00, 0xFF];

pub const RED: Color = [255, 0, 0, 255];
pub const ORANGE: Color = [255, 112, 0, 255];
pub const YELLOW: Color = [255, 242, 0, 255];
pub const GREEN: Color = [0, 255, 0, 255];
pub const BLUE: Color = [0, 0, 255, 255];
pub const VIOLET: Color = [255, 0, 238, 255];

const COLORS: [Color; 12] = [
    TRANSPARENT,
    WHITE,
    BLACK,
    GRAY1, // Light Gray
    GRAY4, // Gray
    GRAY7, // Dark Gray
    RED,
    ORANGE,
    YELLOW,
    GREEN,
    BLUE,
    VIOLET,
];

#[derive(Debug, PartialEq, EnumString)]
#[strum(serialize_all = "lowercase", ascii_case_insensitive)]
enum Colors {
    Transparent,
    White,
    Black,
    #[strum(serialize = "lightgray", serialize = "lightgrey")] // support wrong spelling
    LightGray,
    #[strum(serialize = "gray", serialize = "grey")] // support wrong spelling
    Gray,
    #[strum(serialize = "darkgray", serialize = "darkgrey")] // support wrong spelling
    DarkGray,
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    #[strum(serialize = "purple", serialize = "violet")]
    Violet,
}

pub fn color_from_string(name: String) -> Option<Color> {
    let name: String = name
        .chars()
        .filter(|c| !matches!(c, ' ' | '_' | '-'))
        .collect();

    if let Ok(color) = name.parse::<Colors>() {
        let idx = (color as u8) as usize;
        return Some(COLORS[idx]);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_fn() {
        assert_eq!(color_from_string("Transparent".to_string()), Some(TRANSPARENT));
        assert_eq!(color_from_string("White".to_string()), Some(WHITE));
        assert_eq!(color_from_string("Black".to_string()), Some(BLACK));
        assert_eq!(color_from_string("Light Gray".to_string()), Some(GRAY1));
        assert_eq!(color_from_string("Light_Gray".to_string()), Some(GRAY1));
        assert_eq!(color_from_string("Light Grey".to_string()), Some(GRAY1));
        assert_eq!(color_from_string("Light_Grey".to_string()), Some(GRAY1));
        assert_eq!(color_from_string("Gray".to_string()), Some(GRAY4));
        assert_eq!(color_from_string("Grey".to_string()), Some(GRAY4));
        assert_eq!(color_from_string("Dark Gray".to_string()), Some(GRAY7));
        assert_eq!(color_from_string("Dark_Gray".to_string()), Some(GRAY7));
        assert_eq!(color_from_string("Dark Grey".to_string()), Some(GRAY7));
        assert_eq!(color_from_string("Dark_Grey".to_string()), Some(GRAY7));
        assert_eq!(color_from_string("Red".to_string()), Some(RED));
        assert_eq!(color_from_string("Orange".to_string()), Some(ORANGE));
        assert_eq!(color_from_string("Yellow".to_string()), Some(YELLOW));
        assert_eq!(color_from_string("Green".to_string()), Some(GREEN));
        assert_eq!(color_from_string("Blue".to_string()), Some(BLUE));
        assert_eq!(color_from_string("Violet".to_string()), Some(VIOLET));
        assert_eq!(color_from_string("Purple".to_string()), Some(VIOLET));

        assert_eq!(color_from_string("REd".to_string()), Some(RED));
        assert_eq!(color_from_string("wHitE".to_string()), Some(WHITE));

        assert_eq!(color_from_string("Seven".to_string()), None);
        assert_eq!(color_from_string("!@#$%^&*()_".to_string()), None);
    }
}
