#[derive(Debug, Clone)]
pub struct Document {
    pub statements: Vec<Node>,
}

#[derive(Debug, Clone)]
pub enum Node {
    VariableDecl(VariableDecl),
    Command(Command),
    Block(Block),
    Property(Property),
}

#[derive(Debug, Clone)]
pub struct VariableDecl {
    pub name: String,
    pub value: Node,
}

#[derive(Debug, Clone)]
pub struct VariableRef {
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub name: String,
    pub args: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub name: String,
    pub attributes: Vec<Attribute>,
    pub body: Vec<Node>,
}

#[derive(Debug, Clone)]
pub struct Attribute {
    pub target: String,
    pub params: Vec<Param>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: Option<Value>,
    pub params: Vec<Param>,
}

#[derive(Debug, Clone)]
pub struct VariableRef {
    pub name: String,
}
