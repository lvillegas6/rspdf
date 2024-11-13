

mod units;
mod page_format;
mod page;
mod meta_data;

use page::{Page};
use crate::meta_data::MetaData;

#[derive(Debug)]
struct RsPdf {
    meta_data: MetaData,

    pages: Vec<Page>,
}

impl RsPdf {

    pub fn new(meta_data: MetaData) -> RsPdf {
        RsPdf {
            meta_data,
            pages: Vec::new(),
        }
    }

    pub fn new_with_meta_data(meta_data: MetaData) -> RsPdf {
        RsPdf {
            meta_data,
            pages: Vec::new(),
        }
    }

    pub fn add_page(&mut self, page: Page) {
        self.pages.push(page);
    }

    pub fn build() -> Vec<u8> {

        //<< /Type /Catalog /Pages 2 0 R >>
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

        //FECHAS datelocalTime
        // Fonds global
        //paginas
        let meta_data = MetaData::default();
        let mut pdf = RsPdf::new(meta_data);

        //pdf.add_page(Page::new());

        println!("{:?}", pdf);
    }
}
