use super::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    sync::Arc,
};
use ts_rs::TS;

/// Trait for converting mathematical objects into rich SectionNode structures.
pub trait ToSectionNode {
    /// Converts the object to a Section representation.
    /// - `id_prefix`: A prefix to ensure unique IDs for generated nodes.
    /// Implementers will need to compute/access AbstractionLevel ad-hoc if needed.
    fn to_section_node(&self, id_prefix: &str) -> Section;

    /// Renders the object as a Level 1 (L1) schema section.
    /// This is separate from to_section_node because L1 objects are never instantiated directly.
    /// Implementations should override this for proper L1 schema rendering
    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        // Default implementation uses to_section_node but adds a warning
        // Implementations should override this for proper L1 schema rendering
        let mut section = self.to_section_node(id_prefix);

        // Add warning metadata that this is a default implementation
        if !section.metadata.is_empty() {
            section.metadata.push((
                "warning".to_string(),
                "Default L1 schema rendering used".to_string(),
            ));
        } else {
            section.metadata = vec![(
                "warning".to_string(),
                "Default L1 schema rendering used".to_string(),
            )];
        }

        section
    }

    /// Renders the object as a Level 1 (L1) schema document.
    /// This is separate from to_math_document because L1 objects are never instantiated directly.
    /// Implementers should provide a manual L1 schema representation of their type.
    fn render_as_l1_schema_document(&self, id_prefix: &str) -> MathDocument {
        // Default implementation uses render_as_l1_schema for the main section
        let main_section = self.render_as_l1_schema(&format!("{}-main", id_prefix));

        MathDocument {
            id: format!("{}-l1-doc", id_prefix),
            content_type: MathDocumentType::ScientificPaper(ScientificPaperContent {
                title: main_section.title.as_ref().map_or_else(
                    || "Schema Document".to_string(),
                    |p| {
                        p.segments
                            .iter()
                            .map(|s| match s {
                                RichTextSegment::Text(t) => t.clone(),
                                RichTextSegment::StyledText { text, .. } => text.clone(),
                                _ => "".to_string(),
                            })
                            .collect::<String>()
                    },
                ),
                paper_type: PaperType::Research,
                venue: None,
                peer_reviewed: false,
                content_metadata: ContentMetadata {
                    language: Some("en-US".to_string()),
                    version: Some("1.0".to_string()),
                    created_at: None,
                    last_modified: None,
                    content_hash: None,
                },
                academic_metadata: AcademicMetadata {
                    authors: vec![],
                    date_published: None,
                    date_modified: None,
                    venue: None,
                    doi: None,
                    keywords: vec![],
                },
                structure: DocumentStructure {
                    abstract_content: Some(main_section.clone()),
                    table_of_contents: None,
                    body: vec![main_section],
                    footnotes: vec![],
                    glossary: vec![],
                    bibliography: vec![],
                },
                relationships: DocumentRelationships {
                    parent_documents: vec![],
                    child_documents: vec![],
                    related_concepts: vec![],
                    cross_references: vec![],
                    dependency_graph: None,
                },
            }),
        }
    }
}

// --- Core Building Blocks for Rich Text ---

/// Represents a segment of rich text, allowing for mixed content within paragraphs, list items, etc.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum RichTextSegment {
    Text(String),
    StyledText {
        text: String,
        styles: Vec<TextStyle>, // e.g., bold, italic, color
    },
    Math(MathNode), // Inline mathematical expression
    Link {
        /// The visible content of the link, can be rich text itself.
        content: Vec<RichTextSegment>,
        target: LinkTarget,
        tooltip: Option<String>, // TODO: id of the tooltip page
    },
    FootnoteReference(String), // ID of a footnote
    CodeInline(String),        // For short inline code snippets, e.g., `variable_name`
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum TextStyle {
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Superscript,
    Subscript,
    Color(String), // CSS color string
    BackgroundColor(String),
    FontSize(String), // e.g., "1.2em", "10px"
    FontFamily(String),
}

