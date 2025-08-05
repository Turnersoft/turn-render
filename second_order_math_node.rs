use std::sync::Arc;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::turn_render::math_node::{MathNode, ToTurnMath};
use crate::turn_render::{
    AbstractionMetadata, CollapsibleBlockNode, Identifier, InteractiveControls, LinkTarget,
    QuantifierType, RichText, RichTextSegment, Section, SectionContentNode, SelectableProperty,
};

/// This matches the TypeScript StructuredMathNode bindings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum SecondOrderMathNode {
    Logic(LogicalNode),
    Judgement(Judgement),
    SystemOf(Vec<MathNode>), // can be equality or inequality
    Solution(Solution),
    VariableDeclaration(VariableDeclaration), // name : type pair
    QuantifiedVariableDeclarationGroup(QuantifiedVariableDeclarationGroup), // name : type pair
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum LogicalNode {
    And(Vec<LogicalNode>), // vertically aligned
    Or(Vec<LogicalNode>),  // horizontally aligned
    // Not(Arc<LogicalNode>),
    // Implies(Arc<LogicalNode>, Arc<LogicalNode>),
    // Equivalence(Arc<LogicalNode>, Arc<LogicalNode>),
    // Equality(Arc<LogicalNode>, Arc<LogicalNode>), // this is special equality
    Atomic(MathNode),

    True,
    False,
}

pub trait ToLogicalNode {
    fn to_logical_node(&self) -> LogicalNode;
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Judgement {
    pub non_quantifiers: Vec<VariableDeclaration>, // non-quantified variables with type or value or constraint info
    pub quantifiers: Vec<QuantifiedVariableDeclarationGroup>,
    pub statement: LogicalNode,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct QuantifierGroup {
    pub quantifier_type: QuantifierType,
    pub variables: Vec<SecondOrderMathNode>, // variable with type or value or constraint info
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct VariableDeclaration {
    pub name: MathNode,
    pub type_info: RichText,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum QuantifiedVariableDeclarationGroup {
    Exists(VariableDeclaration),
    UniqueExists(VariableDeclaration),
    ForAll(Vec<VariableDeclaration>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum TheoremLikeKind {
    Theorem,
    Lemma,
    Proposition,
    Corollary,
    Conjecture,
    Principle,
    Axiom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Solution {
    // todo: what do we need to store the space?
    pub solution_space: Vec<Section>,
    // pub solution_space_type: SolutionSpaceType,
}
