#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub content: Option<String>,
    pub description: Option<String>,
}
