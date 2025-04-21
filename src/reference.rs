
#[derive(Debug)]
pub struct Ref {
    pub id: u32,
}

impl Ref {
    pub fn new(id: u32) -> Self {
        Ref { id }
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl From<Ref> for String {
    fn from(content: Ref) -> String {
        format!("{} 0", content.id)
    }
}