use std::fmt::{Debug, Display, Formatter};
use crate::units::Pt;


#[derive(Debug)]
enum PageSizeError {
    InvalidWidth(f32),
    InvalidHeight(f32),
}

impl Display for PageSizeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PageSizeError::InvalidWidth(width) => write!(f, "Invalid page width: {}", width),
            PageSizeError::InvalidHeight(height) => write!(f, "Invalid page height: {}", height),
        }
    }
}

impl std::error::Error for PageSizeError {}

#[derive(Debug, Clone)]
pub struct PageSize {
    width: Pt,
    height: Pt,
}

impl PageSize {

    pub fn new(width: f32, height: f32) -> Result<Self, PageSizeError> {

        if width <= 0.0 {
            return Err(PageSizeError::InvalidWidth(width));
        }

        if height <= 0.0 {
            return Err(PageSizeError::InvalidHeight(height));
        }

        Ok(PageSize {
            width: Pt(width),
            height: Pt(height)
        })
    }

    pub const fn new_with_const(width: f32, height: f32) -> Self {
        PageSize {
            width: Pt(width),
            height: Pt(height)
        }
    }

    pub fn inverse(self) -> PageSize {
        PageSize {
            width: self.height,
            height: self.width
        }
    }

    pub fn width(&self) -> Pt {
        self.width
    }

    pub fn width_value(&self) -> f32 {
        self.width.0
    }

    pub fn height(&self) -> Pt {
        self.height
    }

    pub fn height_value(&self) -> f32 {
        self.height.0
    }
}

const A4_SIZE: PageSize = PageSize::new_with_const(595.28, 841.89);
const A5_SIZE: PageSize = PageSize::new_with_const(419.53, 595.28);
const LETTER_SIZE: PageSize = PageSize::new_with_const(612.0, 792.0);
const GOVERNMENT_LETTER_SIZE: PageSize = PageSize::new_with_const(612.0, 756.0);
const LEGAL_SIZE: PageSize = PageSize::new_with_const(612.0, 1008.0);
const JUNIOR_LEGAL_SIZE: PageSize = PageSize::new_with_const(576.0, 360.0);
const LEDGER_SIZE: PageSize = PageSize::new_with_const(1224.0, 792.0);
const TABLOID_SIZE: PageSize = PageSize::new_with_const(792.0, 1224.0);

#[derive(Debug)]
pub enum PageFormat {
    A4,
    A5,
    Letter,
    GovernmentLetter,
    Legal,
    JuniorLegal,
    Ledger,
    Tabloid,
}

impl PageFormat {
    pub fn get_format(&self) -> PageSize {
        match self {
            PageFormat::A4 => A4_SIZE,
            PageFormat::A5 => A5_SIZE,
            PageFormat::Letter => LETTER_SIZE,
            PageFormat::GovernmentLetter => GOVERNMENT_LETTER_SIZE,
            PageFormat::Legal => LEGAL_SIZE,
            PageFormat::JuniorLegal => JUNIOR_LEGAL_SIZE,
            PageFormat::Ledger => LEDGER_SIZE,
            PageFormat::Tabloid => TABLOID_SIZE
        }
    }
}