#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub content: String,
    pub description: Option<String>,
}
