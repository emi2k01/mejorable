#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Node {
    FunctionCall {
        name: String,
        options: Vec<String>,
        children: Vec<Node>,
    },
    Text(String),
}
