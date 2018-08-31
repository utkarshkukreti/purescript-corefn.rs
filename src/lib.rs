#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

#[derive(Debug, Deserialize)]
pub struct Module {
    #[serde(rename = "moduleName")]
    pub name: Vec<String>,
    pub imports: Vec<Import>,
    pub exports: Vec<String>,
    pub decls: Vec<Decl>,
}

#[derive(Debug, Deserialize)]
pub struct Import {
    #[serde(rename = "moduleName")]
    pub module: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "bindType")]
pub enum Decl {
    NonRec(Bind),
    Rec { binds: Vec<Bind> },
}

#[derive(Debug, Deserialize)]
pub struct Bind {
    pub identifier: String,
    pub expression: Expression,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum Expression {
    Abs {
        argument: String,
        body: Box<Expression>,
    },
    Accessor {
        expression: Box<Expression>,
        #[serde(rename = "fieldName")]
        field: String,
    },
    App {
        abstraction: Box<Expression>,
        argument: Box<Expression>,
    },
    Case {
        #[serde(rename = "caseAlternatives")]
        alternatives: Vec<Alternative>,
        #[serde(rename = "caseExpressions")]
        expressions: Vec<Expression>,
    },
    Constructor {
        #[serde(rename = "constructorName")]
        name: String,
        #[serde(rename = "typeName")]
        type_: String,
        #[serde(rename = "fieldNames")]
        fields: Vec<String>,
    },
    Let {
        expression: Box<Expression>,
        binds: Vec<Decl>,
    },
    Literal {
        value: Literal,
    },
    Var {
        value: ModuleAndIdentifier,
    },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "literalType")]
pub enum Literal {
    #[serde(rename = "ArrayLiteral")]
    Array { value: Vec<Expression> },
    #[serde(rename = "BooleanLiteral")]
    Boolean { value: bool },
    #[serde(rename = "CharLiteral")]
    Char { value: char },
    #[serde(rename = "IntLiteral")]
    Int { value: i64 },
    #[serde(rename = "NumberLiteral")]
    Number { value: f64 },
    #[serde(rename = "ObjectLiteral")]
    Object { value: Vec<(String, Expression)> },
    #[serde(rename = "StringLiteral")]
    String { value: String },
}

#[derive(Debug, Deserialize)]
pub struct ModuleAndIdentifier {
    #[serde(rename = "moduleName")]
    pub module: Option<Vec<String>>,
    pub identifier: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Alternative {
    Guarded {
        binders: Vec<Binder>,
        expressions: Vec<GuardedExpression>,
    },
    Unguarded {
        binders: Vec<Binder>,
        expression: Expression,
    },
}

#[derive(Debug, Deserialize)]
pub struct GuardedExpression {
    pub guard: Expression,
    pub expression: Expression,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "binderType")]
pub enum Binder {
    #[serde(rename = "ConstructorBinder")]
    Constructor {
        #[serde(rename = "constructorName")]
        constructor: ModuleAndIdentifier,
        #[serde(rename = "typeName")]
        type_: ModuleAndIdentifier,
        binders: Vec<Binder>,
    },
    #[serde(rename = "LiteralBinder")]
    Literal { literal: LiteralBinder },
    #[serde(rename = "NamedBinder")]
    Named {
        identifier: String,
        binder: Box<Binder>,
    },
    #[serde(rename = "NullBinder")]
    Null {},
    #[serde(rename = "VarBinder")]
    Var { identifier: String },
}

#[derive(Debug, Deserialize)]
#[serde(tag = "literalType")]
pub enum LiteralBinder {
    #[serde(rename = "ArrayLiteral")]
    Array { value: Vec<Binder> },
    #[serde(rename = "BooleanLiteral")]
    Boolean { value: bool },
    #[serde(rename = "CharLiteral")]
    Char { value: char },
    #[serde(rename = "IntLiteral")]
    Int { value: i64 },
    #[serde(rename = "NumberLiteral")]
    Number { value: f64 },
    #[serde(rename = "ObjectLiteral")]
    Object { value: Vec<(String, Binder)> },
    #[serde(rename = "StringLiteral")]
    String { value: String },
}

pub fn from_str(str: &str) -> Result<Module, serde_json::Error> {
    serde_json::from_str(str)
}
