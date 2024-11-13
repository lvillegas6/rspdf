use crate::units::Pt;

pub struct PaperSize {
    width: Pt,
    height: Pt,
}

impl PaperSize {
    pub fn width(&self) -> Pt {
        self.width
    }

    pub fn height(&self) -> Pt {
        self.height
    }
}

const A4_SIZE: PaperSize = PaperSize { width: Pt(595.28), height: Pt(841.89) };
const A5_SIZE: PaperSize = PaperSize { width: Pt(419.53), height: Pt(595.28) };
const LETTER_SIZE: PaperSize = PaperSize { width: Pt(612.0), height: Pt(792.0) };
const GOVERNMENT_LETTER_SIZE: PaperSize = PaperSize { width: Pt(576.0), height: Pt(756.0) };
const LEGAL_SIZE: PaperSize = PaperSize { width: Pt(612.0), height: Pt(1008.0) };
const JUNIOR_LEGAL_SIZE: PaperSize = PaperSize { width: Pt(576.0), height: Pt(360.0) };
const LEDGER_SIZE: PaperSize = PaperSize { width: Pt(1224.0), height: Pt(792.0) };
const TABLOID_SIZE: PaperSize = PaperSize { width: Pt(792.0), height: Pt(1224.0) };

pub enum PaperFormat {
    A4,
    A5,
    Letter,
    GovernmentLetter,
    Legal,
    JuniorLegal,
    Ledger,
    Tabloid,
}

impl PaperFormat {
    pub fn get_format(&self) -> PaperSize {
        match self {
            PaperFormat::A4 => A4_SIZE,
            PaperFormat::A5 => A5_SIZE,
            PaperFormat::Letter => LETTER_SIZE,
            PaperFormat::GovernmentLetter => GOVERNMENT_LETTER_SIZE,
            PaperFormat::Legal => LEGAL_SIZE,
            PaperFormat::JuniorLegal => JUNIOR_LEGAL_SIZE,
            PaperFormat::Ledger => LEDGER_SIZE,
            PaperFormat::Tabloid => TABLOID_SIZE
        }
    }
}