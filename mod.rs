use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use ts_rs::TS;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum RelationOperatorNode {
    IsEqual,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
    Collinear,
    Perpendicular,
    Equivalent, // this is expression equivlence under "math", todo: this context can vary
    Similar,
    Congruent,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum AddOrSubOperatorNode {
    Addition,
    Subtraction,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SpecialMiddleScriptNode {
    pub super_script: Vec<SpecialMiddleScriptContentTypeNode>,
    pub sub_script: Vec<SpecialMiddleScriptContentTypeNode>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum SpecialMiddleScriptContentTypeNode {
    Hat,
    Dot(usize),
    Tilde,
    Bar,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum BaseUnitTypeNode {
    Unitless,
    Meter,
    Second,
    Gram,
    Ampere,
    Mole,
    Kelvin,
    Candela,

    Newton,
    Joul,
    Hertz,
    Pascal,
    Volt,
    Ohm,
    Steradian, // Added steradian to the enum
    Watt,
    Coulumb,
    Siemens, // Added Siemens assuming "S" stands for Siemens
    Lux,
    Lumen,
    Weber,
    Tesla,
    Decibel, // Added Decibel assuming "dpt" stands for Decibel
    Henry,
    Hour,   // Added Hour assuming "h" stands for Hour
    Minute, // Added Minute assuming "min" stands for Minute

    Custom(String),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct MathNode {
    pub id: String,
    pub content: Box<MathNodeContent>,
}

impl MathNode {
    pub fn empty() -> MathNode {
        MathNode {
            id: String::new(),
            content: Box::new(MathNodeContent::Empty),
        }
    }
    pub fn is_quantity(&self) -> bool {
        matches!(*self.content, MathNodeContent::Quantity { .. })
    }
    pub fn is_expression_in_bracket(&self) -> bool {
        matches!(*self.content, MathNodeContent::Bracketed { .. })
    }
    pub fn is_expression_in_round_bracket(&self) -> bool {
        matches!(
            *self.content,
            MathNodeContent::Bracketed {
                style: BracketStyle::Round,
                ..
            }
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum MathNodeContent {
    Empty,
    Text(String),
    String(String),
    Integration {
        integrand: Box<MathNode>,
        variable: String,
        lower_limit: Option<Box<MathNode>>,
        upper_limit: Option<Box<MathNode>>,
    },
    Limit {
        function: Box<MathNode>,
        variable: String,
        approaching_value: Box<MathNode>,
    },
    Multiplications {
        terms: Vec<(RefinedMulOrDivOperation, MathNode)>,
    },
    Additions {
        terms: Vec<(AddOrSubOperatorNode, MathNode)>,
    },
    Division {
        numerator: Box<MathNode>,
        denominator: Box<MathNode>,
        style: DivisionStyle,
    },
    SumNotation {
        summand: Box<MathNode>,
        variable: Option<MathNode>,
        lower_limit: Option<Box<MathNode>>,
        upper_limit: Option<Box<MathNode>>,
    },
    ProductNotation {
        multiplicand: Box<MathNode>,
        variable: Option<MathNode>,
        lower_limit: Option<Box<MathNode>>,
        upper_limit: Option<Box<MathNode>>,
    },
    Fraction {
        numerator: Box<MathNode>,
        denominator: Box<MathNode>,
    },
    Bracketed {
        inner: Box<MathNode>,
        style: BracketStyle,
        size: BracketSize,
    },
    Matrix {
        rows: Vec<Vec<MathNode>>,
    },
    // Unary Functions
    LogFunction {
        base: Option<MathNode>, // "2", "10", "e", "empty" etc.
        parameter: Box<MathNode>,
    },
    Log2 {
        parameter: Box<MathNode>,
    },
    Log10 {
        parameter: Box<MathNode>,
    },
    Ln {
        parameter: Box<MathNode>,
    },
    UnaryPostfix {
        parameter: Box<MathNode>,
        operator: String, // "!", "T", "%"
    },
    UnaryPrefix {
        parameter: Box<MathNode>,
        operator: String, // "-", "√"
    },
    Abs {
        parameter: Box<MathNode>,
    },
    // Multinary Functions
    Power {
        base: Box<MathNode>,
        exponent: Box<MathNode>,
    },
    CustomFunction {
        name: Box<MathNode>,
        parameters: Vec<MathNode>,
    },
    SimpleUnaryFunction {
        name: String,
        parameter: Box<MathNode>,
    },
    SimpleMultinaryFunction {
        name: String,
        parameters: Vec<MathNode>,
    },
    Quantity {
        number: String,
        unit: Option<MathNode>,
    }, // Add more content types as needed
    Identifier {
        body: String,
        pre_script: Option<Box<MathNode>>,
        mid_script: Option<SpecialMiddleScriptNode>,
        post_script: Option<Box<MathNode>>,
        primes: usize,
        is_function: bool,
    },
    Script {
        subscripts: Vec<MathNode>,
        superscripts: Vec<MathNode>,
    },

    Unit {
        original_form: Box<MathNode>,  // multiplication
        flattened_form: Box<MathNode>, // multiplication
    },

    BaseUnit(String),

    Relationship {
        lhs: Box<MathNode>,
        rhs: Box<MathNode>,
        operator: RelationOperatorNode,
    },

    VariableDefinition {
        name: Box<MathNode>,
        definition: Option<MathNode>,
    },

    FunctionDefinition {
        custom_function: Box<MathNode>,
        definition: Option<MathNode>,
    },
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum RefinedMulOrDivOperation {
    Multiplication(MulSymbol),
    Division(DivSymbol),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum MulSymbol {
    Quantity, // \times for numbers
    Symbol,   // \, for symbols
    Simple,   // \, for bracketed expressions
    None,     // no symbol
}
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum DivSymbol {
    Slash,  // /
    Divide, // ÷
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum AdditionOperator {
    Plus,  // +
    Minus, // -
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum DivisionStyle {
    Fraction, // \frac{a}{b}
    Inline,   // a/b
    Division, // a÷b
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum BracketStyle {
    Round,          // ( )
    Square,         // [ ]
    Curly,          // { }
    Angle,          // ⟨ ⟩
    Vertical,       // | |
    DoubleVertical, // ∥ ∥
    Ceiling,        // ⌈ ⌉
    Floor,          // ⌊ ⌋
    None,           // Invisible brackets
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum BracketSize {
    Normal,
    Auto,      // \left \right
    Sized(u8), // \big, \Big, \bigg, \Bigg
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum MulType {
    Quantity, // \times
    Symbol,   // \,
    Simple,   // \,
    None,
}

impl MathNode {
    pub fn to_json(&self) -> Value {
        json!({
            "id": self.id,
            "content": self.content.to_json(),
        })
    }
}

impl MathNodeContent {
    pub fn to_json(&self) -> Value {
        match self {
            MathNodeContent::Empty => json!({
                "type": "empty",
            }),
            MathNodeContent::String(s) => json!(s),
            MathNodeContent::Integration {
                integrand,
                variable,
                lower_limit,
                upper_limit,
            } => json!({
                "type": "integration",
                "integrand": integrand.to_json(),
                "variable": variable,
                "lower_limit": lower_limit.as_ref().map(|l| l.to_json()),
                "upper_limit": upper_limit.as_ref().map(|u| u.to_json()),
            }),
            MathNodeContent::Limit {
                function,
                variable,
                approaching_value,
            } => json!({
                "type": "limit",
                "function": function.to_json(),
                "variable": variable,
                "approaching_value": approaching_value.to_json(),
            }),
            MathNodeContent::Multiplications { terms } => json!({
                "type": "multiplication",
                "terms": terms.iter().map(|(op, term)| json!({
                    "operator": match op {
                        RefinedMulOrDivOperation::Multiplication(symbol_type) => json!({
                            "type": "multiplication",
                            "symbol": match symbol_type {
                                MulSymbol::Quantity => "times",
                                MulSymbol::Symbol => "symbol",
                                MulSymbol::Simple => "simple",
                                MulSymbol::None => "none",
                            }
                        }),
                        RefinedMulOrDivOperation::Division(symbol_type) => json!({
                            "type": "division",
                            "symbol": match symbol_type {
                                DivSymbol::Slash => "slash",
                                DivSymbol::Divide => "divide",
                            }
                        }),
                    },
                    "term": term.to_json(),
                })).collect::<Vec<_>>(),
            }),
            MathNodeContent::Additions { terms } => json!({
                "type": "addition",
                "terms": terms.iter().map(|(op, term)| json!({
                    "operator": match op {
                        AddOrSubOperatorNode::Addition => "plus",
                        AddOrSubOperatorNode::Subtraction => "minus",
                    },
                    "term": term.to_json(),
                })).collect::<Vec<_>>(),
            }),
            MathNodeContent::Division {
                numerator,
                denominator,
                style,
            } => json!({
                "type": "division",
                "numerator": numerator.to_json(),
                "denominator": denominator.to_json(),
                "style": match style {
                    DivisionStyle::Fraction => "fraction",
                    DivisionStyle::Inline => "inline",
                    DivisionStyle::Division => "division",
                },
            }),
            MathNodeContent::SumNotation {
                summand,
                variable,
                lower_limit,
                upper_limit,
            } => json!({
                "type": "sum",
                "summand": summand.to_json(),
                "variable": variable.as_ref().map(|l| l.to_json()),
                "lower_limit": lower_limit.as_ref().map(|l| l.to_json()),
                "upper_limit": upper_limit.as_ref().map(|u| u.to_json()),
            }),
            MathNodeContent::ProductNotation {
                multiplicand,
                variable,
                lower_limit,
                upper_limit,
            } => json!({
                "type": "product",
                "multiplicand": multiplicand.to_json(),
                "variable": variable.as_ref().map(|l| l.to_json()),
                "lower_limit": lower_limit.as_ref().map(|l| l.to_json()),
                "upper_limit": upper_limit.as_ref().map(|u| u.to_json()),
            }),
            MathNodeContent::Fraction {
                numerator,
                denominator,
            } => json!({
                "type": "fraction",
                "numerator": numerator.to_json(),
                "denominator": denominator.to_json(),
            }),
            MathNodeContent::Bracketed { inner, style, size } => json!({
                "type": "bracketed".to_owned(),
                "inner": inner.to_json(),
                "style": match style {
                    BracketStyle::Round => "round",
                    BracketStyle::Square => "square",
                    BracketStyle::Curly => "curly",
                    BracketStyle::Angle => "angle",
                    BracketStyle::Vertical => "vertical",
                    BracketStyle::DoubleVertical => "double_vertical",
                    BracketStyle::Ceiling => "ceiling",
                    BracketStyle::Floor => "floor",
                    BracketStyle::None => "none",
                }.to_owned(),
                "size": match size {
                    BracketSize::Normal => "normal".to_owned(),
                    BracketSize::Auto => "auto".to_owned(),
                    BracketSize::Sized(n) => format!("sized_{}", n),
                },
            }),
            MathNodeContent::Matrix { rows } => json!({
                "type": "matrix",
                "rows": rows.iter().map(|row|
                    row.iter().map(|cell| cell.to_json()).collect::<Vec<_>>()
                ).collect::<Vec<_>>(),
            }),
            MathNodeContent::LogFunction { base, parameter } => json!({
                "type": "logarithmic_function",
                "base": base,
                "parameter": parameter.to_json(),
            }),
            MathNodeContent::UnaryPostfix {
                parameter,
                operator,
            } => json!({
                "type": "unary_postfix",
                "parameter": parameter.to_json(),
                "operator": operator,
            }),
            MathNodeContent::UnaryPrefix {
                parameter,
                operator,
            } => json!({
                "type": "unary_prefix",
                "parameter": parameter.to_json(),
                "operator": operator,
            }),
            MathNodeContent::Abs { parameter } => json!({
                "type": "absolute_value",
                "parameter": parameter.to_json(),
            }),
            MathNodeContent::Power { base, exponent } => json!({
                "type": "power_function",
                "base": base.to_json(),
                "exponent": exponent.to_json(),
            }),
            MathNodeContent::CustomFunction { name, parameters } => json!({
                "type": "custom_function",
                "name": name.to_json(),
                "parameters": parameters.iter().map(|p| p.to_json()).collect::<Vec<_>>(),
            }),
            MathNodeContent::SimpleUnaryFunction { name, parameter } => json!({
                "type": "unary_function",
                "name": name,
                "parameter": parameter.to_json(),
            }),
            MathNodeContent::SimpleMultinaryFunction { name, parameters } => json!({
                "type": "multinary_function",
                "name": name,
                "parameters": parameters.iter().map(|p| p.to_json()).collect::<Vec<_>>(),
            }),
            MathNodeContent::Quantity { number, unit } => json!({
                "type": "quantity",
                "number": number,
                "unit": unit.as_ref().map(|u| u.to_json()),
            }),
            MathNodeContent::Identifier {
                body,
                pre_script,
                mid_script,
                post_script,
                primes,
                is_function,
            } => json!({
                "type": "identifier",
                "body": body,
                "pre_script": pre_script.as_ref().map(|p| p.to_json()),
                "mid_script": mid_script.as_ref().map(|m| m.to_json()),
                "post_script": post_script.as_ref().map(|p| p.to_json()),
                "primes": primes,
                "is_function": is_function,
            }),
            MathNodeContent::Script {
                subscripts,
                superscripts,
            } => json!({
                "type": "script",
                "subscripts": subscripts.iter().map(|s| s.to_json()).collect::<Vec<_>>(),
                "superscripts": superscripts.iter().map(|s| s.to_json()).collect::<Vec<_>>(),
            }),
            MathNodeContent::Unit {
                original_form,
                flattened_form,
            } => json!({
                "type": "unit",
                "original": original_form.to_json(),
                "is_flattened": flattened_form.to_json(),
            }),
            MathNodeContent::BaseUnit(string) => json!({
                "type": "base_unit",
                "name": string,
            }),
            MathNodeContent::Log2 { parameter } => todo!(),
            MathNodeContent::Log10 { parameter } => todo!(),
            MathNodeContent::Ln { parameter } => todo!(),
            MathNodeContent::Text(string) => json!({
                "type": "text",
                "content": string,
            }),
            MathNodeContent::Relationship { lhs, rhs, operator } => json!({
                "type": "relationship",
                "lhs": lhs.to_json(),
                "rhs": rhs.to_json(),
                "operator": match operator {
                    RelationOperatorNode::Equal|RelationOperatorNode::IsEqual=>"equal",
                    RelationOperatorNode::NotEqual=>"not_equal",
                    RelationOperatorNode::Greater=>"greater",
                    RelationOperatorNode::Less=>"less",
                    RelationOperatorNode::GreaterEqual=>"greater_equal",
                    RelationOperatorNode::LessEqual=>"less_equal",
                    RelationOperatorNode::Collinear => "collinear",
                    RelationOperatorNode::Perpendicular => "perpendicular",
                    RelationOperatorNode::Equivalent => "equivalent",
                    RelationOperatorNode::Similar => "similar",
                    RelationOperatorNode::Congruent => "congruent",
                },
            }),
            MathNodeContent::VariableDefinition { name, definition } => json!({
                "type": "variable_definition",
                "name": name.to_json(),
                "definition": definition.as_ref().map(|d| d.to_json()),
            }),
            MathNodeContent::FunctionDefinition {
                custom_function,
                definition,
            } => json!({
                "type": "function_definition",
                "custom_function": custom_function.to_json(),
                "definition": definition.as_ref().map(|d| d.to_json()),
            }),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum UnitComponent {
    BaseUnit {
        name: BaseUnitTypeNode,
        prefix: Option<String>,
    },
    CompoundUnit {
        components: Vec<(UnitComponent, MathNode)>, // (unit, exponent)
    },
}

pub trait ToTurnMath {
    fn to_turn_math(&self, master_id: String) -> MathNode;
}

impl SpecialMiddleScriptNode {
    pub fn to_json(&self) -> Value {
        json!({
            "superscripts": self.super_script.iter().map(|s| match s {
                SpecialMiddleScriptContentTypeNode::Bar => "bar".to_string(),
                SpecialMiddleScriptContentTypeNode::Dot(n) => format!("dot_{}", n),
                SpecialMiddleScriptContentTypeNode::Hat => "hat".to_string(),
                SpecialMiddleScriptContentTypeNode::Tilde => "tilde".to_string(),
            }).collect::<Vec<_>>(),
            "subscripts": self.sub_script.iter().map(|s| match s {
                SpecialMiddleScriptContentTypeNode::Bar => "bar".to_string(),
                SpecialMiddleScriptContentTypeNode::Dot(n) => format!("dot_{}", n),
                SpecialMiddleScriptContentTypeNode::Hat => "hat".to_string(),
                SpecialMiddleScriptContentTypeNode::Tilde => "tilde".to_string(),
            }).collect::<Vec<_>>(),
        })
    }
}
