use crate::types::{Point, RGB};
use crate::Font;

pub struct Text<'a> {
    pub text: String,
    //use struct Font
    pub font: &'a Font,
    pub size: i32,
    pub point: Point,
    pub color: RGB,
}

impl Text<'_> {
    pub fn new<'a>(text: &str, font: &'a Font, size: i32, point: Point, color: RGB) -> Text<'a> {
        Text {
            text: text.to_string(),
            font,
            point,
            size,
            color,
        }
    }
    pub fn new_with_default_color<'a>(text: &str, font: &'a Font, size: i32, point: Point) -> Text<'a> {
        Text {
            text: text.to_string(),
            font,
            point,
            size,
            color: RGB(0, 0, 0),
        }
    }
}

impl From<Text<'_>> for Vec<u8> {
    fn from(content: Text) -> Vec<u8> {
        let r = content.color.0 as f32 / 255.0;
        let g = content.color.1 as f32 / 255.0;
        let b = content.color.2 as f32 / 255.0;

        format!(
            "{:.3} {:.3} {:.3} rg\nBT\n/{} {} Tf\n1 0 0 1 {:.2} {:.2} Tm\n({}) Tj\nET\n",
            r, g, b,
            content.font.name, content.size,
            content.point.0, content.point.1 - (content.font.height * content.size as f32),
            content.text
        ).into_bytes()
    }
}

