use std::io::{BufReader, BufWriter, Write};
enum OrientationType {
    Portrait,
    Landscape
}

type PageFormat = (u64, u64);
const A4: PageFormat = (210, 297);
const A5: PageFormat = (148, 210);
const LETTER: PageFormat = (216, 279);
const LEGAL: PageFormat = (216, 356);
const PDF_VERSION: &str = "1.4";

struct rsPdf {
    orientation: OrientationType,
    format: PageFormat,
    output_buffer: Vec<u8>

}

impl rsPdf {

    fn new(orientation: OrientationType, format: PageFormat) -> rsPdf {
        rsPdf {
            orientation,
            format,
            output_buffer: Vec::new()
        }
    }

    fn format(&self) -> PageFormat {
        self.format
    }

    fn get_header(&mut self) {
        self.output_buffer.append(&mut b"%PDF-1.4\n".to_vec());
        self.output_buffer.append(&mut b"%\xC2\xB5\xC2\xB5\xC2\xB5\xC2\xB5\n".to_vec());
    }

    fn output_buffer(&self) -> &[u8] {
        self.output_buffer.as_ref()
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        let mut pdf = rsPdf::new(OrientationType::Portrait, A4);

        pdf.get_header();

        println!("PDF format: {:?}", pdf.format());
        println!("PDF version: {}", PDF_VERSION);
        println!("PDF buffer: {:?}", String::from_utf8_lossy(pdf.output_buffer()));
    }
}
