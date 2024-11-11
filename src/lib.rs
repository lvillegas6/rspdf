mod units;

enum OrientationType {
    Portrait,
    Landscape
}

type PageSize = (u32, u32);


enum PageFormat {
    A4,
    A5,
    Letter,
    Legal
}

impl PageFormat {
    fn get_format(&self) -> PageSize {
        match &self {
            PageFormat::A4 => (210, 297),
            PageFormat::A5 => (148, 210),
            PageFormat::Letter => (216, 279),
            PageFormat::Legal => (216, 356)
        }
    }
}

struct Page {
    page_size: PageSize,
}

impl Page {
    fn new(page_size: PageSize) -> Page {
        Page {
            page_size
        }
    }
    fn page_size(&self) -> PageSize {
        self.page_size
    }
}

struct RsPdf {
    orientation: OrientationType,
    pages: Vec<Page>,
    format: PageFormat,
    page_size: PageSize,
    output_buffer: Vec<u8>
}

impl RsPdf {

    fn new(orientation: OrientationType, format: PageFormat) -> RsPdf {

        let page_size = format.get_format();

        RsPdf {
            orientation,
            format,
            page_size,
            pages: Vec::new(),
            output_buffer: Vec::new()
        }
    }

    fn get_header(&mut self) {
        self.output_buffer.append(&mut b"%PDF-1.4\n".to_vec());
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

        let mut pdf = RsPdf::new(OrientationType::Portrait, PageFormat::A4);

        pdf.get_header();

        println!("PDF buffer: {:?}", String::from_utf8_lossy(pdf.output_buffer()));
    }
}
