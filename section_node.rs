use super::math_node::MathNode;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use ts_rs::TS;

// --- Core Building Blocks for Rich Text ---

/// Represents a segment of rich text, allowing for mixed content within paragraphs, list items, etc.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
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
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ParagraphNode {
    pub segments: Vec<RichTextSegment>,
    pub alignment: Option<TextAlignment>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum TextAlignment {
    Left,
    Center,
    Right,
    Justify,
}

/// Defines various targets a link can point to, enabling rich interactivity.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum LinkTarget {
    Url(String),            // External web URL
    InternalPageId(String), // ID of another MathDocumentNode or SectionNode within the system
    DefinitionId {
        // Link to a specific defined term or MathNode concept
        term_id: String,                // Unique ID of the definition or concept
        theory_context: Option<String>, // e.g., "ZFC", "GroupTheory"
    },
    TheoremId(String), // Link to a specific Theorem, Lemma, etc.
    ObjectConstructorTemplate {
        // A page/section acting as a template for creating math objects
        template_id: String, // ID of the page/section that is the template
        /// Pre-filled parameters for the template, MathNode can represent concrete values or variables.
        parameters: Option<Vec<(String, MathNode)>>,
        /// Indicates the desired abstraction level (L1-L4) for the constructed object.
        target_abstraction_level: Option<u8>,
    },
    GlossaryTerm(String),         // Link to a term in a glossary
    BibliographyKey(String),      // Link to a bibliography entry
    InteractiveElementId(String), // Link to trigger/focus an interactive component on the page
}

// --- High-Level Structural Content Nodes ---

/// Enum representing the different types of content blocks that can appear in a section.
/// This is the primary building block for document content.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum SectionContentNode {
    Paragraph(ParagraphNode),
    MathBlock {
        // For display-style math equations (block-level)
        math: MathNode,
        label: Option<String>, // For equation numbering/referencing
        caption: Option<ParagraphNode>,
    },
    StructuredMath(StructuredMathContentNode), // Definitions, Theorems, Proofs, etc.
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
        fallback_content: Option<Vec<SectionContentNode>>, // Content to show if component fails
    },
    // Embeds another section, useful for transclusion or master documents.
    EmbeddedSectionRef(String), // ID of another SectionNode to embed
}

// --- Structured Mathematical Content Types ---

/// Represents formal mathematical structures like definitions, theorems, etc.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum StructuredMathContentNode {
    Definition {
        term_display: Vec<RichTextSegment>, // How the term being defined is displayed, e.g., "Group (G, *)"
        formal_term: Option<MathNode>, // The underlying formal MathNode for the term, if applicable
        label: Option<String>,         // e.g., "Definition 1.1"
        body: Vec<SectionContentNode>, // The content of the definition
        /// Metadata relating to the 4-level abstraction model.
        abstraction_meta: Option<AbstractionMetadata>,
        selectable_properties: Option<Vec<SelectableProperty>>,
    },
    TheoremLike {
        // Covers Theorem, Lemma, Proposition, Corollary, Conjecture
        kind: TheoremLikeKind,
        label: Option<String>,                // e.g., "Theorem 2.3"
        statement: Vec<SectionContentNode>,   // The theorem's statement
        proof: Option<Box<ProofDisplayNode>>, // Display-oriented proof structure
        abstraction_meta: Option<AbstractionMetadata>,
    },
    Proof(ProofDisplayNode), // A standalone proof block, perhaps linked to a theorem elsewhere
    Example {
        label: Option<String>, // e.g., "Example 1.2"
        introduction: Option<Vec<SectionContentNode>>,
        body: Vec<SectionContentNode>,
        explanation: Option<Vec<SectionContentNode>>,
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
        hints: Option<Vec<CollapsibleBlockNode>>, // Hints can be collapsible
        solution: Option<Box<CollapsibleBlockNode>>, // Solution is often collapsible
    },
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AbstractionMetadata {
    /// Abstraction level (L1-L4) as per theory_and_render.md
    pub level: Option<u8>,
    /// Link to the L1/L2 blueprint or source template for this object/definition.
    pub source_template_id: Option<String>,
    /// For L2/L3/L4, parameters that have been specified or concretized.
    pub specified_parameters: Option<Vec<(String, MathNode)>>,
    /// For L2, properties that are universally quantified (or "any valid option").
    pub universally_quantified_properties: Option<Vec<String>>,
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
}

// --- Proof Display Structures ---

/// Represents a display-oriented proof structure.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProofDisplayNode {
    pub title: Option<ParagraphNode>, // e.g., "Proof.", "Proof of Theorem 1.2."
    pub strategy: Option<Vec<SectionContentNode>>, // Optional outline of the proof strategy
    pub steps: Vec<ProofStepNode>,
    pub qed_symbol: Option<String>, // e.g., "□", "∎", or "" for none
}

