use crate::page_format::PageSize;

#[derive(Debug)]
pub enum OrientationType {
    Portrait,
    Landscape
}

impl OrientationType {
    pub fn rotate(self) -> i16 {
        match self {
            OrientationType::Portrait => 0,
            OrientationType::Landscape => 90,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Page {
    size: PageSize,
    content: Vec<u8>,
}

impl Page {

    pub fn new(orientation: OrientationType, size: PageSize) -> Page {
        let base_size = match orientation {
            OrientationType::Portrait => size,
            OrientationType::Landscape => size.inverse(),
        };

        Page {
            size: base_size,
            content: Vec::new(),
        }
    }

    pub fn add_content(&mut self, content: Vec<u8>) {
        self.content.extend_from_slice(content.as_slice());
    }

    pub fn size(&self) -> &PageSize {
        &self.size
    }

    pub fn length(&self) -> usize {
        self.content.len()
    }
    pub fn content(&self) -> &[u8] {
        &self.content
    }

}

