#[derive(Debug, PartialEq, Eq)]
pub(crate) enum Node {
    FunctionCall {
        name: String,
        args: Vec<String>,
        children: Vec<Node>,
    },
    Text(String),
}
