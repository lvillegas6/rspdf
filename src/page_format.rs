use crate::units::Pt;

pub struct PageSize {
    width: Pt,
    height: Pt,
}

impl PageSize {

    pub fn new(width: f32, height: f32) -> PageSize {
        PageSize {
            width: Pt(width),
            height: Pt(height)
        }
    }

    pub fn width(&self) -> Pt {
        self.width
    }

    pub fn height(&self) -> Pt {
        self.height
    }
}

const A4_SIZE: PageSize = PageSize::new(595.28, 841.89);
const A5_SIZE: PageSize = PageSize::new(419.53, 595.28);
const LETTER_SIZE: PageSize = PageSize::new(612.0, 792.0);
const GOVERNMENT_LETTER_SIZE: PageSize = PageSize::new(612.0, 756.0);
const LEGAL_SIZE: PageSize = PageSize::new(612.0, 1008.0);
const JUNIOR_LEGAL_SIZE: PageSize = PageSize::new(576.0, 360.0);
const LEDGER_SIZE: PageSize = PageSize::new(1224.0, 792.0);
const TABLOID_SIZE: PageSize = PageSize::new(792.0, 1224.0);

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