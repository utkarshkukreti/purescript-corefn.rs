#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

pub mod walk;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Module {
    #[serde(rename = "moduleName")]
    pub name: Vec<String>,
    pub imports: Vec<Import>,
    pub exports: Vec<String>,
    pub decls: Vec<Decl>,
    pub foreign: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Import {
    #[serde(rename = "moduleName")]
    pub module: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(tag = "bindType")]
pub enum Decl {
    NonRec(Bind),
    Rec { binds: Vec<Bind> },
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Bind {
    pub identifier: String,
    pub expression: Expression,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(tag = "type")]
pub enum Expression {
    Abs {
        argument: String,
        body: Box<Expression>,
        annotation: Annotation,
    },
    Accessor {
        expression: Box<Expression>,
        #[serde(rename = "fieldName")]
        field: String,
        annotation: Annotation,
    },
    App {
        abstraction: Box<Expression>,
        argument: Box<Expression>,
        annotation: Annotation,
    },
    Case {
        #[serde(rename = "caseAlternatives")]
        alternatives: Vec<Alternative>,
        #[serde(rename = "caseExpressions")]
        expressions: Vec<Expression>,
        annotation: Annotation,
    },
    Constructor {
        #[serde(rename = "constructorName")]
        name: String,
        #[serde(rename = "typeName")]
        type_: String,
        #[serde(rename = "fieldNames")]
        fields: Vec<String>,
        annotation: Annotation,
    },
    Let {
        expression: Box<Expression>,
        binds: Vec<Decl>,
        annotation: Annotation,
    },
    Literal {
        value: Literal,
        annotation: Annotation,
    },
    Var {
        value: Qualified,
        annotation: Annotation,
    },
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Qualified {
    #[serde(rename = "moduleName")]
    pub module: Option<Vec<String>>,
    pub identifier: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct GuardedExpression {
    pub guard: Expression,
    pub expression: Expression,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(tag = "binderType")]
pub enum Binder {
    #[serde(rename = "ConstructorBinder")]
    Constructor {
        #[serde(rename = "constructorName")]
        constructor: Qualified,
        #[serde(rename = "typeName")]
        type_: Qualified,
        binders: Vec<Binder>,
        annotation: Annotation,
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

#[derive(Clone, Debug, PartialEq, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct Annotation {
    pub meta: Option<Meta>,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
#[serde(tag = "metaType")]
pub enum Meta {
    #[serde(rename = "IsConstructor")]
    Constructor {
        identifiers: Vec<String>,
        #[serde(rename = "constructorType")]
        type_: ConstructorType,
    },
    #[serde(rename = "IsForeign")]
    Foreign,
    #[serde(rename = "IsNewtype")]
    Newtype,
    #[serde(rename = "IsTypeClassConstructor")]
    TypeClassConstructor,
    #[serde(rename = "IsWhere")]
    Where,
}

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub enum ConstructorType {
    #[serde(rename = "ProductType")]
    Product,
    #[serde(rename = "SumType")]
    Sum,
}

pub fn from_str(str: &str) -> Result<Module, serde_json::Error> {
    serde_json::from_str(str)
}
