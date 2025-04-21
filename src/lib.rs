

mod units;
mod page_format;
mod page;
mod meta_data;
mod text;
mod reference;
mod line;
mod types;

use std::io::Read;
use std::ops::Add;
use page::{Page};
use crate::meta_data::MetaData;
use crate::reference::Ref;

#[derive(Debug)]
struct RsPdf {
    meta_data: MetaData,
    pages: Vec<Page>,
    current_id: u32,
}

impl RsPdf {

    pub fn new(title: &str) -> RsPdf {
        RsPdf {
            meta_data: MetaData {
                title: title.to_string(),
                ..MetaData::default()
            },
            pages: Vec::new(),
            current_id: 1,
        }
    }

    pub fn new_with_meta_data(meta_data: MetaData) -> RsPdf {
        RsPdf {
            meta_data,
            pages: Vec::new(),
            current_id: 1,
        }
    }

    fn alloc_id(&mut self) -> Ref {
        let id = self.current_id;
        self.current_id += 1;
        Ref::new(id)
    }
    pub fn add_page(&mut self, page: Page) {
        self.pages.push(page);
    }

    pub fn build(&mut self) -> Vec<u8> {
        let face_obj_id: String = self.alloc_id().into();
        let mut document: Vec<u8> = Vec::new();
        let mut xref_offset: Vec<u32> = Vec::new();

        document.extend_from_slice(format!("%PDF-{}\n", self.meta_data.version).as_bytes());
        document.extend_from_slice(format!("{} obj\n<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>\nendobj\n", face_obj_id).as_bytes());
        xref_offset.push(document.len() as u32);
        let font_obj_id: String = self.alloc_id().into();
        document.extend_from_slice(format!("{} obj\n<< /Font << /F1 {} R >> >>\nendobj\n", font_obj_id, face_obj_id).as_bytes());
        xref_offset.push(document.len() as u32);

        let catalog_obj_id: String = self.alloc_id().into();
        let pages_obj_id: String = self.alloc_id().into();
        document.extend_from_slice(format!("{} obj\n<< /Type /Catalog /Pages {} R >>\nendobj\n", catalog_obj_id, pages_obj_id).as_bytes());
        xref_offset.push(document.len() as u32);
        let mut kids: Vec<String> = vec![];
        let page_length = self.pages.len();
        let pages = self.pages.clone();

        for page in pages {
            let page_obj_id: String = self.alloc_id().into();
            kids.push(page_obj_id.clone().add(" R"));
            let page_size = page.size();
            let content_obj_id: String = self.alloc_id().into();
            document.extend_from_slice(format!("{} obj\n<< /Type /Page /Parent {} R /MediaBox [0 0 {} {}] /Contents {} R >>\nendobj\n", page_obj_id, pages_obj_id, page_size.width_value(), page_size.height_value(), content_obj_id).as_bytes());
            xref_offset.push(document.len() as u32);

            document.extend_from_slice(format!("{} obj\n<< /Length {} >>\nstream\n", content_obj_id, page.length()).as_bytes());
            document.extend_from_slice(page.content());
            document.extend_from_slice(b"\nendstream\nendobj\n");
            xref_offset.push(document.len() as u32);
        }

        document.extend_from_slice(format!("{} obj\n<< /Type /Pages /Kids [{}] /Count {} >>\nendobj\n", pages_obj_id, kids.join(" "), page_length).as_bytes());
        xref_offset.push(document.len() as u32);

        let xref_obj_id = self.alloc_id();
        document.extend_from_slice(format!("xref 0 {}\n", xref_obj_id.id).as_bytes());
        document.extend_from_slice(b"0000000000 65535 f\n");
        for offset in xref_offset {
            document.extend_from_slice(format!("{:010} 00000 n\n", offset).as_bytes());
        }
        document.extend_from_slice(format!("trailer\n<< /Root {} R /Size {} >>\nstartxref\n{}\n", catalog_obj_id, self.current_id, document.len()).as_bytes());
        document.extend_from_slice(b"%%EOF");

        document
    }
}

#[cfg(test)]
mod tests {
    use crate::line::Line;
    use crate::meta_data::MetaData;
    use crate::page::{OrientationType, Page};
    use crate::page_format::{PageFormat};
    use crate::RsPdf;
    use crate::text::{Text};
    use crate::types::{Point, RGB};

    #[test]
    fn it_works() {

        let meta_data = MetaData::default();
        let mut pdf = RsPdf::new("My first PDF");

        let mut page = Page::new(OrientationType::Portrait, PageFormat::A4.get_format());
        page.add_content(Text::new("Hello World!", "F1", 16, Point(100.0, page.size().height_value() - 200.0), RGB(87, 150, 100)).into());
        page.add_content(Text::new("Other text!", "F1", 12, Point(0.0, page.size().height_value() - 20.0), RGB(74, 7, 7)).into());
        page.add_content(Line::new(Point(20.0, 200.0), Point(page.size().width_value() - 20.0, 200.0), RGB(0, 0, 0), 1.0).into());

        let mut page_tabloid = Page::new(OrientationType::Portrait, PageFormat::Tabloid.get_format());
        page_tabloid.add_content(Text::new("Hello", "F1", 42, Point(100.0, 700.0), RGB(87, 150, 100)).into());
        page_tabloid.add_content(Line::new(Point(20.0, 200.0), Point(page.size().width_value() - 20.0, 700.0), RGB(0, 0, 0), 1.0).into());

        pdf.add_page(page);
        pdf.add_page(page_tabloid);

        println!("{:?}", pdf.build());

    }
}
