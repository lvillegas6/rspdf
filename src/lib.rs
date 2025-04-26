

mod units;
mod page_format;
mod page;
mod meta_data;
mod text;
mod reference;
mod line;
mod types;
mod font;

use std::collections::HashMap;
use std::io::{Read};
use std::ops::Add;
use page::{Page};
use crate::font::Font;
use crate::meta_data::MetaData;
use crate::reference::Ref;

#[derive(Debug)]
struct RsPdf {
    meta_data: MetaData,
    pages: Vec<Page>,
    current_id: u32,
    fonts: HashMap<String, Font>,
    current_font: u32,
    document: Vec<u8>,
    xref_offset: Vec<u32>,
}

impl RsPdf {

    pub fn new(title: &str) -> RsPdf {
        let meta_data = MetaData {
            title: title.to_string(),
            ..MetaData::default()
        };

        let version = meta_data.version;
        RsPdf {
            meta_data,
            pages: Vec::new(),
            current_id: 1,
            current_font: 0,
            document: Vec::from(format!("%PDF-{}\n", version)),
            xref_offset: Vec::new(),
            fonts: HashMap::new(),
        }
    }

    pub fn new_with_meta_data(meta_data: MetaData) -> RsPdf {
        RsPdf {
            meta_data,
            pages: Vec::new(),
            current_id: 1,
            current_font: 0,
            document: Vec::new(),
            xref_offset: Vec::new(),
            fonts: HashMap::new(),
        }
    }

    pub fn add_font(&mut self, data: &[u8]) -> Result<String, String> {

        self.current_font += 1;
        let font_name = format!("F{}", self.current_font);

        let font_obj_id: Ref = self.alloc_id();
        let font_obj_id_str: String = font_obj_id.clone().into();

        let face_obj_id: String = self.alloc_id().into();
        let font_file_obj_id: String = self.alloc_id().into();


        let face = ttf_parser::Face::parse(data, 0)
            .map_err(|_| "Error parsing font data".to_string())?;

        // /FontBBox [xMin yMin xMax yMax]
        let bbox = face.global_bounding_box();

        // /Ascent y /Descent
        let units_per_em = face.units_per_em() as f32;
        let ascent = face.ascender();
        let descent = face.descender();
        let height_in_font_units = (ascent - descent) as f32;

        let font = Font {
            font_ref: font_obj_id.clone(),
            name: font_name.clone(),
            height: height_in_font_units / units_per_em,
        };
        self.fonts.insert(font_name.clone(), font);

        let cap_height = face.capital_height().unwrap_or(ascent);
        let italic_angle = face.italic_angle();

        self.document.extend_from_slice(format!(
            "{} obj\n<< /Type /Font /Subtype /TrueType /BaseFont /{} /Encoding /WinAnsiEncoding /FirstChar 32 /LastChar 255 /FontDescriptor {} R >>\nendobj\n",
            font_obj_id_str,
            font_name.replace(" ", ""),
            face_obj_id
        ).as_bytes());

        self.xref_offset.push(self.document.len() as u32);


        self.document.extend_from_slice(format!(
            "{} obj\n<< /Type /FontDescriptor /FontName /{} /Flags 32 /FontBBox [ {} {} {} {} ] /ItalicAngle {} /Ascent {} /Descent {} /CapHeight {} /StemV 80 /FontFile2 {} R >>\nendobj\n",
            face_obj_id,
            font_name,
            bbox.x_min, bbox.y_min, bbox.x_max, bbox.y_max,
            italic_angle,
            ascent,
            descent,
            cap_height,
            font_file_obj_id
        ).as_bytes());
        self.xref_offset.push(self.document.len() as u32);

        self.document.extend_from_slice(format!("{} obj\n<< /Length {} >>\nstream\n", font_file_obj_id, data.len()).as_bytes());
        self.document.extend_from_slice(data);
        self.document.extend_from_slice(b"\nendstream\nendobj\n");
        self.xref_offset.push(self.document.len() as u32);

        Ok(font_name)
    }