/// Represents a single step or block within a proof's display.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ProofStepNode {
    Statement {
        claim: Vec<RichTextSegment>, // The mathematical claim or derivation for this step
        justification: Option<Vec<RichTextSegment>>, // e.g., "by Definition 1", "from (3.2)"
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

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProofCaseNode {
    pub condition: ParagraphNode, // e.g., "Case 1: n is even."
    pub proof_for_case: ProofDisplayNode,
}

// --- Layout and Utility Content Types ---

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ListNode {
    pub items: Vec<ListItemNode>,
    pub style: ListStyle,
    pub start_index: Option<i32>, // For ordered lists
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ListItemNode {
    /// Content of a list item can be complex, allowing nested structures.
    pub content: Vec<SectionContentNode>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ListStyle {
    Unordered(UnorderedListStyle),
    Ordered(OrderedListStyle),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum UnorderedListStyle {
    Disc, // default bullet
    Circle,
    Square,
    None,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum OrderedListStyle {
    Decimal,    // 1, 2, 3
    AlphaLower, // a, b, c
    AlphaUpper, // A, B, C
    RomanLower, // i, ii, iii
    RomanUpper, // I, II, III
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TableNode {
    pub caption: Option<ParagraphNode>,
    pub header_rows: Option<Vec<TableRowNode>>,
    pub body_rows: Vec<TableRowNode>,
    pub footer_rows: Option<Vec<TableRowNode>>,
    pub column_styles: Option<Vec<ColumnStyle>>,
    pub table_style_options: Option<TableStyleOptions>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TableRowNode {
    pub cells: Vec<TableCellNode>,
    // pub style: Option<RowStyle>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TableCellNode {
    pub content: Vec<SectionContentNode>,
    pub col_span: Option<usize>,
    pub row_span: Option<usize>,
    pub cell_type: TableCellType,
    pub alignment: Option<TextAlignment>,
    // pub style: Option<CellStyle>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum TableCellType {
    Header,
    Data,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ColumnStyle {
    pub width: Option<String>, // e.g., "20%", "100px"
    pub alignment: Option<TextAlignment>,
    // Add other column-specific styles
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TableStyleOptions {
    pub borders: Option<bool>, // Show all borders
    pub striped_rows: Option<bool>,
    // Add other table-wide styles
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CodeBlockNode {
    pub code: String,
    pub language: Option<String>, // e.g., "rust", "python", "latex", "lean", "plaintext"
    pub caption: Option<ParagraphNode>,
    pub show_line_numbers: Option<bool>,
    pub highlight_lines: Option<Vec<usize>>,
    pub is_executable: Option<bool>, // For interactive code blocks
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ImageNode {
    pub src: String, // URL or path
    pub alt_text: Option<String>,
    pub caption: Option<ParagraphNode>,
    pub width: Option<String>,
    pub height: Option<String>,
    pub alignment: Option<HorizontalAlignment>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct InteractiveDiagramNode {
    pub diagram_type_id: String, // Identifier for the type of diagram (e.g., "commutative_diagram", "function_plot")
    pub data: String,            // Diagram-specific data
    pub caption: Option<ParagraphNode>,
    pub config_options: Option<String>, // UI options for the diagram
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CollapsibleBlockNode {
    pub summary: Vec<RichTextSegment>, // The visible part when collapsed (clickable)
    pub details: Vec<SectionContentNode>, // The content shown when expanded
    pub initially_collapsed: Option<bool>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GridNode {
    pub items: Vec<GridItemNode>,
    /// Number of columns, or CSS grid-template-columns string.
    pub column_template: String, // e.g., "3" for 3 equal columns, or "1fr 2fr"
    pub row_gap: Option<String>, // e.g., "10px"
    pub column_gap: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GridItemNode {
    pub content: SectionContentNode,
    pub col_start: Option<usize>,
    pub col_end: Option<usize>, // Or col_span
    pub row_start: Option<usize>,
    pub row_end: Option<usize>, // Or row_span
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ColumnsNode {
    pub columns_content: Vec<Vec<SectionContentNode>>, // Each inner Vec is a column
    pub column_widths: Option<Vec<String>>,            // e.g., ["30%", "70%"] or ["1fr", "2fr"]
    pub gap: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ThematicBreakNode;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum AlertBoxStyle {
    Information,
    Success,
    Warning,
    Error,
    Note,
    Tip,
}

// --- Top-Level Section and Document Structures ---

/// A `SectionNode` represents a major, navigable part of a document (like a chapter or a named section).
/// It can have a title and contains various content blocks. Sections can be nested.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Section {
    // Renamed from SectionNode to avoid confusion with enum SectionContentNode
    pub id: String, // Unique ID for linking, navigation, and referencing
    pub title: Option<ParagraphNode>, // The title of the section
    pub content: Vec<SectionContentNode>, // Ordered list of content blocks within this section
    pub sub_sections: Vec<Section>, // For hierarchical document structure
    pub metadata: Option<Vec<(String, String)>>, // For tags, abstraction level, visibility, etc.
    pub display_options: Option<SectionDisplayOptions>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SectionDisplayOptions {
    pub show_title_numbering: Option<bool>,
    // Add other display-related options
}

/// Represents the root of a math document or page.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct MathDocument {
    // Renamed from MathDocumentNode
    pub id: String, // Unique ID for the document
    pub title: String,
    pub language: Option<String>, // e.g., "en-US"
    pub version: Option<String>,
    pub authors: Option<Vec<String>>,
    pub date_published: Option<String>, // ISO 8601 format
    pub date_modified: Option<String>,  // ISO 8601 format
    pub abstract_content: Option<Vec<SectionContentNode>>,
    pub table_of_contents: Option<TocNode>, // Auto-generated or manual ToC
    pub body: Vec<Section>,                 // The main content as a list of top-level sections
    pub footnotes: Option<Vec<(String, Vec<SectionContentNode>)>>, // ID -> Content
    pub glossary: Option<Vec<(String, StructuredMathContentNode)>>, // Term -> Definition
    pub bibliography: Option<Vec<(String, BibEntryNode)>>, // CitationKey -> Entry
    pub document_metadata: Option<Vec<(String, String)>>, // Page-level settings, global abstraction context
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TocNode {
    pub title: String,     // e.g., "1. Introduction"
    pub target_id: String, // ID of the Section
    pub children: Vec<TocNode>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct BibEntryNode {
    pub entry_type: String, // e.g., "article", "book", "inproceedings"
    pub fields: Vec<(String, String)>, // BibTeX-like fields (author, title, year, journal, etc.)
                            // The key for this entry would be the key in the bibliography HashMap.
                            // pub formatted_citation_html: Option<String>, // Could be pre-rendered by a citation processor
}

// Original TurnTextLineNode from user, for reference.
// Consider how its functionalities are covered by the richer structures above.
// If it's for a very specific line-by-line rendering context, it might still have a place,
// but for general document structure, the block-based SectionContentNode is more robust.
// #[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
// #[ts(export)]
// pub enum TurnTextLineNode {
//     Math(MathNode, String),
//     Phrase(String),
//     Empty,
//     Comment(String),
//     Latex(String),
//     PageLink(String), // Covered by RichTextSegment::Link with InternalPageId
//     Image(String),    // Covered by ImageNode
// }

// --- Added SelectableProperty and ToSectionNode Trait ---

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

    fn to_tooltip_node(&self, id_prefix: &str) -> Vec<RichTextSegment>;
    fn to_reference_node(&self, id_prefix: &str) -> Vec<RichTextSegment>;

    /// Generates a full document representation.
    /// Implementers are responsible for determining the appropriate AbstractionLevel for the main section.
    fn to_math_document(&self, id_prefix: &str) -> MathDocument;

    /// Renders the object as a Level 1 (L1) schema section.
    /// This is separate from to_section_node because L1 objects are never instantiated directly.
    /// Implementers should provide a manual L1 schema representation of their type.
    fn render_as_l1_schema(&self, id_prefix: &str) -> Section {
        // Default implementation uses to_section_node but adds a warning
        // Implementations should override this for proper L1 schema rendering
        let mut section = self.to_section_node(id_prefix);

        // Add warning metadata that this is a default implementation
        if let Some(metadata) = &mut section.metadata {
            metadata.push((
                "warning".to_string(),
                "Default L1 schema rendering used".to_string(),
            ));
        } else {
            section.metadata = Some(vec![(
                "warning".to_string(),
                "Default L1 schema rendering used".to_string(),
            )]);
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
            language: Some("en-US".to_string()),
            version: Some("1.0".to_string()),
            authors: None,
            date_published: None,
            date_modified: None,
            abstract_content: Some(vec![SectionContentNode::Paragraph(p_text(
                "Schema document (Level 1) - automatically generated from default implementation",
            ))]),
            table_of_contents: None,
            body: vec![main_section],
            footnotes: None,
            glossary: None,
            bibliography: None,
            document_metadata: Some(vec![
                ("abstraction_level".to_string(), "1".to_string()),
                (
                    "warning".to_string(),
                    "Default L1 schema document".to_string(),
                ),
            ]),
        }
    }
}

// Helper function to create a simple text paragraph (already exists in this file, ensure no duplication)
pub fn p_text(text: &str) -> ParagraphNode {
    ParagraphNode {
        segments: vec![RichTextSegment::Text(text.to_string())],
        alignment: None,
    }
}

// Helper to create a RichTextSegment link (already exists in this file, ensure no duplication)
pub fn link_to_definition(text: &str, term_id: &str, theory: Option<&str>) -> RichTextSegment {
    RichTextSegment::Link {
        content: vec![RichTextSegment::Text(text.to_string())],
        target: LinkTarget::DefinitionId {
            term_id: term_id.to_string(),
            theory_context: theory.map(String::from),
        },
        tooltip: Some(format!("View definition of {}", term_id)),
    }
}
