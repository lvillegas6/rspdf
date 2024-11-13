
#[derive(Debug)]
pub struct MetaData {
    pub title: String,
    pub author: String,
    pub creation_date: String,
    pub modification_date: String,
    pub version: f32,
    pub encryption: bool,
}

impl MetaData {
    pub fn new(title: &str, version: f32, encryption: bool) -> MetaData {
        MetaData {
            title: title.to_string(),
            version,
            encryption,
            ..Self::default()
        }
    }
}

impl Default for MetaData {
    fn default() -> Self {
        MetaData {
            title: String::new(),
            author: String::from("rspdf"),
            creation_date: String::new(),
            modification_date: String::new(),
            version: 1.3,
            encryption: false,
        }
    }
}
