use crate::reference::Ref;

#[derive(Debug, Clone)]
pub struct Font {
    pub name: String,
    pub height: f32,
    pub font_ref: Ref,
}
