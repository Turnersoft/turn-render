use std::sync::Arc;

use crate::turn_render::{MathNode, RichText, RichTextSegment};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum SecondOrderMathNode {
    Logic(LogicalNode),
    Judgement(Judgement),
    SystemOf(Vec<MathNode>), // can be equality or inequality
    Solution(Solution),
    VariableDeclaration(VariableDeclaration), // name : type pair
    QuantifiedVariableDeclarationGroup(QuantifiedVariableDeclarationGroup), // name : type pair
    InteractiveProof(InteractiveProofDisplay), // Interactive proof visualization
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Judgement {
    pub non_quantifiers: Vec<VariableDeclaration>, // non-quantified variables with type or value or constraint info
    pub quantifiers: Vec<QuantifiedVariableDeclarationGroup>,
    pub statement: LogicalNode,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct QuantifierGroup {
    pub quantifier_type: QuantifierType,
    pub variables: Vec<SecondOrderMathNode>, // variable with type or value or constraint info
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct VariableDeclaration {
    pub name: MathNode,
    pub type_info: RichText,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum QuantifiedVariableDeclarationGroup {
    Exists(VariableDeclaration),
    UniqueExists(VariableDeclaration),
    ForAll(Vec<VariableDeclaration>),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum QuantifierType {
    Universal,
    Existential,
    UniqueExistential,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Solution {
    // todo: what do we need to store the space?
    pub solution_space: Vec<crate::turn_render::Section>,
    // pub solution_space_type: SolutionSpaceType,
}

// Interactive Proof Display Containers

/// Display container for interactive proof visualization
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InteractiveProofDisplay {
    pub id: String,
    pub title: String,
    pub proof_forest: ProofForestDisplay,
    pub transformation_data: Vec<ProofTransformationData>,
    pub visual_config: ProofVisualConfig,
    pub interaction_config: ProofInteractionConfig,
}

/// Display container for proof forest visualization
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProofForestDisplay {
    pub forest_id: String,
    pub root_nodes: Vec<ProofNodeDisplay>,
    pub layout_type: ProofLayoutType,
    pub visual_style: ProofVisualStyle,
}

/// Display container for individual proof node
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProofNodeDisplay {
    pub node_id: String,
    pub step_number: usize,
    pub tactic_display: TacticDisplay,
    pub goal_display: GoalDisplay,
    pub transformation_display: Option<TransformationDisplay>,
    pub children: Vec<ProofNodeDisplay>,
    pub visual_state: ProofNodeVisualState,
}

/// Display container for tactic information
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TacticDisplay {
    pub tactic_name: String,
    pub tactic_type: String,
    pub description: String,
    pub interactive_elements: Vec<String>,
    pub workflow_stage: TransformationWorkflowStage,
    pub visual_style: TacticVisualStyle,
}

/// Display container for proof goal
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GoalDisplay {
    pub context_variables: Vec<ContextVariableDisplay>,
    pub goal_statement: MathNode,
    pub visual_style: GoalVisualStyle,
}

/// Display container for context variable
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ContextVariableDisplay {
    pub variable_name: MathNode,
    pub variable_type: RichText,
    pub is_highlighted: bool,
    pub interaction_handlers: Vec<InteractionHandler>,
}

/// Display container for transformation visualization
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TransformationDisplay {
    pub source_expressions: Vec<InteractiveExpression>,
    pub target_expressions: Vec<InteractiveExpression>,
    pub pattern_matches: Vec<PatternMatch>,
    pub instantiations: Vec<InstantiationMap>,
    pub visual_connections: Vec<VisualConnection>,
    pub interactive_elements: Vec<InteractiveElement>,
    pub animation_config: Option<TransformationAnimationConfig>,
}

/// Configuration for proof visualization
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProofVisualConfig {
    pub layout_type: ProofLayoutType,
    pub visual_style: ProofVisualStyle,
    pub animation_enabled: bool,
    pub show_connections: bool,
    pub show_interactive_elements: bool,
    pub color_scheme: ProofColorScheme,
}

/// Configuration for proof interactions
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProofInteractionConfig {
    pub allow_click_interactions: bool,
    pub allow_hover_interactions: bool,
    pub allow_drag_interactions: bool,
    pub allow_selection: bool,
    pub allow_highlighting: bool,
    pub interaction_handlers: Vec<InteractionHandler>,
}

/// Layout types for proof visualization
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ProofLayoutType {
    Tree,
    Linear,
    Graph,
    Timeline,
    OmniOutliner,
}

/// Visual styles for proof elements
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ProofVisualStyle {
    Minimal,
    Standard,
    Detailed,
    Interactive,
    Animated,
}

/// Visual state of a proof node
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ProofNodeVisualState {
    Normal,
    Highlighted,
    Selected,
    Active,
    Completed,
    Error,
}

/// Visual styles for tactics
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum TacticVisualStyle {
    Default,
    Introduction,
    Elimination,
    Structural,
    Completion,
    Automated,
}

/// Visual styles for goals
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum GoalVisualStyle {
    Standard,
    Highlighted,
    Focused,
    Completed,
}

/// Color scheme for proof visualization
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProofColorScheme {
    pub primary_color: String,
    pub secondary_color: String,
    pub accent_color: String,
    pub success_color: String,
    pub error_color: String,
    pub warning_color: String,
    pub info_color: String,
}

/// Animation configuration for transformations
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TransformationAnimationConfig {
    pub duration_ms: u32,
    pub easing: String,
    pub show_progress: bool,
    pub highlight_source: bool,
    pub highlight_target: bool,
    pub show_connections: bool,
}

/// Represents the workflow stages in a proof transformation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum TransformationWorkflowStage {
    Prescribe,   // Prescribing what to transform
    Search,      // Searching for applicable rules/patterns
    Instantiate, // Instantiating variables
    Replace,     // Replacing expressions
    Verify,      // Verifying the transformation
}

/// Represents the complete interactivity data for a proof transformation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProofTransformationData {
    /// The tactic that performed this transformation
    pub tactic_name: String,

    /// The workflow stage this transformation represents
    pub workflow_stage: TransformationWorkflowStage,

    /// Source expressions from the previous proof node
    pub source_expressions: Vec<InteractiveExpression>,

    /// Target expressions in the current proof node
    pub target_expressions: Vec<InteractiveExpression>,

    /// Pattern matching data showing how expressions were matched
    pub pattern_matches: Vec<PatternMatch>,

    /// Instantiation maps showing variable substitutions
    pub instantiations: Vec<InstantiationMap>,

    /// Visual connections between expressions
    pub visual_connections: Vec<VisualConnection>,

    /// Interactive elements that can be highlighted or selected
    pub interactive_elements: Vec<InteractiveElement>,
}

/// Represents a single expression that can be interacted with
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InteractiveExpression {
    pub id: String,
    pub expression: MathNode,
    pub position: ExpressionPosition,
    pub interaction_type: ProofExpressionInteractionType,
    pub metadata: std::collections::HashMap<String, String>,
}

/// Position of an expression within a proof node
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ExpressionPosition {
    pub node_id: String,
    pub context_type: ContextType, // "goal", "hypothesis", "premise", etc.
    pub index: Option<usize>,
    pub path: Vec<String>, // e.g., ["left", "operand", "0"] for nested expressions
}

/// Types of interaction available for expressions
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ProofExpressionInteractionType {
    Highlightable,
    Selectable,
    PatternMatchable,
    Transformable,
    Clickable,
    Draggable,
}

/// Context where an expression appears
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ContextType {
    Goal,
    Hypothesis,
    Premise,
    Axiom,
    Theorem,
    Definition,
    Assumption,
}

/// Represents a pattern match between expressions
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct PatternMatch {
    pub pattern_id: String,
    pub source_expression: String,
    pub matched_expression: String,
    pub confidence: f64,
    pub substitution_map: std::collections::HashMap<String, String>,
}

/// Represents an instantiation map for variable substitution
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InstantiationMap {
    pub variable_name: String,
    pub instantiated_value: MathNode,
    pub source_expression: String,
    pub target_expression: String,
    pub direction: InstantiationDirection,
}

/// Direction of instantiation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum InstantiationDirection {
    Forward,  // From pattern to target
    Backward, // From target to pattern
    Bidirectional,
}

/// Represents an interactive element that can be highlighted or selected
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InteractiveElement {
    pub id: String,
    pub element_type: InteractiveElementType,
    pub expression: Option<MathNode>,
    pub text: Option<String>,
    pub position: ExpressionPosition,
    pub interaction_handlers: Vec<InteractionHandler>,
}

/// Types of interactive elements
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum InteractiveElementType {
    Variable,
    Expression,
    Hypothesis,
    Goal,
    Tactic,
    Axiom,
    Theorem,
    Pattern,
    Instantiation,
}

/// Represents an interaction handler for an element
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InteractionHandler {
    pub handler_type: HandlerType,
    pub action: String,
    pub parameters: std::collections::HashMap<String, String>,
}

/// Types of interaction handlers
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum HandlerType {
    Click,
    Hover,
    Select,
    Drag,
    Transform,
    Highlight,
}

/// Represents a visual connection between elements in a transformation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct VisualConnection {
    pub from: String,
    pub to: String,
    pub connection_type: String,
    pub style: String,
}

/// Trait for converting proof structures to interactive display containers
pub trait ToInteractiveProofDisplay {
    fn to_interactive_proof_display(&self, id_prefix: &str) -> InteractiveProofDisplay;
    fn collect_transformation_data(&self, id_prefix: &str, data: &mut Vec<ProofTransformationData>);
}

/// Trait for converting proof forests to display containers
pub trait ToProofForestDisplay {
    fn to_proof_forest_display(&self, id_prefix: &str) -> ProofForestDisplay;
    fn convert_children_to_display(
        &self,
        parent_id: &str,
        id_prefix: &str,
        step_counter: &mut usize,
    ) -> Vec<ProofNodeDisplay>;
}

/// Trait for converting proof nodes to display containers
pub trait ToProofNodeDisplay {
    fn to_proof_node_display(&self, id_prefix: &str, step_number: usize) -> ProofNodeDisplay;
}

/// Represents the visual flow of a tactic transformation
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TransformationFlow {
    pub tactic_type: String,
    pub direction: String,
    pub source_elements: Vec<String>,
    pub target_elements: Vec<String>,
    pub transformation_type: String,
    pub visual_connections: Vec<VisualConnection>,
}
