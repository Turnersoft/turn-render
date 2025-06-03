use super::*;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use ts_rs::TS;

/// Trait for converting mathematical objects into rich SectionNode structures.
pub trait ToSectionNode {
    /// Converts the object to a Section representation.
    /// - `id_prefix`: A prefix to ensure unique IDs for generated nodes.
    /// Implementers will need to compute/access AbstractionLevel ad-hoc if needed.
    fn to_section_node(&self, id_prefix: &str) -> Section;

    /// Generates a full document representation.
    /// Implementers are responsible for determining the appropriate AbstractionLevel for the main section.
    fn to_math_document(&self, id_prefix: &str) -> MathDocument;

    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment>;
    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment>;

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
        tooltip: Option<String>,
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

/// Represents a paragraph of rich text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ParagraphNode {
    pub segments: Vec<RichTextSegment>,
    pub alignment: Option<TextAlignment>,
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
    TooltipDocument(Box<MathDocument>), // NEW: Embedded tooltip document
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
    Paragraph(ParagraphNode),
    MathNode {
        // For display-style math equations (block-level)
        math: MathNode,
        label: Option<String>, // For equation numbering/referencing
        caption: Option<ParagraphNode>,
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
        content: Vec<ParagraphNode>,
        attribution: Option<ParagraphNode>,
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
    // New variant for subsections
    SubSection(Box<Section>), // Box to avoid recursive type definition issues

    // NEW: Enhanced layout and interaction types
    SideBySideLayout(SideBySideLayout), // For comparison pages, transformation mappings
    PanelLayout(PanelLayout),           // For resource panels, multi-panel displays
    AnnotationOverlay(AnnotationOverlay), // For type mappings, explanatory overlays
    InteractiveControls(InteractiveControls), // For playgrounds with parameter controls
    EmbeddedDocument(Box<MathDocument>), // For nested documents, tooltips
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
    pub title: Option<ParagraphNode>,
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

// --- Structured Mathematical Content Types ---

/// Represents formal mathematical structures like definitions, theorems, etc.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum StructuredMathNode {
    Definition {
        term_display: Vec<RichTextSegment>,
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
        solution: Option<Box<CollapsibleBlockNode>>,
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
        description: ParagraphNode,
        variants: Vec<(String, String)>,
        variant_links: Vec<LinkTarget>,
        abstraction_meta: Option<AbstractionMetadata>,
    },
}

/// Represents different ways a theorem statement can be expressed
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum TheoremStatement {
    /// Traditional content-based statement
    Content(Vec<SectionContentNode>),
    /// Structured mathematical statement
    Mathematical(MathNode),
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

/// Represents a display-oriented proof structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProofDisplayNode {
    pub title: Option<ParagraphNode>, // e.g., "Proof.", "Proof of Theorem 1.2."
    pub strategy: Vec<SectionContentNode>, // Optional outline of the proof strategy
    pub steps: Vec<ProofStepNode>,
    pub qed_symbol: Option<String>, // e.g., "□", "∎", or "" for none
}

/// Represents a single step or block within a proof's display.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ProofStepNode {
    Statement {
        claim: Vec<RichTextSegment>, // The mathematical claim or derivation for this step
        justification: Vec<RichTextSegment>, // e.g., "by Definition 1", "from (3.2)"
    },
    Elaboration(Vec<SectionContentNode>), // A more detailed explanation or sub-derivation
    CaseAnalysis {
        introduction: Option<ParagraphNode>,
        cases: Vec<ProofCaseNode>,
    },
    InductiveProof {
        variable_of_induction: MathNode,
        base_case: Box<ProofDisplayNode>,
        inductive_hypothesis: ParagraphNode, // Statement of P(k)
        inductive_step: Box<ProofDisplayNode>, // Proof of P(k) => P(k+1)
    },
    Assume(ParagraphNode), // For assumptions in direct proofs or contradiction
    Goal(ParagraphNode),   // Stating a subgoal
    NestedProof(Box<ProofDisplayNode>), // For a sub-lemma or smaller part proven inline
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProofCaseNode {
    pub condition: ParagraphNode, // e.g., "Case 1: n is even."
    pub proof_for_case: ProofDisplayNode,
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
    pub caption: Option<ParagraphNode>,
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
    pub caption: Option<ParagraphNode>,
    pub show_line_numbers: Option<bool>,
    pub highlight_lines: Vec<usize>,
    pub is_executable: Option<bool>, // For interactive code blocks
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ImageNode {
    pub src: String, // URL or path
    pub alt_text: Option<String>,
    pub caption: Option<ParagraphNode>,
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
    pub caption: Option<ParagraphNode>,
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
    pub id: String, // Unique ID for linking, navigation, and referencing
    pub title: Option<ParagraphNode>, // The title of the section
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

/// Represents a structured proof goal with proper mathematical formatting
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Goal {
    pub quantified_objects: Vec<QuantifiedObject>,
    pub variable_bindings: Vec<VariableBinding>,
    pub statement: MathNode,
    pub goal_type: GoalType,
}

/// Represents a quantified mathematical object in structured form
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct QuantifiedObject {
    pub variable: String,
    pub quantifier_type: QuantifierType,
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

/// Types of proof goals
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum GoalType {
    Prove,
    Disprove,
    Show,
    Establish,
    Verify,
    Construct,
}

/// Represents a structured proof tactic
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum Tactic {
    Introduction {
        variable: String,
        assumption: Option<MathNode>,
    },
    Elimination {
        target: MathNode,
        method: EliminationMethod,
    },
    Substitution {
        target: MathNode,
        replacement: MathNode,
        location: Option<Vec<usize>>,
    },
    TheoremApplication {
        theorem_name: String,
        instantiation: Vec<(String, MathNode)>,
        target: Option<MathNode>,
    },
    CaseAnalysis {
        cases: Vec<Case>,
    },
    Induction {
        variable: String,
        base_case: Goal,
        inductive_step: Goal,
    },
    Contradiction {
        assumption: MathNode,
    },
    DirectProof,
    Custom {
        name: String,
        description: String,
        arguments: Vec<String>,
    },
}

/// Elimination methods for logical rules
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum EliminationMethod {
    ModusPonens,
    ModusTollens,
    DisjunctiveSyllogism,
    UniversalInstantiation,
    ExistentialInstantiation,
}

/// Represents a case in case analysis
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Case {
    pub condition: MathNode,
    pub proof: Goal,
    pub case_name: Option<String>,
}

/// Status of proof steps
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ProofStepStatus {
    Complete,
    InProgress,
    Todo,
    WorkInProgress,
    Abandoned,
}

/// Structured proof case
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProofCase {
    pub condition: MathNode,
    pub proof: ProofDisplayNode,
    pub case_name: Option<String>,
}
