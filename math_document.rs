// --- MAIN: Mathematical Content System ---

use super::{MathNode, RichTextSegment, Section, SectionContentNode};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub trait ToMathDocument {
    fn to_math_document(&self, id_prefix: &str) -> MathDocument;
}

/// The main container for mathematical content with a unique ID and content type
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct MathDocument {
    pub id: String,
    pub content_type: MathDocumentType,
}

/// Each variant represents a distinct document type with its own specialized structure and behavior
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum MathDocumentType {
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct PersonalNotesContent {
    pub title: String,
    pub author_level: AudienceLevel,
    pub note_style: NoteStyle,
    pub content_metadata: ContentMetadata,
    pub structure: DocumentStructure,
    pub relationships: DocumentRelationships,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct MathematicianNotesContent {
    pub title: String,
    pub research_area: String,
    pub formality_level: FormalityLevel,
    pub content_metadata: ContentMetadata,
    pub structure: DocumentStructure,
    pub relationships: DocumentRelationships,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AbstractSummaryContent {
    pub abstraction_level: u8, // L1-L4
    pub key_properties: Vec<String>,
    pub source_references: Vec<SourceReference>,
    pub derivation_metadata: DerivationMetadata,
    pub content: SimplifiedContentStructure,
    pub presentation_config: PresentationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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
pub struct ContentMetadata {
    pub language: Option<String>,
    pub version: Option<String>,
    pub created_at: Option<String>,
    pub last_modified: Option<String>,
    pub content_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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
pub struct DocumentRelationships {
    pub parent_documents: Vec<String>, // Documents this is derived from
    pub child_documents: Vec<String>,  // Documents derived from this
    pub related_concepts: Vec<ConceptReference>,
    pub cross_references: Vec<CrossReference>,
    pub dependency_graph: Option<DependencyGraph>,
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SimplifiedContentStructure {
    pub key_points: Vec<KeyPoint>,
    pub essential_definitions: Vec<EssentialDefinition>,
    pub core_examples: Vec<CoreExample>,
    pub concept_relationships: Vec<ConceptRelationship>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
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