    pub fn get_font(&self, font_name: &str) -> Option<&Font> {
        self.fonts.get(font_name)
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

        let catalog_obj_id: String = self.alloc_id().into();
        let pages_obj_id: String = self.alloc_id().into();
        self.document.extend_from_slice(format!("{} obj\n<< /Type /Catalog /Pages {} R >>\nendobj\n", catalog_obj_id, pages_obj_id).as_bytes());
        self.xref_offset.push(self.document.len() as u32);
        let mut kids: Vec<String> = vec![];
        let page_length = self.pages.len();
        let pages = self.pages.clone();

        let mut fonts_dict = String::new();
        for (font_name, font_ref) in self.fonts.iter() {
            fonts_dict.push_str(&format!("/{0} {1} 0 R ", font_name.replace(" ", ""), font_ref.font_ref.id));
        }

        for page in pages {
            let page_obj_id: String = self.alloc_id().into();
            kids.push(page_obj_id.clone().add(" R"));
            let page_size = page.size();
            let content_obj_id: String = self.alloc_id().into();
            self.document.extend_from_slice(format!(
                "{} obj\n<< /Type /Page /Parent {} R /MediaBox [0 0 {} {}] /Contents {} R /Rotate 0 /Resources << /Font << {} >> >> >>\nendobj\n",
                page_obj_id,
                pages_obj_id,
                page_size.width_value(),
                page_size.height_value(),
                content_obj_id,
                fonts_dict
            ).as_bytes());
            self.xref_offset.push(self.document.len() as u32);

            self.document.extend_from_slice(format!("{} obj\n<< /Length {} >>\nstream\n", content_obj_id, page.length()).as_bytes());
            self.document.extend_from_slice(page.content());
            self.document.extend_from_slice(b"\nendstream\nendobj\n");
            self.xref_offset.push(self.document.len() as u32);
        }

        self.document.extend_from_slice(format!("{} obj\n<< /Type /Pages /Kids [{}] /Count {} >>\nendobj\n", pages_obj_id, kids.join(" "), page_length).as_bytes());
        self.xref_offset.push(self.document.len() as u32);

        let xref_obj_id = self.alloc_id();
        self.document.extend_from_slice(format!("xref 0 {}\n", xref_obj_id.id).as_bytes());
        self.document.extend_from_slice(b"0000000000 65535 f\n");

        for offset in self.xref_offset.iter() {
            self.document.extend_from_slice(format!("{:010} 00000 n\n", offset).as_bytes());
        }
        self.document.extend_from_slice(format!("trailer\n<< /Root {} R /Size {} >>\nstartxref\n{}\n", catalog_obj_id, self.current_id, self.document.len()).as_bytes());
        self.document.extend_from_slice(b"%%EOF");

        self.document.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::line::Line;
    use crate::page::{OrientationType, Page};
    use crate::page_format::{PageFormat};
    use crate::RsPdf;
    use crate::text::{Text};
    use crate::types::{Point, RGB};
    use std::fs::File;
    use std::io::Write;

    #[test]
    fn it_works() {
        let mut file = File::create("template.pdf").unwrap();
        let mut pdf = RsPdf::new("My first PDF");

        let font_name = pdf.add_font(include_bytes!("../assets/Helvetica.ttf")).unwrap();
        let other_font_name = pdf.add_font(include_bytes!("../assets/NotoSansMono.ttf")).unwrap();

        let font = pdf.get_font(&font_name).unwrap();
        let other_font = pdf.get_font(&other_font_name).unwrap();

        let mut page = Page::new(OrientationType::Portrait, PageFormat::A4.get_format());
        let page_height = page.size().height_value();
        page.add_content(Text::new("Hello World!", font, 100, Point(0.0, page_height - 60.0), RGB(87, 150, 100)).into());
        page.add_content(Text::new("Other Hello World!", other_font, 32, Point(0.0, page_height), RGB(74, 7, 7)).into());
        page.add_content(Line::new(Point(20.0, 200.0), Point(page.size().width_value() - 20.0, 200.0), RGB(0, 0, 0), 1.0).into());

        let mut page_tabloid = Page::new(OrientationType::Portrait, PageFormat::Tabloid.get_format());
        let page_tabloid_height = page_tabloid.size().height_value();
        page_tabloid.add_content(Text::new("Hello", font, 32, Point(100.0, page_tabloid_height - 700.0), RGB(87, 150, 100)).into());
        page_tabloid.add_content(Line::new(Point(20.0, 200.0), Point(page.size().width_value() - 20.0, page_tabloid_height - 700.0), RGB(0, 0, 0), 1.0).into());

        pdf.add_page(page);
        pdf.add_page(page_tabloid);

        file.write_all(pdf.build().as_slice()).unwrap();

    }
}
