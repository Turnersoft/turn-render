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

// Rich text types are now in rich_text.rs module

// --- High-Level Structural Content Nodes ---

/// Enum representing the different types of content blocks that can appear in a section.
/// This is the primary building block for document content.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum SectionContentNode {
    // New variant for subsections
    SubSection(Vec<Section>), // Box to avoid recursive type definition issues
    // non-recursive content nodes
    RichText(RichText),

    // most like "math"
    Math(MathNode), // simple inline/standalone math display like $$
    SecondOrderMath(SecondOrderMathNode), // More cluster info(solution to an ode, that has to be structured), etc.
    InteractiveDiagram(InteractiveDiagramNode), // More generic than Visualization
    Theorem,

    List(ListNode),
    Table(TableNode),
    CodeBlock(CodeBlockNode),
    Image(ImageNode),
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

    // NEW: Abstract branching container for any hierarchical structure
    BranchingContainer(BranchingContainer), // For ProofForest, storyboards, multiverse, etc.
}

// --- NEW: Abstract Hierarchical Container ---

/// Abstract container for representing any hierarchical, branching structure
/// Can represent ProofForest, storyboards, multiverse narratives, etc.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct BranchingContainer {
    pub container_id: String,
    pub container_type: ContainerType,
    pub nodes: Vec<BranchingNode>,
    pub layout_config: Option<ContainerLayout>,
    pub container_metadata: Vec<(String, String)>,
}

/// Types of branching containers
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ContainerType {
    ProofForest,    // Mathematical proof exploration
    Storyboard,     // Narrative storyboard
    Multiverse,     // Parallel universe/timeline structure
    DecisionTree,   // Decision-making tree
    Workflow,       // Process workflow
    MindMap,        // Conceptual mind map
    Timeline,       // Chronological timeline
    Custom(String), // Custom container type
}

/// Individual node in a branching container
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct BranchingNode {
    pub node_id: String,
    pub parent_id: Option<String>, // None for root nodes
    pub node_type: NodeType,
    pub content: Vec<SectionContentNode>, // Rich content for this node
    pub node_metadata: Vec<(String, String)>,
    pub children: Vec<String>, // IDs of child nodes
    pub node_state: NodeState,
}

/// Types of nodes in branching containers
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum NodeType {
    // Proof-specific
    ProofGoal,
    ProofStep,
    ProofManager,
    ProofCompleted,
    ProofDisproved,

    // Narrative-specific
    StoryScene,
    Character,
    Event,
    Timeline,

    // Generic
    Branch,
    Leaf,
    Decision,
    Outcome,
    Custom(String),
}

/// State of a branching node
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum NodeState {
    Active,         // Currently being worked on
    Completed,      // Successfully completed
    Failed,         // Failed/abandoned
    Pending,        // Waiting for input
    Disproved,      // Proven false (for proofs)
    Suspended,      // Temporarily paused
    Custom(String), // Custom state
}

/// Layout configuration for branching containers
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ContainerLayout {
    pub layout_type: LayoutType,
    pub direction: LayoutDirection,
    pub spacing: Option<String>, // CSS spacing
    pub alignment: Option<LayoutAlignment>,
    pub max_depth: Option<usize>,        // Maximum depth to show
    pub collapse_branches: Option<bool>, // Auto-collapse long branches
}

/// Types of layout for branching containers
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum LayoutType {
    Tree,           // Traditional tree layout
    Radial,         // Radial tree layout
    Timeline,       // Horizontal timeline
    Grid,           // Grid layout
    Flow,           // Flowchart style
    MindMap,        // Mind map style
    Custom(String), // Custom layout
}

/// Direction for layout
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum LayoutDirection {
    TopDown,   // Root at top, children below
    BottomUp,  // Root at bottom, children above
    LeftRight, // Root at left, children to right
    RightLeft, // Root at right, children to left
    Radial,    // Radial from center
}

/// Alignment within layout
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum LayoutAlignment {
    Start,   // Align to start
    Center,  // Center align
    End,     // Align to end
    Justify, // Justify content
    Stretch, // Stretch to fill
}

// --- Example Usage of Abstract Branching Container ---
//
// This abstract container can represent any hierarchical, branching structure:
//
// 1. ProofForest (from mod.rs):
// BranchingContainer {
//     container_id: "proof-1".to_string(),
//     container_type: ContainerType::ProofForest,
//     nodes: vec![
//         BranchingNode {
//             node_id: "goal-1".to_string(),
//             parent_id: None,
//             node_type: NodeType::ProofGoal,
//             content: vec![
//                 SectionContentNode::RichText(RichText::text("∀x. P(x) → Q(x)".to_string())),
//                 SectionContentNode::Math(goal_math_node),
//             ],
//             node_state: NodeState::Active,
//             children: vec!["step-1".to_string()],
//             node_metadata: vec![("tactic".to_string(), "assume".to_string())],
//         },
//         BranchingNode {
//             node_id: "step-1".to_string(),
//             parent_id: Some("goal-1".to_string()),
//             node_type: NodeType::ProofStep,
//             content: vec![
//                 SectionContentNode::RichText(RichText::text("Assume P(x)".to_string())),
//             ],
//             node_state: NodeState::Completed,
//             children: vec![],
//             node_metadata: vec![("tactic".to_string(), "assume".to_string())],
//         },
//     ],
//     layout_config: Some(ContainerLayout {
//         layout_type: LayoutType::Tree,
//         direction: LayoutDirection::TopDown,
//         spacing: Some("20px".to_string()),
//         alignment: Some(LayoutAlignment::Center),
//         max_depth: Some(5),
//         collapse_branches: Some(true),
//     }),
//     container_metadata: vec![("status".to_string(), "active".to_string())],
// }
//
// 2. Storyboard/Multiverse:
// BranchingContainer {
//     container_id: "story-1".to_string(),
//     container_type: ContainerType::Storyboard,
//     nodes: vec![
//         BranchingNode {
//             node_id: "scene-1".to_string(),
//             parent_id: None,
//             node_type: NodeType::StoryScene,
//             content: vec![
//                 SectionContentNode::RichText(RichText::text("Opening scene".to_string())),
//                 SectionContentNode::Image(image_node),
//             ],
//             node_state: NodeState::Completed,
//             children: vec!["branch-a".to_string(), "branch-b".to_string()],
//             node_metadata: vec![("location".to_string(), "forest".to_string())],
//         },
//     ],
//     layout_config: Some(ContainerLayout {
//         layout_type: LayoutType::Timeline,
//         direction: LayoutDirection::LeftRight,
//         spacing: Some("50px".to_string()),
//         alignment: Some(LayoutAlignment::Start),
//         max_depth: None,
//         collapse_branches: Some(false),
//     }),
//     container_metadata: vec![("genre".to_string(), "adventure".to_string())],
// }
//
// This approach:
// 1. Single abstract container for all branching structures
// 2. Rich content support via SectionContentNode
// 3. Flexible layout configuration
// 4. No type duplication
// 5. Extensible for any narrative or logical structure

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
    pub content: SectionContentNode, // Ordered list of content blocks within this section
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
