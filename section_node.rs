use super::{
    MathNodeContent,
    math_node::{IdentifierNode, MathNode, RelationOperatorNode, ScriptNode},
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use ts_rs::TS;

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
    GlossaryTerm(String),                      // Link to a term in a glossary
    BibliographyKey(String),                   // Link to a bibliography entry
    InteractiveElementId(String), // Link to trigger/focus an interactive component on the page
    TooltipDocument(Box<MathematicalContent>), // NEW: Embedded tooltip document
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
    EmbeddedDocument(Box<MathematicalContent>), // For nested documents, tooltips
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
pub struct Control {
    pub id: String,
    pub label: String,
    pub control_type: ControlType,
    pub parameter_name: String,
    pub default_value: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ControlType {
    Slider { min: f64, max: f64, step: f64 },
    Toggle,
    Dropdown { options: Vec<String> },
    NumberInput { min: Option<f64>, max: Option<f64> },
    ColorPicker,
    Button { action: String },
    RadioGroup { options: Vec<String> },
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

// --- MAIN: Mathematical Content System ---

/// The main container for mathematical content with a unique ID and content type
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct MathematicalContent {
    pub id: String,
    pub content_type: MathematicalContentType,
}

/// Each variant represents a distinct document type with its own specialized structure and behavior
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum MathematicalContentType {
    // --- Primary Knowledge Documents ---
    WikiPage(WikiPageContent),
    Textbook(TextbookContent),
    ScientificPaper(ScientificPaperContent),
    PersonalNotes(PersonalNotesContent),
    MathematicianNotes(MathematicianNotesContent),
    StudyNotes(StudyNotesContent),

    // --- Derived/Simplified Content ---
    TooltipSummary(TooltipSummaryContent),
    BlogPost(BlogPostContent),
    AbstractSummary(AbstractSummaryContent),
    ConceptMap(ConceptMapContent),

    // --- Interactive/Dynamic Content ---
    AnimatedPresentation(AnimatedPresentationContent),
    InteractivePlayground(InteractivePlaygroundContent),
    TypeMappingDisplay(TypeMappingDisplayContent),
    ResourcePanel(ResourcePanelContent),

    // --- Relational/Comparison Content ---
    ComparisonPage(ComparisonPageContent),
    TransformationMapping(TransformationMappingContent),
    ConceptAlignment(ConceptAlignmentContent),

    // --- Embedded/Preview Content ---
    StaticPreview(StaticPreviewContent),
    LiveEmbed(LiveEmbedContent),
    ConceptExtract(ConceptExtractContent),
    IFrameEmbed(IFrameEmbedContent),
}

// --- Primary Knowledge Document Structs ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct WikiPageContent {
    pub title: String,
    pub theory_domain: String,
    pub completeness_level: CompletenessLevel,
    pub maintainer: Option<String>,
    pub content_metadata: ContentMetadata,
    pub structure: DocumentStructure,
    pub relationships: DocumentRelationships,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TextbookContent {
    pub title: String,
    pub course_level: CourseLevel,
    pub chapter_info: Option<ChapterInfo>,
    pub prerequisites: Vec<String>,
    pub content_metadata: ContentMetadata,
    pub academic_metadata: AcademicMetadata,
    pub structure: DocumentStructure,
    pub relationships: DocumentRelationships,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ScientificPaperContent {
    pub title: String,
    pub paper_type: PaperType,
    pub venue: Option<String>,
    pub peer_reviewed: bool,
    pub content_metadata: ContentMetadata,
    pub academic_metadata: AcademicMetadata,
    pub structure: DocumentStructure,
    pub relationships: DocumentRelationships,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct PersonalNotesContent {
    pub title: String,
    pub author_level: AudienceLevel,
    pub note_style: NoteStyle,
    pub content_metadata: ContentMetadata,
    pub structure: DocumentStructure,
    pub relationships: DocumentRelationships,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct MathematicianNotesContent {
    pub title: String,
    pub research_area: String,
    pub formality_level: FormalityLevel,
    pub content_metadata: ContentMetadata,
    pub structure: DocumentStructure,
    pub relationships: DocumentRelationships,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct StudyNotesContent {
    pub title: String,
    pub subject: String,
    pub study_level: CourseLevel,
    pub exam_prep: bool,
    pub content_metadata: ContentMetadata,
    pub structure: DocumentStructure,
    pub relationships: DocumentRelationships,
}

// --- Derived/Simplified Content Structs ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TooltipSummaryContent {
    pub summarization_level: SummarizationLevel,
    pub max_length: Option<usize>,
    pub focus_concepts: Vec<String>,
    pub source_references: Vec<SourceReference>,
    pub derivation_metadata: DerivationMetadata,
    pub content: SimplifiedContentStructure,
    pub presentation_config: PresentationConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct BlogPostContent {
    pub title: String,
    pub writing_style: WritingStyle,
    pub target_audience: AudienceLevel,
    pub examples_included: bool,
    pub source_references: Vec<SourceReference>,
    pub derivation_metadata: DerivationMetadata,
    pub content: SimplifiedContentStructure,
    pub presentation_config: PresentationConfig,
    pub academic_metadata: Option<AcademicMetadata>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AbstractSummaryContent {
    pub abstraction_level: u8, // L1-L4
    pub key_properties: Vec<String>,
    pub source_references: Vec<SourceReference>,
    pub derivation_metadata: DerivationMetadata,
    pub content: SimplifiedContentStructure,
    pub presentation_config: PresentationConfig,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ConceptMapContent {
    pub central_concept: String,
    pub relationship_types: Vec<RelationshipType>,
    pub source_references: Vec<SourceReference>,
    pub derivation_metadata: DerivationMetadata,
    pub content: SimplifiedContentStructure,
    pub presentation_config: PresentationConfig,
}

// --- Interactive/Dynamic Content Structs ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AnimatedPresentationContent {
    pub title: String,
    pub slide_count: usize,
    pub auto_advance: bool,
    pub base_content: Vec<Section>,
    pub interaction_system: InteractionSystem,
    pub animation_timeline: AnimationTimeline,
    pub control_bindings: Vec<ControlBinding>,
    pub interaction_points: Vec<InteractionPoint>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InteractivePlaygroundContent {
    pub title: String,
    pub parameter_space: ParameterSpace,
    pub visualization_types: Vec<VisualizationType>,
    pub real_time_feedback: bool,
    pub base_content: Vec<Section>,
    pub interaction_system: InteractionSystem,
    pub control_bindings: Vec<ControlBinding>,
    pub animation_timeline: Option<AnimationTimeline>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TypeMappingDisplayContent {
    pub title: String,
    pub source_theory: String,
    pub target_theory: String,
    pub mapping_visualizations: Vec<MappingVisualization>,
    pub base_content: Vec<Section>,
    pub interaction_system: InteractionSystem,
    pub animation_timeline: Option<AnimationTimeline>,
    pub control_bindings: Vec<ControlBinding>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ResourcePanelContent {
    pub title: String,
    pub resource_categories: Vec<ResourceCategory>,
    pub search_capabilities: SearchCapabilities,
    pub filtering_options: Vec<FilterOption>,
    pub base_content: Vec<Section>,
    pub interaction_system: InteractionSystem,
    pub control_bindings: Vec<ControlBinding>,
}

// --- Relational/Comparison Content Structs ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ComparisonPageContent {
    pub title: String,
    pub comparison_criteria: Vec<ComparisonCriterion>,
    pub highlight_differences: bool,
    pub synchronized_navigation: bool,
    pub theories_involved: Vec<TheoryReference>,
    pub relationship_metadata: RelationshipMetadata,
    pub comparison_structure: ComparisonStructure,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TransformationMappingContent {
    pub title: String,
    pub transformation_type: TransformationType,
    pub step_by_step: bool,
    pub bidirectional: bool,
    pub source_theory: String,
    pub target_theory: String,
    pub transformation_steps: Vec<TransformationStep>,
    pub theories_involved: Vec<TheoryReference>,
    pub relationship_metadata: RelationshipMetadata,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ConceptAlignmentContent {
    pub title: String,
    pub alignment_type: AlignmentType,
    pub correspondence_mappings: Vec<ConceptCorrespondence>,
    pub theories_involved: Vec<TheoryReference>,
    pub relationship_metadata: RelationshipMetadata,
    pub alignment_visualizations: Vec<AlignmentVisualization>,
}

// --- Embedded/Preview Content Structs ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct StaticPreviewContent {
    pub source_document_id: String,
    pub content_snapshot: SimplifiedContentStructure,
    pub last_updated: String,
    pub auto_refresh: bool,
    pub extraction_metadata: ExtractionMetadata,
    pub viewport_config: ViewportConfig,
    pub interaction_level: InteractionLevel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct LiveEmbedContent {
    pub source_document_id: String,
    pub sync_with_source: bool,
    pub allowed_interactions: Vec<AllowedInteraction>,
    pub extraction_metadata: ExtractionMetadata,
    pub viewport_config: ViewportConfig,
    pub interaction_level: InteractionLevel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ConceptExtractContent {
    pub source_document_id: String,
    pub extracted_concepts: Vec<String>,
    pub context_preservation: ContextPreservationLevel,
    pub extraction_metadata: ExtractionMetadata,
    pub viewport_config: ViewportConfig,
    pub interaction_level: InteractionLevel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct IFrameEmbedContent {
    pub source_document_id: String,
    pub sandbox_permissions: Vec<SandboxPermission>,
    pub responsive_scaling: bool,
    pub extraction_metadata: ExtractionMetadata,
    pub viewport_config: ViewportConfig,
    pub interaction_level: InteractionLevel,
}

// --- Supporting Types ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SourceReference {
    pub source_id: String,
    pub source_type: String,            // e.g., "WikiPage", "Textbook"
    pub specific_sections: Vec<String>, // Section IDs referenced
    pub derivation_method: DerivationMethod,
    pub confidence_level: f64, // 0.0 - 1.0
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum DerivationMethod {
    ManualSummarization,
    AutomaticExtraction,
    ConceptualSimplification,
    ExampleFocus,
    AnalogicalMapping,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DerivationMetadata {
    pub derived_at: String, // timestamp
    pub derivation_rules: Vec<String>,
    pub human_reviewed: bool,
    pub accuracy_metrics: Option<AccuracyMetrics>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AccuracyMetrics {
    pub conceptual_fidelity: f64,
    pub completeness_score: f64,
    pub readability_score: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum SummarizationLevel {
    KeyDefinitionsOnly,
    MainTheoremsOnly,
    ConceptualOverview,
    DetailedSummary,
    FullDetail,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AnimationTimeline {
    pub total_duration: f64, // seconds
    pub keyframes: Vec<AnimationKeyframe>,
    pub interaction_points: Vec<InteractionPoint>,
    pub synchronization_groups: Vec<SyncGroup>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AnimationKeyframe {
    pub time: f64,                    // seconds from start
    pub target_elements: Vec<String>, // CSS selectors or element IDs
    pub animation_type: AnimationType,
    pub properties: std::collections::HashMap<String, String>,
    pub easing: EasingFunction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum AnimationType {
    FadeIn,
    FadeOut,
    SlideIn { direction: Direction },
    Highlight { color: String },
    Morph { target_shape: String },
    TypewriteText,
    CountUp { target_value: f64 },
    Transform { matrix: Vec<f64> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InteractionPoint {
    pub time: f64,
    pub interaction_type: InteractionType,
    pub target_element: String,
    pub required_action: UserAction,
    pub timeout: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum InteractionType {
    PauseForClick,
    RequireHover,
    WaitForInput,
    BranchingChoice { options: Vec<String> },
    ParameterAdjustment { parameter_name: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum UserAction {
    Click,
    Hover,
    KeyPress { key: String },
    TextInput,
    SliderAdjust,
    Selection { from_options: Vec<String> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DocumentRelationships {
    pub parent_documents: Vec<String>, // Documents this is derived from
    pub child_documents: Vec<String>,  // Documents derived from this
    pub related_concepts: Vec<ConceptReference>,
    pub cross_references: Vec<CrossReference>,
    pub dependency_graph: Option<DependencyGraph>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ConceptReference {
    pub concept_id: String,
    pub relationship_type: ConceptRelationType,
    pub strength: f64, // 0.0 - 1.0
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ConceptRelationType {
    Defines,
    Uses,
    Extends,
    Contradicts,
    Supports,
    Examples,
    Applications,
}

// --- Simplified Content Structure for Derived/Embedded Content ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SimplifiedContentStructure {
    pub key_points: Vec<KeyPoint>,
    pub essential_definitions: Vec<EssentialDefinition>,
    pub core_examples: Vec<CoreExample>,
    pub concept_relationships: Vec<ConceptRelationship>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct KeyPoint {
    pub id: String,
    pub content: Vec<RichTextSegment>,
    pub importance_level: ImportanceLevel,
    pub source_section_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ImportanceLevel {
    Critical,
    Important,
    Helpful,
    Supplementary,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct EssentialDefinition {
    pub term: String,
    pub simplified_definition: Vec<RichTextSegment>,
    pub formal_definition: Option<MathNode>,
    pub intuitive_explanation: Option<Vec<RichTextSegment>>,
}

// --- Additional Supporting Types ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum CompletenessLevel {
    Stub,
    Basic,
    Comprehensive,
    Complete,
    Authoritative,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum CourseLevel {
    HighSchool,
    UndergraduateIntro,
    UndergraduateAdvanced,
    Graduate,
    PostGraduate,
    Research,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum WritingStyle {
    Formal,
    Conversational,
    Tutorial,
    Explanatory,
    Narrative,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum RelationshipType {
    Equivalence,
    Implication,
    Specialization,
    Generalization,
    Application,
    Analogy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum InteractionLevel {
    ReadOnly,
    BasicInteraction,
    FullInteraction,
    EditingAllowed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ContentMetadata {
    pub language: Option<String>,
    pub version: Option<String>,
    pub created_at: Option<String>,
    pub last_modified: Option<String>,
    pub content_hash: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AcademicMetadata {
    pub authors: Vec<String>,
    pub date_published: Option<String>,
    pub date_modified: Option<String>,
    pub venue: Option<String>,
    pub doi: Option<String>,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DocumentStructure {
    pub abstract_content: Option<Section>,
    pub table_of_contents: Option<TocNode>,
    pub body: Vec<Section>,
    pub footnotes: Vec<Section>,
    pub glossary: Vec<Section>,
    pub bibliography: Vec<BibEntry>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ChapterInfo {
    pub chapter_number: Option<u32>,
    pub chapter_title: String,
    pub prerequisites: Vec<String>,
    pub learning_objectives: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum PaperType {
    Research,
    Survey,
    Tutorial,
    Conference,
    Journal,
    Preprint,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum NoteStyle {
    Formal,
    Casual,
    Outline,
    MindMap,
    Cornell,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InteractionSystem {
    pub controls: Vec<Control>,
    pub event_handlers: Vec<EventHandler>,
    pub state_variables: Vec<StateVariable>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct EventHandler {
    pub event_type: String,
    pub target_selector: String,
    pub action: InteractionAction,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum InteractionAction {
    UpdateParameter { parameter: String, value: String },
    TriggerAnimation { animation_id: String },
    NavigateToSlide { slide_index: usize },
    ShowTooltip { content: String },
    HighlightElement { element_id: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct StateVariable {
    pub name: String,
    pub initial_value: String,
    pub variable_type: StateVariableType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum StateVariableType {
    Number,
    Text,
    Boolean,
    Color,
    Position,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ControlBinding {
    pub control_id: String,
    pub target_variable: String,
    pub transformation: Option<ValueTransformation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ValueTransformation {
    Linear { scale: f64, offset: f64 },
    Logarithmic { base: f64 },
    Exponential { base: f64 },
    Custom { formula: String },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ParameterSpace {
    pub parameters: Vec<Parameter>,
    pub constraints: Vec<Constraint>,
    pub default_values: std::collections::HashMap<String, f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Parameter {
    pub name: String,
    pub parameter_type: ParameterType,
    pub range: ParameterRange,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ParameterType {
    Continuous,
    Discrete,
    Integer,
    Boolean,
    Categorical { options: Vec<String> },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ParameterRange {
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub step: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Constraint {
    pub constraint_type: ConstraintType,
    pub parameters_involved: Vec<String>,
    pub formula: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ConstraintType {
    Equality,
    Inequality,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum VisualizationType {
    Plot2D,
    Plot3D,
    Vector,
    Matrix,
    Graph,
    Tree,
    Diagram,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct MappingVisualization {
    pub visualization_type: VisualizationType,
    pub source_elements: Vec<String>,
    pub target_elements: Vec<String>,
    pub mapping_arrows: Vec<MappingArrow>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct MappingArrow {
    pub from: String,
    pub to: String,
    pub arrow_style: ArrowStyle,
    pub label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ArrowStyle {
    Solid,
    Dashed,
    Dotted,
    Bidirectional,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ResourceCategory {
    pub name: String,
    pub icon: Option<String>,
    pub color: Option<String>,
    pub items: Vec<ResourceItem>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ResourceItem {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub resource_type: ResourceType,
    pub link: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ResourceType {
    Definition,
    Theorem,
    Example,
    Exercise,
    Reference,
    Tool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SearchCapabilities {
    pub full_text_search: bool,
    pub semantic_search: bool,
    pub filter_by_type: bool,
    pub sort_options: Vec<SortOption>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum SortOption {
    Alphabetical,
    ByType,
    ByRelevance,
    ByDate,
    ByImportance,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct FilterOption {
    pub name: String,
    pub filter_type: FilterType,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum FilterType {
    Checkbox,
    Radio,
    Dropdown,
    Range,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TheoryReference {
    pub theory_id: String,
    pub theory_name: String,
    pub version: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct RelationshipMetadata {
    pub relationship_type: String,
    pub strength: Option<f64>,
    pub bidirectional: Option<bool>,
    pub properties: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ComparisonStructure {
    pub comparison_type: String,
    pub layout: ComparisonLayout,
    pub sections: Vec<ComparisonSection>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ComparisonLayout {
    SideBySide,
    Tabbed,
    Overlaid,
    Sequential,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ComparisonSection {
    pub section_id: String,
    pub left_content: Vec<SectionContentNode>,
    pub right_content: Vec<SectionContentNode>,
    pub comparison_notes: Option<Vec<RichTextSegment>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ComparisonCriterion {
    pub criterion_id: String,
    pub name: String,
    pub description: Option<String>,
    pub weight: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TransformationType {
    pub transformation_id: String,
    pub name: String,
    pub description: Option<String>,
    pub reversible: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AlignmentType {
    pub alignment_id: String,
    pub name: String,
    pub description: Option<String>,
    pub precision_level: AlignmentPrecision,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum AlignmentPrecision {
    Exact,
    Approximate,
    Conceptual,
    Analogical,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ConceptCorrespondence {
    pub source_concept: String,
    pub target_concept: String,
    pub correspondence_type: CorrespondenceType,
    pub confidence: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum CorrespondenceType {
    Identical,
    Similar,
    Analogous,
    Opposite,
    Related,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ExtractionMetadata {
    pub extracted_at: String,
    pub extraction_method: String,
    pub source_version: Option<String>,
    pub extraction_rules: Vec<String>,
    pub quality_metrics: Option<std::collections::HashMap<String, f64>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ViewportConfig {
    pub width: Option<String>,
    pub height: Option<String>,
    pub responsive: Option<bool>,
    pub scroll_behavior: Option<ScrollBehavior>,
    pub zoom_level: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ScrollBehavior {
    Auto,
    Smooth,
    Instant,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AllowedInteraction {
    pub interaction_type: String,
    pub permissions: Vec<String>,
    pub restrictions: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ContextPreservationLevel {
    pub level_name: String,
    pub preserve_structure: bool,
    pub preserve_formatting: bool,
    pub preserve_links: bool,
    pub preserve_metadata: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SandboxPermission {
    pub permission_type: String,
    pub allowed: bool,
    pub restrictions: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CoreExample {
    pub example_id: String,
    pub title: String,
    pub content: Vec<SectionContentNode>,
    pub difficulty_level: Option<String>,
    pub concepts_illustrated: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ConceptRelationship {
    pub source_concept: String,
    pub target_concept: String,
    pub relationship_type: ConceptRelationType,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SyncGroup {
    pub group_id: String,
    pub elements: Vec<String>,
    pub sync_type: SyncType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum SyncType {
    Timeline,
    State,
    Animation,
    Scroll,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct EasingFunction {
    pub function_type: EasingType,
    pub parameters: Option<Vec<f64>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum EasingType {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    Cubic,
    Bounce,
    Elastic,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Direction {
    pub direction_type: DirectionType,
    pub angle: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum DirectionType {
    Up,
    Down,
    Left,
    Right,
    Custom,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CrossReference {
    pub reference_id: String,
    pub target_id: String,
    pub reference_type: CrossReferenceType,
    pub display_text: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum CrossReferenceType {
    Citation,
    SeeAlso,
    Definition,
    Theorem,
    Example,
    Figure,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DependencyGraph {
    pub nodes: Vec<DependencyNode>,
    pub edges: Vec<DependencyEdge>,
    pub graph_metadata: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DependencyNode {
    pub node_id: String,
    pub content_id: String,
    pub node_type: DependencyNodeType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum DependencyNodeType {
    Concept,
    Definition,
    Theorem,
    Example,
    Section,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DependencyEdge {
    pub from_node: String,
    pub to_node: String,
    pub dependency_type: DependencyType,
    pub strength: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum DependencyType {
    Requires,
    Builds,
    References,
    Extends,
    Contradicts,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TransformationStep {
    pub step_number: usize,
    pub description: Vec<RichTextSegment>,
    pub source_concept: String,
    pub target_concept: String,
    pub transformation_rule: String,
    pub visual_representation: Option<String>,
    pub interactive_demo: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AlignmentVisualization {
    pub visualization_type: VisualizationType,
    pub source_elements: Vec<String>,
    pub target_elements: Vec<String>,
    pub alignment_arrows: Vec<AlignmentArrow>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AlignmentArrow {
    pub from_concept: String,
    pub to_concept: String,
    pub alignment_strength: f64, // 0.0 - 1.0
    pub alignment_type: String,
    pub visual_style: ArrowStyle,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TocNode {
    pub title: String,     // e.g., "1. Introduction"
    pub target_id: String, // ID of the Section
    pub children: Vec<TocNode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct BibEntry {
    pub entry_type: String, // e.g., "article", "book", "inproceedings"
    pub fields: Vec<(String, String)>, // BibTeX-like fields (author, title, year, journal, etc.)
                            // The key for this entry would be the key in the bibliography HashMap.
                            // pub formatted_citation_html: Option<String>, // Could be pre-rendered by a citation processor
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

/// Trait for converting mathematical objects into rich SectionNode structures.
pub trait ToSectionNode {
    /// Converts the object to a Section representation.
    /// - `id_prefix`: A prefix to ensure unique IDs for generated nodes.
    /// Implementers will need to compute/access AbstractionLevel ad-hoc if needed.
    fn to_section_node(&self, id_prefix: &str) -> Section;

    /// Generates a full document representation.
    /// Implementers are responsible for determining the appropriate AbstractionLevel for the main section.
    fn to_math_document(&self, id_prefix: &str) -> MathematicalContent;

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
    fn render_as_l1_schema_document(&self, id_prefix: &str) -> MathematicalContent {
        // Default implementation uses render_as_l1_schema for the main section
        let main_section = self.render_as_l1_schema(&format!("{}-main", id_prefix));

        MathematicalContent {
            id: format!("{}-l1-doc", id_prefix),
            content_type: MathematicalContentType::ScientificPaper(ScientificPaperContent {
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

// --- Presentation Configuration ---

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct PresentationConfig {
    pub layout_style: LayoutStyle,
    pub interaction_features: Vec<InteractionFeature>,
    pub target_audience: AudienceLevel,
    pub formality_level: FormalityLevel,
    pub animation_config: Option<AnimationConfig>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum LayoutStyle {
    SingleColumn,
    TwoColumn,
    MultiPanel,
    Sidebar,
    Dashboard,
    Presentation,
    Compact,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum InteractionFeature {
    ClickableLinks,
    HoverTooltips,
    ExpandableProofs,
    InteractiveControls,
    Animations,
    TypeAnnotations,
    HighlightCorrespondence,
    ParameterAdjustment,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum AudienceLevel {
    Expert,
    Graduate,
    Undergraduate,
    HighSchool,
    GeneralPublic,
    Mathematician,
    Student,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum FormalityLevel {
    FullyFormal,
    SemiFormal,
    Intuitive,
    Conversational,
    Sketchy,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AnimationConfig {
    pub enabled_animations: Vec<String>,
    pub animation_speed: f64,
    pub auto_play: Option<bool>,
    pub show_controls: Option<bool>,
}

// --- Backwards Compatibility ---

/// Legacy DocumentType enum for backwards compatibility
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum DocumentType {
    ScientificPaper,
    Textbook,
    WikiPage,
    PersonalNotes,
    BlogPost,
    TooltipSummary,
    AnimatedPresentation,
    TypeMappingDisplay,
    ComparisonPage,
    TransformationMapping,
    InteractivePlayground,
    MathematicianNotes,
    StudyNotes,
    ResourcePanel,
}

/// Backwards compatibility alias for MathDocument
pub type MathDocument = MathematicalContent;

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
