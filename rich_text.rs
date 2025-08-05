use super::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
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
        tooltip: Option<String>, // TODO: id of the tooltip page
    },
    FootnoteReference(String), // ID of a footnote
    CodeInline(String),        // For short inline code snippets, e.g., `variable_name`
    InteractiveVariable {
        /// Interactive variable that can be hovered and shows context
        variable_id: String,
        display_name: String,
        tooltip_content: Option<RichText>,
    },
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
