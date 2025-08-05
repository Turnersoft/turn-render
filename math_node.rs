use std::sync::Arc;

use crate::subjects::math::formalism::location::Located;
use crate::subjects::math::formalism::relations::MathRelation;
use crate::turn_render::RichText;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use ts_rs::TS;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct MathNode {
    pub id: String,
    pub content: Arc<MathNodeContent>,
}

impl MathNode {
    pub fn empty() -> MathNode {
        MathNode {
            id: String::new(),
            content: Arc::new(MathNodeContent::Empty),
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

    pub fn identifier(input: Identifier) -> MathNode {
        MathNode {
            id: input.body.clone(),
            content: Arc::new(MathNodeContent::Identifier(input)),
        }
    }

    pub fn string(input: String) -> MathNode {
        MathNode {
            id: input.clone(),
            content: Arc::new(MathNodeContent::String(input)),
        }
    }

    pub fn text(input: String) -> MathNode {
        MathNode {
            id: input.clone(),
            content: Arc::new(MathNodeContent::Text(input)),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum MathNodeContent {
    Empty, // dummy placeholder

    Text(String), // this is in story font

    String(String), // this is in latex math font

    // bracketed scopes
    Bracketed {
        inner: Arc<MathNode>,
        style: BracketStyle,
        size: BracketSize,
    },

    // matrix and tensor
    Matrix {
        rows: Vec<Vec<MathNode>>,
    },

    // multinary operations/funcions
    Multiplications {
        terms: Vec<(RefinedMulOrDivOperation, MathNode)>,
    },
    Additions {
        terms: Vec<(RefinedAddOrSubOperator, MathNode)>,
    },
    Division {
        numerator: Arc<MathNode>,
        denominator: Arc<MathNode>,
        style: DivisionStyle,
    },

    SumNotation {
        summand: Arc<MathNode>,
        variable: Option<MathNode>,
        lower_limit: Option<Arc<MathNode>>,
        upper_limit: Option<Arc<MathNode>>,
    },
    ProductNotation {
        multiplicand: Arc<MathNode>,
        variable: Option<MathNode>,
        lower_limit: Option<Arc<MathNode>>,
        upper_limit: Option<Arc<MathNode>>,
    },
    Fraction {
        numerator: Arc<MathNode>,
        denominator: Arc<MathNode>,
    },

    Power {
        base: Arc<MathNode>,
        exponent: Arc<MathNode>,
    },

    UnaryPostfixOperation {
        parameter: Arc<MathNode>,
        operator: Arc<MathNode>, // "!", "T", "%"
    },
    // the question is whether symbol should have separate variant? logical not is
    // is such example, but it probably has no interactivity, so we keep it in .
    UnaryPrefixOperation {
        parameter: Arc<MathNode>,
        operator: Arc<MathNode>, // "-", "∇", "∇²"
    },

    // this is different than SimpleUnaryFunction, this will use the special notation |x| instead of abs(x)
    Abs {
        parameter: Arc<MathNode>,
    },

    // general function names
    FunctionCall {
        name: Arc<MathNode>,
        parameters: Vec<MathNode>,
    },

    Quantity {
        number: String,
        scientific_notation: Option<MathNode>, // we need the id
        unit: Option<MathNode>,
    }, // Add more content types as needed

    ScientificNotation {
        magnitude: Arc<MathNode>,
        style: ScientificNotationStyle,
    },

    Identifier(Identifier),

    Unit {
        original_form: Arc<MathNode>,  // multiplication
        flattened_form: Arc<MathNode>, // multiplication
    },

    // universal relations for all theories
    Relationship {
        lhs: Arc<MathNode>,
        rhs: Arc<MathNode>,
        operator: RelationOperatorNode,
    },

    UnaryRelationship {
        subject: Arc<MathNode>,
        predicate: UnaryRelationOperatorNode,
    },

    // variable declarations
    VariableDefinition {
        name: Arc<MathNode>, // should only be MathNodeContent::identifier
        definition: Option<MathNode>,
    },

    FunctionDefinition {
        custom_function: Arc<MathNode>, // this ia MathNodeContent::FunctionCall
        definition: Option<MathNode>,
    },

    // Calculus
    Limit {
        function: Arc<MathNode>,
        variable: String,
        approaching_value: Arc<MathNode>,
    },
    Differential {
        target: Arc<MathNode>,
        order: Arc<MathNode>,
        diff_style: DifferentialStyle,
    },
    Integration {
        integrand: Arc<MathNode>,
        differentials: Vec<(Arc<MathNode>, Option<Arc<MathNode>>, Option<Arc<MathNode>>)>, // Array of (differential, lower_bound, upper_bound)
        domain: Option<Arc<MathNode>>, // Optional geometric domain rendered beneath the integral signs
    },

    // Quantified expression structure (e.g., "∀ x ∈ S" or "∃ x : P(x)")
    // This is a fundamental mathematical structure used across all theories
    QuantifiedExpression {
        quantifier: QuantificationNode,
        variables: Vec<Identifier>,       // The quantified variables
        domain: Option<Arc<MathNode>>,    // Optional domain (the "∈ S" part)
        predicate: Option<Arc<MathNode>>, // Optional predicate (the ": P(x)" part)
    },

    // logical connectives
    And(Vec<MathNode>),
    Or(Vec<MathNode>),
    Not(Arc<MathNode>),
    True,
    False,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Identifier {
    pub body: String,
    pub pre_script: Option<ScriptNode>,
    pub mid_script: Option<SpecialMiddleScriptNode>,
    pub post_script: Option<ScriptNode>,
    pub primes: usize,
    pub is_function: bool,
}

impl PartialOrd for Identifier {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Identifier {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.body.cmp(&other.body)
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)
    }
}

impl Identifier {
    pub fn new_simple(body: String) -> Self {
        Identifier {
            body,
            pre_script: None,
            mid_script: None,
            post_script: None,
            primes: 0,
            is_function: false,
        }
    }
    pub fn simple_string_subscript(name: String, subscript: String) -> Self {
        Identifier {
            body: name,
            pre_script: None,
            mid_script: None,
            post_script: Some(ScriptNode {
                subscripts: vec![MathNode::string(subscript)],
                superscripts: vec![],
            }),
            primes: 0,
            is_function: false,
        }
    }
    pub fn simple_text_subscript(name: String, subscript: String) -> Self {
        Identifier {
            body: name,
            pre_script: None,
            mid_script: None,
            post_script: Some(ScriptNode {
                subscripts: vec![MathNode::text(subscript)],
                superscripts: vec![],
            }),
            primes: 0,
            is_function: false,
        }
    }

    pub fn simple_identifier_subscript(name: String, subscript: Identifier) -> Self {
        Identifier {
            body: name,
            pre_script: None,
            mid_script: None,
            post_script: Some(ScriptNode {
                subscripts: vec![MathNode::identifier(subscript)],
                superscripts: vec![],
            }),
            primes: 0,
            is_function: false,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ScriptNode {
    pub subscripts: Vec<MathNode>,
    pub superscripts: Vec<MathNode>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ScientificNotationStyle {
    LowerCaseE,
    UpperCaseE,
    TimesTenPower,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum DifferentialStyle {
    Partial,
    Total,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum IntegralType {
    /// Single integral: ∫
    Single,
    /// Double integral: ∫∫
    Double,
    /// Triple integral: ∫∫∫
    Triple,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum IntegralDomain {
    /// Regular integral with no domain specification
    Regular,
    /// Integral over a geometric domain (e.g., ∫[C], ∫[D])
    Geometric(Arc<MathNode>),
    /// Integral with parameter domain (e.g., ∫[C|t:R])
    ParametricGeometric {
        path: Arc<MathNode>,
        parameters: Vec<(Arc<MathNode>, Arc<MathNode>)>, // (parameter, domain) pairs
    },
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum QuantificationNode {
    /// Universal quantification (∀)
    Universal,

    /// Existential quantification (∃)
    Existential,

    /// Unique existential quantification (∃!)
    UniqueExistential,

    /// Object defined in terms of others
    Defined,

    /// Arbitrary but fixed object
    Fixed,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum RefinedMulOrDivOperation {
    Multiplication(MulSymbol),
    Division(DivSymbol),
    None,
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
pub enum RefinedAddOrSubOperator {
    Addition,
    Subtraction,
    None,
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

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum MulSymbol {
    Times,       // \times for numbers
    Dot,         // \, for symbols
    LittleSpace, // \, for bracketed expressions
}
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum DivSymbol {
    Slash,  // /
    Divide, // ÷
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum UnitComponent {
    BaseUnit {
        name: BaseUnitTypeNode,
        prefix: Option<String>,
    },
    CompoundUnit {
        components: Vec<(UnitComponent, MathNode)>, // (unit, exponent)
    },
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum RelationOperatorNode {
    // Binary relations

    // Basic equality and inequality
    IsEqual,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,

    // Geometry binary relations
    Collinear,
    Perpendicular,
    Equivalent, // this is expression equivlence under "math", todo: this context can vary
    Similar,
    Congruent, // Geometric congruence

    // Set Theory binary relations
    ElementOf,        // ∈
    NotElementOf,     // ∉
    SubsetOf,         // ⊆
    ProperSubsetOf,   // ⊂
    SupersetOf,       // ⊇
    ProperSupersetOf, // ⊃
    Disjoint,         // A ∩ B = ∅
    Union,            // ∪
    Intersection,     // ∩
    CartesianProduct, // ×
    SameCardinality,  // |A| = |B|

    // Number Theory binary relations
    Divides,         // |
    NotDivides,      // ∤
    CongruentMod,    // ≡ (modular congruence)
    NotCongruentMod, // ≢
    AreCoprime,

    // Group Theory binary relations
    IsSubgroupOf,
    IsNormalSubgroupOf,
    IsIsomorphicTo, // ≅
    IsHomomorphicTo,
    IsQuotientOf,
    IsInCenterOf,
    AreConjugateIn,

    // Ring Theory binary relations
    IsSubringOf,
    IsIdealOf,

    // Topology binary relations
    IsOpenIn,
    IsClosedIn,
    IsHomeomorphicTo,
    IsDense,

    // Category Theory binary relations
    IsMorphismBetween,
    IsIsomorphismIn,
    IsMonomorphismIn,
    IsEpimorphismIn,
    IsNaturalTransformationBetween,
    IsAdjunctionBetween,
    ComposesTo,

    // Logic relations
    Implies, // →
    Iff,     // ↔

    // Custom relations
    Custom(String), // For custom notation not covered by built-in operators
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum UnaryRelationOperatorNode {
    // Number Theory unary relations
    IsPrime,
    IsComposite,

    // Group Theory unary relations
    HasOrderInGroup,
    HasUniqueInverse,

    // Ring Theory unary relations
    IsPrimeIdeal,
    IsMaximalIdeal,
    IsPrincipalIdeal,
    IsUnit,
    IsIrreducible,
    IsPrimeElement,
    IsField,
    IsIntegralDomain,
    IsUFD, // Unique Factorization Domain
    IsPID, // Principal Ideal Domain

    // Topology unary relations
    IsCompact,
    IsConnected,
    IsContinuous,
    Converges,
    IsHausdorff,

    // Category Theory unary relations
    IsObjectIn,
    IsEndomorphismIn,
    IsAutomorphismIn,

    // Set Theory unary operations
    Complement, // ^c
    PowerSet,   // P()

    // Custom unary relation
    Custom(String),
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

pub trait ToTurnMath {
    fn to_turn_math(&self, master_id: String) -> MathNode;
}

pub trait ToRichText {
    fn to_rich_text(&self) -> RichText;
}
