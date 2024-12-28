#[derive(Clone, Debug)]
pub enum ChatEvent {
    Selected { id: String },
    None,
}
