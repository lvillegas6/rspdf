use crate::types::{Point, RGB};

pub struct Text {
    pub text: String,
    pub font: String,
    pub size: i32,
    pub point: Point,
    pub color: RGB,
}

impl Text {
    pub fn new(text: &str, font: &str, size: i32, point: Point, color: RGB) -> Text {
        Text {
            text: text.to_string(),
            font: font.to_string(),
            point,
            size,
            color,
        }
    }
    pub fn new_with_default_color(text: &str, font: &str, size: i32, point: Point) -> Text {
        Text {
            text: text.to_string(),
            font: font.to_string(),
            point,
            size,
            color: RGB(0, 0, 0),
        }
    }
}

impl From<Text> for Vec<u8> {
    fn from(content: Text) -> Vec<u8> {
        let r = content.color.0 as f32 / 255.0;
        let g = content.color.1 as f32 / 255.0;
        let b = content.color.2 as f32 / 255.0;
        format!(
            "{:.3} {:.3} {:.3} rg\nBT\n/{} {} Tf\n1 0 0 1 {:.2} {:.2} Tm\n({}) Tj\nET\n",
            r, g, b,
            content.font, content.size,
            content.point.0, content.point.1,
            content.text
        ).into_bytes()
    }
}