/// Represents a paragraph of rich text. It doesn't have line breaks
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct RichText {
    pub segments: Vec<RichTextSegment>,
    pub alignment: Option<TextAlignment>,
}

impl RichText {
    pub fn text(text: String) -> RichText {
        RichText {
            segments: vec![RichTextSegment::Text(text)],
            alignment: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}

/// Defines various targets a link can point to, enabling rich interactivity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum LinkTarget {
    Url(String),            // External web URL
    InternalPageId(String), // ID of another MathematicalContent or Section within the system
    DefinitionId {
        // Link to a specific defined term or MathNode concept
        term_id: String,                // Unique ID of the definition or concept
        theory_context: Option<String>, // e.g., "ZFC", "GroupTheory"
    },
    DefinitionAspect {
        // Link to a specific aspect/property of a definition
        term_id: String,   // ID of the main definition (e.g., an L2 group instance)
        aspect_id: String, // Identifier for the property (e.g., "Order", "Commutativity")
        theory_context: Option<String>,
    },
    TheoremId(String), // Link to a specific Theorem, Lemma, etc.
    ObjectConstructorTemplate {
        // A page/section acting as a template for creating math objects
        template_id: String, // ID of the page/section that is the template
        /// Pre-filled parameters for the template, MathNode can represent concrete values or variables.
        parameters: Vec<(String, MathNode)>,
        /// Indicates the desired abstraction level (L1-L4) for the constructed object.
        target_abstraction_level: Option<u8>,
    },
    GlossaryTerm(String),               // Link to a term in a glossary
    BibliographyKey(String),            // Link to a bibliography entry
    InteractiveElementId(String), // Link to trigger/focus an interactive component on the page
    TooltipDocument(Arc<MathDocument>), // NEW: Embedded tooltip document
    AnimationTrigger {
        // NEW: Trigger for animations
        animation_id: String,
        trigger_type: AnimationTriggerType,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum AnimationTriggerType {
    Click,
    Hover,
    Toggle,
    Sequence,
}

// --- High-Level Structural Content Nodes ---

/// Enum representing the different types of content blocks that can appear in a section.
/// This is the primary building block for document content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum SectionContentNode {
    // New variant for subsections
    SubSection(Arc<Section>), // Box to avoid recursive type definition issues
    // non-recursive content nodes
    RichText(RichText),
    MathNode {
        // For display-style math equations (block-level)
        math: MathNode,
        label: Option<String>, // For equation numbering/referencing
        caption: Option<RichText>,
    },
    StructuredMath(StructuredMathNode), // Definitions, Theorems, Proofs, etc.

    List(ListNode),
    Table(TableNode),
    CodeBlock(CodeBlockNode),
    Image(ImageNode),
    InteractiveDiagram(InteractiveDiagramNode), // More generic than Visualization
    CollapsibleBlock(CollapsibleBlockNode),
    Grid(GridNode),
    Columns(ColumnsNode),
    ThematicBreak(ThematicBreakNode), // Horizontal rule
    QuoteBlock {
        content: Vec<RichText>,
        attribution: Option<RichText>,
    },
    AlertBox {
        // For notes, warnings, tips
        style: AlertBoxStyle,
        content: Vec<SectionContentNode>, // Can contain other blocks
    },
    // Placeholder for more complex or custom components
    CustomComponent {
        component_name: String, // Identifier for a specific React/WASM component
        props: Option<String>,  // Properties to pass to the component
        fallback_content: Vec<SectionContentNode>, // Content to show if component fails
    },
    // Embeds another section, useful for transclusion or master documents.
    EmbeddedSectionRef(String), // ID of another SectionNode to embed

    // NEW: Enhanced layout and interaction types
    SideBySideLayout(SideBySideLayout), // For comparison pages, transformation mappings
    PanelLayout(PanelLayout),           // For resource panels, multi-panel displays
    AnnotationOverlay(AnnotationOverlay), // For type mappings, explanatory overlays
    InteractiveControls(InteractiveControls), // For playgrounds with parameter controls
    EmbeddedDocument(Arc<MathDocument>), // For nested documents, tooltips
}

// --- NEW: Enhanced Layout Types ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SideBySideLayout {
    pub left_panel: Panel,
    pub right_panel: Panel,
    pub sync_scrolling: Option<bool>,
    pub highlight_correspondence: Option<bool>, // For synchronized highlighting
    pub layout_config: Option<SideBySideConfig>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SideBySideConfig {
    pub left_width: Option<String>, // e.g., "50%", "300px"
    pub right_width: Option<String>,
    pub gap: Option<String>,
    pub responsive_breakpoint: Option<String>, // Switch to vertical on small screens
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct PanelLayout {
    pub panels: Vec<Panel>,
    pub layout_type: PanelLayoutType,
    pub panel_controls: Option<PanelControls>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum PanelLayoutType {
    Tabs,
    Accordion,
    Grid { columns: usize },
    Sidebar { main_panel_id: String },
    FloatingPanels,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Panel {
    pub id: String,
    pub title: Option<RichText>,
    pub content: Vec<SectionContentNode>,
    pub panel_role: PanelRole,
    pub initially_visible: Option<bool>,
    pub resizable: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum PanelRole {
    MainContent,
    ComparisonLeft,
    ComparisonRight,
    SourceTheory,
    TargetTheory,
    TypeAnnotations,
    ResourceBank,
    Navigation,
    ControlPanel,
    InfoBox,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct PanelControls {
    pub allow_minimize: Option<bool>,
    pub allow_close: Option<bool>,
    pub allow_reorder: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AnnotationOverlay {
    pub base_content: Vec<SectionContentNode>,
    pub annotations: Vec<Annotation>,
    pub overlay_style: OverlayStyle,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Annotation {
    pub id: String,
    pub target_selector: String, // CSS selector or element ID to target
    pub annotation_content: Vec<RichTextSegment>,
    pub annotation_type: AnnotationType,
    pub position: Option<AnnotationPosition>,
    pub styling: Option<AnnotationStyling>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum AnnotationType {
    TypeInfo,
    Definition,
    Explanation,
    Animation,
    Highlight,
    Warning,
    Step,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum OverlayStyle {
    Tooltip,
    Popover,
    Inline,
    Sidebar,
    Highlight,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AnnotationPosition {
    pub x: f64,
    pub y: f64,
    pub anchor: PositionAnchor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum PositionAnchor {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AnnotationStyling {
    pub color: Option<String>,
    pub background_color: Option<String>,
    pub border_color: Option<String>,
    pub opacity: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InteractiveControls {
    pub controls: Vec<Control>,
    pub target_content_ids: Vec<String>, // IDs of content that these controls affect
    pub layout: ControlLayout,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ControlLayout {
    Horizontal,
    Vertical,
    Grid { columns: usize },
    Floating,
}

/// Interactive control element
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Control {
    pub id: String,
    pub label: Option<RichText>,
    pub control_type: ControlType,
    pub default_value: Option<String>,
    pub validation_rules: Vec<String>,
}

/// Types of interactive controls
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ControlType {
    Slider { min: f64, max: f64, step: f64 },
    TextInput { placeholder: Option<String> },
    NumberInput { min: Option<f64>, max: Option<f64> },
    Checkbox,
    RadioGroup { options: Vec<String> },
    Dropdown { options: Vec<String> },
    Button { action: String },
}

// --- Structured Mathematical Content Types ---

/// Represents formal mathematical structures like definitions, theorems, etc.
/// This matches the TypeScript StructuredMathNode bindings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum StructuredMathNode {
    Definition {
        term_display: RichText,
        formal_term: Option<MathNode>,
        label: Option<String>,
        body: Vec<SectionContentNode>,
        abstraction_meta: Option<AbstractionMetadata>,
        selectable_properties: Vec<SelectableProperty>,
    },
    TheoremLike {
        kind: TheoremLikeKind,
        label: Option<String>,
        statement: TheoremStatement,
        proof: Option<ProofDisplayNode>,
        abstraction_meta: Option<AbstractionMetadata>,
    },
    Example {
        label: Option<String>,
        introduction: Vec<SectionContentNode>,
        body: Vec<SectionContentNode>,
        explanation: Vec<SectionContentNode>,
    },
    Remark {
        label: Option<String>,
        body: Vec<SectionContentNode>,
    },
    Axiom {
        label: Option<String>,
        statement: Vec<SectionContentNode>,
        abstraction_meta: Option<AbstractionMetadata>,
    },
    Exercise {
        label: Option<String>,
        problem_statement: Vec<SectionContentNode>,
        hints: Vec<CollapsibleBlockNode>,
        solution: Option<Arc<CollapsibleBlockNode>>,
    },
    ConstructorDefinition {
        title_display: Vec<RichTextSegment>,
        label: Option<String>,
        body: Vec<SectionContentNode>,
        formal_parameters: Vec<(String, Vec<RichTextSegment>)>,
        return_type_summary: Vec<RichTextSegment>,
        return_type_link: Option<LinkTarget>,
        abstraction_meta: Option<AbstractionMetadata>,
    },
    CollectionView {
        collection_type: String,
        description: RichText,
        variants: Vec<(String, String)>,
        variant_links: Vec<LinkTarget>,
        abstraction_meta: Option<AbstractionMetadata>,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AbstractionMetadata {
    /// Abstraction level (L1-L4) as per theory_and_render.md
    pub level: Option<u8>,
    /// Link to the L1/L2 blueprint or source template for this object/definition.
    pub source_template_id: Option<String>,
    /// For L2/L3/L4, parameters that have been specified or concretized.
    pub specified_parameters: Vec<(String, MathNode)>,
    /// For L2, properties that are universally quantified (or "any valid option").
    pub universally_quantified_properties: Vec<String>,
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
}

// --- Proof Display Structures ---

/// Represents different ways a theorem statement can be expressed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum TheoremStatement {
    Content(Vec<SectionContentNode>),
    Mathematical(MathNode),
}

/// Represents a display-oriented proof structure (for frontend compatibility)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProofDisplayNode {
    pub title: Option<RichText>,
    pub strategy: Vec<SectionContentNode>,
    pub steps: Vec<ProofStepNode>,
    pub qed_symbol: Option<String>,
}

/// Represents a single step or block within a proof's display
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ProofStepNode {
    // Legacy simple statement format
    Statement {
        claim: Vec<RichTextSegment>,
        justification: Vec<RichTextSegment>,
    },

    // Rich tactic-based proof steps
    TacticApplication(TacticDisplayNode),

    // Structural proof elements
    Elaboration(Vec<SectionContentNode>),
    CaseAnalysis {
        introduction: Option<RichText>,
        cases: Vec<ProofCaseNode>,
    },
    InductiveProof {
        variable_of_induction: MathNode,
        base_case: ProofDisplayNode,
        inductive_hypothesis: RichText,
        inductive_step: ProofDisplayNode,
    },
    Assume(RichText),
    Goal(RichText),
    NestedProof(ProofDisplayNode),
}

/// Represents a case in case analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProofCaseNode {
    pub condition: RichText,
    pub proof_for_case: ProofDisplayNode,
}

/// Rich display representation of tactic applications
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum TacticDisplayNode {
    // ========== QUANTIFIER TACTICS ==========
    IntroduceQuantifier {
        object_description: RichText,
        position: Option<usize>,
        before_state: Option<RichText>,
        after_state: Option<RichText>,
    },

    IntroduceFreshVariable {
        target_quantifier: RichText,
        fresh_variable_name: RichText,
        explanation: RichText,
        mathematical_context: Option<RichText>,
    },

    ProvideWitness {
        target_quantifier: RichText,
        witness_expression: MathNode,
        witness_explanation: RichText,
        verification_steps: Vec<SectionContentNode>,
    },

    ReorderQuantifiers {
        original_order: Vec<RichText>,
        new_order: Vec<RichText>,
        justification: RichText,
    },

    UniversalCaseAnalysis {
        target_quantifier: RichText,
        cases: Vec<CaseDisplayNode>,
        exhaustiveness_proof: Option<RichText>,
    },

    // ========== VALUE VARIABLE TACTICS ==========
    IntroduceValueVariable {
        variable_name: RichText,
        variable_value: MathNode,
        binding_type: VariableBindingType,
        context_explanation: RichText,
        position: Option<usize>,
    },

    SubstituteValueVariable {
        target_variable: RichText,
        substitution_preview: SubstitutionPreview,
        justification: RichText,
    },

    RewriteInValueBinding {
        target_variable: RichText,
        target_subexpression: MathNode,
        replacement: MathNode,
        justification: Vec<SectionContentNode>,
        step_by_step: Vec<RewriteStep>,
    },

    RemoveValueVariable {
        target_variable: RichText,
        reason: RichText,
        cleanup_explanation: Option<RichText>,
    },

    // ========== STATEMENT TACTICS ==========
    ExactWith {
        theorem_name: RichText,
        theorem_statement: RichText,
        instantiation_mapping: Vec<InstantiationPair>,
        match_verification: MatchVerification,
        theorem_link: Option<LinkTarget>,
    },

    Rewrite {
        target_expression: MathNode,
        theorem_name: RichText,
        theorem_rule: RichText,
        instantiation_mapping: Vec<InstantiationPair>,
        direction: RewriteDirectionDisplay,
        step_by_step_transformation: Vec<RewriteStep>,
        theorem_link: Option<LinkTarget>,
    },

    AssumeImplicationAntecedent {
        implication_statement: MathNode,
        hypothesis_name: RichText,
        antecedent: MathNode,
        consequent: MathNode,
        context_explanation: RichText,
    },

    SplitConjunction {
        target_conjunction: MathNode,
        conjuncts: Vec<MathNode>,
        selected_index: usize,
        remaining_goals: Vec<MathNode>,
    },

    SplitDisjunction {
        target_disjunction: MathNode,
        disjuncts: Vec<MathNode>,
        chosen_index: usize,
        chosen_disjunct: MathNode,
        strategy_explanation: RichText,
    },

    StatementCaseAnalysis {
        target_expression: MathNode,
        cases: Vec<CaseDisplayNode>,
        exhaustiveness_proof: Option<RichText>,
    },

    Simplify {
        target_path: Vec<usize>,
        original_expression: MathNode,
        simplified_expression: MathNode,
        simplification_steps: Vec<SimplificationStep>,
        rules_used: Vec<RichText>,
    },

    // ========== META TACTICS ==========
    Auto {
        automated_tactic_type: AutomatedTacticDisplay,
        search_depth: Option<u8>,
        tactics_attempted: Vec<RichText>,
        successful_path: Option<Vec<RichText>>,
        execution_summary: RichText,
    },

    Induction {
        target_relation: MathNode,
        induction_variable: RichText,
        base_case_value: MathNode,
        base_case_proof: ProofDisplayNode,
        inductive_hypothesis: RichText,
        inductive_step_proof: ProofDisplayNode,
        induction_principle: RichText,
    },
}

/// Display representation of case analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CaseDisplayNode {
    pub case_name: RichText,
    pub condition: MathNode,
    pub values: Vec<MathNode>,
    pub case_explanation: RichText,
    pub proof_outline: Option<Vec<SectionContentNode>>,
}

/// Display representation of variable binding types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum VariableBindingType {
    Assumption,
    Definition,
    Hypothesis,
    Given,
    Let,
}

/// Display representation of substitution preview
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SubstitutionPreview {
    pub before: MathNode,
    pub after: MathNode,
    pub highlighted_changes: Vec<SubstitutionHighlight>,
}

/// Highlight information for substitutions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SubstitutionHighlight {
    pub path: Vec<usize>,
    pub original_text: RichText,
    pub replacement_text: RichText,
    pub explanation: Option<RichText>,
}

/// Display representation of rewrite steps
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct RewriteStep {
    pub step_number: usize,
    pub before: MathNode,
    pub after: MathNode,
    pub rule_applied: RichText,
    pub explanation: RichText,
    pub highlighted_region: Option<Vec<usize>>,
}

/// Display representation of instantiation pairs
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InstantiationPair {
    pub variable_name: RichText,
    pub variable_value: MathNode,
    pub type_information: Option<RichText>,
}

/// Display representation of match verification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct MatchVerification {
    pub pattern: MathNode,
    pub target: MathNode,
    pub match_success: bool,
    pub match_explanation: RichText,
    pub unification_details: Vec<UnificationDetail>,
}

/// Details of unification in pattern matching
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UnificationDetail {
    pub pattern_part: MathNode,
    pub target_part: MathNode,
    pub unification_type: UnificationType,
    pub explanation: RichText,
}

/// Types of unification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum UnificationType {
    ExactMatch,
    VariableBinding,
    StructuralMatch,
    TypeCoercion,
}

/// Display representation of rewrite direction
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum RewriteDirectionDisplay {
    LeftToRight {
        left_side: MathNode,
        right_side: MathNode,
        explanation: RichText,
    },
    RightToLeft {
        left_side: MathNode,
        right_side: MathNode,
        explanation: RichText,
    },
}

/// Display representation of simplification steps
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SimplificationStep {
    pub step_number: usize,
    pub before: MathNode,
    pub after: MathNode,
    pub rule_name: RichText,
    pub rule_explanation: RichText,
    pub algebraic_justification: Option<RichText>,
}

/// Display representation of automated tactics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum AutomatedTacticDisplay {
    FindProof {
        target_relation: MathNode,
        search_strategy: RichText,
        theorems_considered: Vec<TheoremReference>,
        proof_found: Option<ProofDisplayNode>,
    },
    Simplify {
        target_expression: MathNode,
        simplification_rules: Vec<RichText>,
        final_result: MathNode,
    },
    ByAssumption {
        matching_assumption: MathNode,
        assumption_name: RichText,
        match_explanation: RichText,
    },
    Contradiction {
        contradictory_statements: Vec<MathNode>,
        contradiction_explanation: RichText,
        ex_falso_principle: RichText,
    },
    Auto {
        search_tree: Option<SearchTreeDisplay>,
        successful_tactics: Vec<RichText>,
        failed_attempts: Vec<FailedAttempt>,
    },
}

/// Reference to a theorem
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TheoremReference {
    pub theorem_id: String,
    pub theorem_name: RichText,
    pub theorem_statement: RichText,
    pub relevance_score: Option<f64>,
    pub link: Option<LinkTarget>,
}

/// Display representation of search tree for automated tactics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SearchTreeDisplay {
    pub root_goal: MathNode,
    pub search_nodes: Vec<SearchNodeDisplay>,
    pub successful_path: Vec<usize>,
    pub max_depth_reached: usize,
}

/// Individual node in search tree
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SearchNodeDisplay {
    pub node_id: usize,
    pub parent_id: Option<usize>,
    pub goal_state: MathNode,
    pub tactic_applied: Option<RichText>,
    pub success: bool,
    pub children: Vec<usize>,
}

/// Failed attempt in automated search
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct FailedAttempt {
    pub tactic_name: RichText,
    pub failure_reason: RichText,
    pub goal_state: MathNode,
    pub error_details: Option<RichText>,
}

/// Represents a display-oriented proof structure with tree-like branching.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProofNodeDisplay {
    // pub title: Option<RichText>,   // e.g., "Proof.", "Proof of Theorem 1.2."
    // pub strategy: Option<Section>, // Optional outline of the proof strategy
    pub step: ProofStepDisplay,
    /// Child proof branches - for representing proof tree structure
    pub children: Vec<ProofNodeDisplay>,
}

pub trait ToProofDisplay {
    /// Convert this object to a proof display node (frontend compatible)
    fn to_proof_display(&self) -> ProofDisplayNode;

    /// Convert this object to a vector of proof display nodes (for multiple proofs)
    fn to_proof_display_vec(&self) -> Vec<ProofDisplayNode> {
        vec![self.to_proof_display()]
    }
}

pub trait ToProofStep {
    fn to_proof_step(&self) -> ProofStepNode;
}

/// Represents a single step or block within a proof's display.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ProofStepDisplay {
    /// A tactic application showing what changed in the ProofGoal
    Tactic {
        /// The tactic that was applied
        tactic_name: String,
        /// Parameters of the tactic (if any)
        parameters: Vec<String>,
        /// Human-readable description of what happened
        description: Section,
        /// Before/after state (optional for interactivity)
        state_change: Option<ProofGoalChange>,
    },
    /// Traditional statement-style step
    Statement {
        claim: Section,
        justification: Section,
    },
    /// Case analysis with branches
    CaseAnalysis {
        introduction: Option<Section>,
        cases: Vec<Section>,
    },
    /// Nested proof structure
    NestedProof(Arc<ProofNodeDisplay>),
}

/// Simple representation of what changed in a ProofGoal after a tactic
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProofGoalChange {
    /// Names of quantifiers that were added/removed/modified
    pub quantifier_changes: Vec<String>,
    /// Names of value variables that were added/removed/modified  
    pub variable_changes: Vec<String>,
    /// Whether the main statement changed
    pub statement_changed: bool,
    /// Summary of the change
    pub summary: String,
}

// --- Layout and Utility Content Types ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ListNode {
    pub items: Vec<ListItemNode>,
    pub style: ListStyle,
    pub start_index: Option<i32>, // For ordered lists
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ListItemNode {
    /// Content of a list item can be complex, allowing nested structures.
    pub content: Vec<SectionContentNode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ListStyle {
    Unordered(UnorderedListStyle),
    Ordered(OrderedListStyle),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum UnorderedListStyle {
    Disc, // default bullet
    Circle,
    Square,
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum OrderedListStyle {
    Decimal,    // 1, 2, 3
    AlphaLower, // a, b, c
    AlphaUpper, // A, B, C
    RomanLower, // i, ii, iii
    RomanUpper, // I, II, III
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TableNode {
    pub caption: Option<RichText>,
    pub header_rows: Vec<TableRowNode>,
    pub body_rows: Vec<TableRowNode>,
    pub footer_rows: Vec<TableRowNode>,
    pub column_styles: Vec<ColumnStyle>,
    pub table_style_options: Option<TableStyleOptions>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TableRowNode {
    pub cells: Vec<TableCellNode>,
    // pub style: Option<RowStyle>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TableCellNode {
    pub content: Vec<SectionContentNode>,
    pub col_span: Option<usize>,
    pub row_span: Option<usize>,
    pub cell_type: TableCellType,
    pub alignment: Option<TextAlignment>,
    // pub style: Option<CellStyle>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum TableCellType {
    Header,
    Data,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ColumnStyle {
    pub width: Option<String>, // e.g., "20%", "100px"
    pub alignment: Option<TextAlignment>,
    // Add other column-specific styles
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TableStyleOptions {
    pub borders: Option<bool>, // Show all borders
    pub striped_rows: Option<bool>,
    // Add other table-wide styles
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CodeBlockNode {
    pub code: String,
    pub language: Option<String>, // e.g., "rust", "python", "latex", "lean", "plaintext"
    pub caption: Option<RichText>,
    pub show_line_numbers: Option<bool>,
    pub highlight_lines: Vec<usize>,
    pub is_executable: Option<bool>, // For interactive code blocks
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ImageNode {
    pub src: String, // URL or path
    pub alt_text: Option<String>,
    pub caption: Option<RichText>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub alignment: Option<HorizontalAlignment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InteractiveDiagramNode {
    pub diagram_type_id: String, // Identifier for the type of diagram (e.g., "commutative_diagram", "function_plot")
    pub data: String,            // Diagram-specific data
    pub caption: Option<RichText>,
    pub config_options: Option<String>, // UI options for the diagram
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CollapsibleBlockNode {
    pub summary: Vec<RichTextSegment>, // The visible part when collapsed (clickable)
    pub details: Vec<SectionContentNode>, // The content shown when expanded
    pub initially_collapsed: Option<bool>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GridNode {
    pub items: Vec<GridItemNode>,
    /// Number of columns, or CSS grid-template-columns string.
    pub column_template: String, // e.g., "3" for 3 equal columns, or "1fr 2fr"
    pub row_gap: Option<String>, // e.g., "10px"
    pub column_gap: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GridItemNode {
    pub content: SectionContentNode,
    pub col_start: Option<usize>,
    pub col_end: Option<usize>, // Or col_span
    pub row_start: Option<usize>,
    pub row_end: Option<usize>, // Or row_span
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ColumnsNode {
    pub columns_content: Vec<Vec<SectionContentNode>>, // Each inner Vec is a column
    pub column_widths: Vec<String>,                    // e.g., ["30%", "70%"] or ["1fr", "2fr"]
    pub gap: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ThematicBreakNode;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum AlertBoxStyle {
    Information,
    Success,
    Warning,
    Error,
    Note,
    Tip,
}

// --- Core Document Structure Types ---

/// A `SectionNode` represents a major, navigable part of a document (like a chapter or a named section).
/// It can have a title and contains various content blocks. Sections can be nested.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Section {
    // Renamed from SectionNode to avoid confusion with enum SectionContentNode
    pub id: String,              // Unique ID for linking, navigation, and referencing
    pub title: Option<RichText>, // The title of the section
    pub content: Vec<SectionContentNode>, // Ordered list of content blocks within this section
    pub metadata: Vec<(String, String)>, // For tags, abstraction level, visibility, etc.
    pub display_options: Option<SectionDisplayOptions>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SectionDisplayOptions {
    pub show_title_numbering: Option<bool>,
    // Add other display-related options
}

/// Represents a property that can have selectable variants in the UI.
#[derive(Debug, Clone, Serialize, Deserialize, TS, PartialEq, Eq, Hash)]
#[ts(export)]
pub struct SelectableProperty {
    pub name: String,
    pub current_variant: String,
    pub all_variants: Vec<String>,
    pub description: Option<String>,
    pub variant_descriptions: Option<BTreeMap<String, String>>,
    pub property_type_def_id: Option<String>,
}

// --- NEW: Structured Proof Content Types ---

/// Represents a quantified mathematical object in structured form
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct QuantifiedObject {
    pub variable: String,
    pub quantification: QuantifierType,
    pub object_type: String, // Could be expanded to a full type system
    pub constraints: Vec<MathNode>,
    pub description: Option<String>,
}

/// Types of quantification
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum QuantifierType {
    Universal,         // ∀
    Existential,       // ∃
    UniqueExistential, // ∃!
    Defined,           // defined in terms of
    Fixed,             // arbitrary but fixed
}

/// Represents a variable binding in structured form
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct VariableBinding {
    pub variable_name: String,
    pub value: MathNode,
    pub binding_type: BindingType,
}

/// Types of variable bindings
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum BindingType {
    Assumption,
    Definition,
    Hypothesis,
    Given,
    Let,
}

/// Types of inequality relations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum InequalityType {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    NotEqual,
}

/// Types of numbers
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum NumberType {
    Integer,
    Rational,
    Real,
    Complex,
    Natural,
}

/// Types of mathematical operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum OperationType {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Power,
    Inverse,
    Composition,
    GroupOperation,
    SetUnion,
    SetIntersection,
    SetDifference,
}

/// Types of sets
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum SetType {
    Explicit, // {1, 2, 3}
    Implicit, // {x | P(x)}
    Standard, // ℕ, ℤ, ℝ, etc.
    Empty,
}
