use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use ts_rs::TS;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum TurnTextLineNode {
    Math(MathNode, String),
    Phrase(String),
    Empty,
    Comment(String),
    Latex(String),
    PageLink(String),
    Image(String),
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
        differentials: Vec<(Box<MathNode>, Option<Box<MathNode>>, Option<Box<MathNode>>)>, // Array of (differential, lower_bound, upper_bound)
        domain: Option<Box<MathNode>>, // Optional geometric domain rendered beneath the integral signs
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
        terms: Vec<(RefinedAddOrSubOperator, MathNode)>,
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

    ScientificNotation {
        magnitude: Box<MathNode>,
        style: ScientificNotationStyle,
    },

    BaseUnit(String),

    Relationship {
        lhs: Box<MathNode>,
        rhs: Box<MathNode>,
        operator: RelationOperatorNode,
    },

    UnaryRelationship {
        subject: Box<MathNode>,
        predicate: UnaryRelationOperatorNode,
    },

    VariableDefinition {
        name: Box<MathNode>,
        definition: Option<MathNode>,
    },

    FunctionDefinition {
        custom_function: Box<MathNode>,
        definition: Option<MathNode>,
    },

    // Calculus
    Differential {
        target: Box<MathNode>,
        order: Box<MathNode>,
        diff_style: DifferentialStyle,
    },
    Theorem {
        name: String,
        description: String,
        goal: Box<MathNode>,
        proofs: Vec<MathNode>, // ProofForest
    },

    ProofGoal {
        statement: Box<MathNode>,   // The main statement being proven
        quantifiers: Vec<MathNode>, // Quantified objects in this state
        variables: Vec<MathNode>,   // Variables with assigned values
    },

    ProofForest {
        // Summary of the forest state
        roots: Vec<MathNode>, // a vec of proof trees
    },

    Quantifier {
        quantification: QuantificationNode,
        variable: Box<MathNode>,
        body: Box<MathNode>,
    },
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
    Geometric(Box<MathNode>),
    /// Integral with parameter domain (e.g., ∫[C|t:R])
    ParametricGeometric {
        path: Box<MathNode>,
        parameters: Vec<(Box<MathNode>, Box<MathNode>)>, // (parameter, domain) pairs
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

pub trait ToTurnMath {
    fn to_turn_math(&self, master_id: String) -> MathNode;
}
