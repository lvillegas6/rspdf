use crate::types::{Point, RGB};

pub struct Line {
    pub from: Point,
    pub to: Point,
    pub color: RGB,
    pub width: f32,
}

impl Line {

    pub fn new(from: Point, to: Point, color: RGB, width: f32) -> Line {
        Line {
            from,
            to,
            color,
            width,
        }
    }

    pub fn new_with_default_color(from: Point, to: Point, width: f32) -> Line {
        Line {
            from,
            to,
            color: RGB(0, 0, 0),
            width,
        }
    }
}


impl From<Line> for Vec<u8> {
    fn from(line: Line) -> Vec<u8> {
        let r = line.color.0 as f32 / 255.0;
        let g = line.color.1 as f32 / 255.0;
        let b = line.color.2 as f32 / 255.0;

        format!(
            "{:.3} {:.3} {:.3} RG\n{:.3} w\n{:.2} {:.2} m\n{:.2} {:.2} l S\n",
            r, g, b,
            line.width,
            line.from.0, line.from.1,
            line.to.0, line.to.1
        ).into_bytes()
    }
}
