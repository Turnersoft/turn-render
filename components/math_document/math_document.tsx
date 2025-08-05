import React from 'react';
import { MathDocument } from '../../bindings/MathDocument';
import { SectionContentRenderer } from '../section_node/section_node';
import { cleanGroupNotation } from '../../../../../../utils/mathNotationCleaner';
import styles from './math_document.module.scss';

interface MathDocumentProps {
  content: MathDocument;
  className?: string;
}

/**
 * Core document renderer that directly uses Rust binding types
 * Handles MathematicalContent and all its content type variants
 */
export const DocumentRenderer: React.FC<MathDocumentProps> = ({ 
  content, 
  className = '' 
}) => {
  return (
    <div 
      className={`${styles.document} ${className}`}
      data-content-id={content.id}
    >
      <ContentTypeRenderer contentType={content.content_type} />
    </div>
  );
};

/**
 * Renders MathematicalContentType using exact binding type structure
 */
const ContentTypeRenderer: React.FC<{ contentType: MathDocument['content_type'] }> = ({ 
  contentType 
}) => {
  // Handle each variant of MathematicalContentType exactly as defined in Rust
  if (typeof contentType === 'object' && contentType !== null && 'ScientificPaper' in contentType) {
    const paper = (contentType as any).ScientificPaper;
    return (
      <div className={styles.scientificPaper}>
        <header className={styles.paperHeader}>
          <h1 className={styles.paperTitle}>{cleanGroupNotation(paper.title)}</h1>
          <div className={styles.paperMeta}>
            <span className={styles.paperType}>{paper.paper_type}</span>
            {paper.venue && (
              <span className={styles.paperVenue}>{paper.venue}</span>
            )}
            {paper.peer_reviewed && (
              <span className={styles.peerReviewed}>Peer Reviewed</span>
            )}
          </div>
          {paper.academic_metadata?.authors && paper.academic_metadata.authors.length > 0 && (
            <div className={styles.paperAuthors}>
              By: {paper.academic_metadata.authors.join(', ')}
            </div>
          )}
          {paper.academic_metadata?.keywords && paper.academic_metadata.keywords.length > 0 && (
            <div className={styles.paperKeywords}>
              {paper.academic_metadata.keywords.map((keyword: string, i: number) => (
                <span key={i} className={styles.keyword}>{keyword}</span>
              ))}
            </div>
          )}
        </header>
        
        {paper.structure?.abstract_content && (
          <section className={styles.abstract}>
            <h2 className={styles.abstractTitle}>Abstract</h2>
            <div className={styles.abstractContent}>
              <SectionContentRenderer sections={[paper.structure.abstract_content]} />
            </div>
          </section>
        )}
        
        {paper.structure?.body && paper.structure.body.length > 0 && (
          <main className={styles.paperBody}>
            <SectionContentRenderer sections={paper.structure.body} />
          </main>
        )}

        {paper.structure?.footnotes && paper.structure.footnotes.length > 0 && (
          <footer className={styles.footnotes}>
            <h3>Footnotes</h3>
            {paper.structure.footnotes.map((footnote: any, index: number) => (
              <div key={index} className={styles.footnote}>
                {footnote}
              </div>
            ))}
          </footer>
        )}
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'WikiPage' in contentType) {
    const wiki = (contentType as any).WikiPage;
    return (
      <div className={styles.wikiPage}>
        <header className={styles.wikiHeader}>
          <h1 className={styles.wikiTitle}>{cleanGroupNotation(wiki.title)}</h1>
          <div className={styles.wikiMeta}>
            <span className={styles.contentType}>Wiki Page</span>
          </div>
        </header>
        <div className={styles.wikiContent}>
          {wiki.structure?.body && (
            <SectionContentRenderer sections={wiki.structure.body} />
          )}
        </div>
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'Textbook' in contentType) {
    const textbook = (contentType as any).Textbook;
    return (
      <div className={styles.textbook}>
        <header className={styles.textbookHeader}>
          <h1 className={styles.textbookTitle}>{cleanGroupNotation(textbook.title)}</h1>
          <div className={styles.textbookMeta}>
            <span className={styles.contentType}>Textbook</span>
          </div>
        </header>
        <div className={styles.textbookContent}>
          {textbook.structure?.body && (
            <SectionContentRenderer sections={textbook.structure.body} />
          )}
        </div>
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'PersonalNotes' in contentType) {
    const notes = (contentType as any).PersonalNotes;
    return (
      <div className={styles.personalNotes}>
        <header className={styles.notesHeader}>
          <h1 className={styles.notesTitle}>{cleanGroupNotation(notes.title)}</h1>
          <div className={styles.notesMeta}>
            <span className={styles.contentType}>Personal Notes</span>
            <span className={styles.authorLevel}>{notes.author_level}</span>
            <span className={styles.noteStyle}>{notes.note_style}</span>
          </div>
        </header>
        <div className={styles.notesContent}>
          {notes.structure?.body && (
            <SectionContentRenderer sections={notes.structure.body} />
          )}
        </div>
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'MathematicianNotes' in contentType) {
    const notes = (contentType as any).MathematicianNotes;
    return (
      <div className={styles.mathematicianNotes}>
        <header className={styles.notesHeader}>
          <h1 className={styles.notesTitle}>{cleanGroupNotation(notes.title)}</h1>
          <div className={styles.notesMeta}>
            <span className={styles.contentType}>Mathematician Notes</span>
            <span className={styles.researchArea}>{notes.research_area}</span>
            <span className={styles.formalityLevel}>{notes.formality_level}</span>
          </div>
        </header>
        <div className={styles.notesContent}>
          {notes.structure?.body && (
            <SectionContentRenderer sections={notes.structure.body} />
          )}
        </div>
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'StudyNotes' in contentType) {
    const notes = (contentType as any).StudyNotes;
    return (
      <div className={styles.studyNotes}>
        <header className={styles.notesHeader}>
          <h1 className={styles.notesTitle}>{cleanGroupNotation(notes.title)}</h1>
          <div className={styles.notesMeta}>
            <span className={styles.contentType}>Study Notes</span>
            <span className={styles.subject}>{notes.subject}</span>
            <span className={styles.studyLevel}>{notes.study_level}</span>
            {notes.exam_prep && <span className={styles.examPrep}>Exam Prep</span>}
          </div>
        </header>
        <div className={styles.notesContent}>
          {notes.structure?.body && (
            <SectionContentRenderer sections={notes.structure.body} />
          )}
        </div>
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'TooltipSummary' in contentType) {
    const tooltip = (contentType as any).TooltipSummary;
    return (
      <div className={styles.tooltipSummary}>
        <div className={styles.tooltipMeta}>
          <span className={styles.summarizationLevel}>{tooltip.summarization_level}</span>
          {tooltip.max_length && <span className={styles.maxLength}>Max: {tooltip.max_length}</span>}
        </div>
        <div className={styles.tooltipContent}>
          {/* Simplified content structure would be rendered here */}
          <div className={styles.placeholder}>[Tooltip Summary Content]</div>
        </div>
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'BlogPost' in contentType) {
    const blog = (contentType as any).BlogPost;
    return (
      <div className={styles.blogPost}>
        <header className={styles.blogHeader}>
          <h1 className={styles.blogTitle}>{cleanGroupNotation(blog.title)}</h1>
          <div className={styles.blogMeta}>
            <span className={styles.contentType}>Blog Post</span>
            <span className={styles.writingStyle}>{blog.writing_style}</span>
            <span className={styles.targetAudience}>{blog.target_audience}</span>
            {blog.examples_included && <span className={styles.examplesIncluded}>With Examples</span>}
          </div>
        </header>
        <div className={styles.blogContent}>
          {/* Simplified content structure would be rendered here */}
          <div className={styles.placeholder}>[Blog Post Content]</div>
        </div>
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'AbstractSummary' in contentType) {
    const summary = (contentType as any).AbstractSummary;
    return (
      <div className={styles.abstractSummary}>
        <div className={styles.summaryMeta}>
          <span className={styles.abstractionLevel}>Level {summary.abstraction_level}</span>
          <span className={styles.keyProperties}>Key Properties: {summary.key_properties.join(', ')}</span>
        </div>
        <div className={styles.summaryContent}>
          {/* Simplified content structure would be rendered here */}
          <div className={styles.placeholder}>[Abstract Summary Content]</div>
        </div>
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'ConceptMap' in contentType) {
    const conceptMap = (contentType as any).ConceptMap;
    return (
      <div className={styles.conceptMap}>
        <header className={styles.conceptMapHeader}>
          <h1 className={styles.conceptMapTitle}>Concept Map</h1>
          <div className={styles.conceptMapMeta}>
            <span className={styles.centralConcept}>Central: {conceptMap.central_concept}</span>
            <span className={styles.relationshipTypes}>Relationships: {conceptMap.relationship_types.join(', ')}</span>
          </div>
        </header>
        <div className={styles.conceptMapContent}>
          {/* Simplified content structure would be rendered here */}
          <div className={styles.placeholder}>[Concept Map Content]</div>
        </div>
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'AnimatedPresentation' in contentType) {
    const presentation = (contentType as any).AnimatedPresentation;
    return (
      <div className={styles.animatedPresentation}>
        <header className={styles.presentationHeader}>
          <h1 className={styles.presentationTitle}>{cleanGroupNotation(presentation.title)}</h1>
          <div className={styles.presentationMeta}>
            <span className={styles.contentType}>Animated Presentation</span>
            <span className={styles.slideCount}>{presentation.slide_count} slides</span>
            {presentation.auto_advance && <span className={styles.autoAdvance}>Auto-advance</span>}
          </div>
        </header>
        <div className={styles.presentationContent}>
          {/* Base content would be rendered here */}
          <div className={styles.placeholder}>[Animated Presentation Content]</div>
        </div>
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'InteractivePlayground' in contentType) {
    const playground = (contentType as any).InteractivePlayground;
    return (
      <div className={styles.interactivePlayground}>
        <header className={styles.playgroundHeader}>
          <h1 className={styles.playgroundTitle}>{cleanGroupNotation(playground.title)}</h1>
          <div className={styles.playgroundMeta}>
            <span className={styles.contentType}>Interactive Playground</span>
            <span className={styles.visualizationTypes}>Visualizations: {playground.visualization_types.join(', ')}</span>
            {playground.real_time_feedback && <span className={styles.realTimeFeedback}>Real-time Feedback</span>}
          </div>
        </header>
        <div className={styles.playgroundContent}>
          {/* Base content would be rendered here */}
          <div className={styles.placeholder}>[Interactive Playground Content]</div>
        </div>
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'TypeMappingDisplay' in contentType) {
    const mapping = (contentType as any).TypeMappingDisplay;
    return (
      <div className={styles.typeMappingDisplay}>
        <header className={styles.mappingHeader}>
          <h1 className={styles.mappingTitle}>{cleanGroupNotation(mapping.title)}</h1>
          <div className={styles.mappingMeta}>
            <span className={styles.contentType}>Type Mapping Display</span>
            <span className={styles.sourceTheory}>From: {mapping.source_theory}</span>
            <span className={styles.targetTheory}>To: {mapping.target_theory}</span>
          </div>
        </header>
        <div className={styles.mappingContent}>
          {/* Base content would be rendered here */}
          <div className={styles.placeholder}>[Type Mapping Display Content]</div>
        </div>
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'ResourcePanel' in contentType) {
    const panel = (contentType as any).ResourcePanel;
    return (
      <div className={styles.resourcePanel}>
        <header className={styles.panelHeader}>
          <h1 className={styles.panelTitle}>{cleanGroupNotation(panel.title)}</h1>
          <div className={styles.panelMeta}>
            <span className={styles.contentType}>Resource Panel</span>
            <span className={styles.resourceCategories}>{panel.resource_categories.length} categories</span>
          </div>
        </header>
        <div className={styles.panelContent}>
          {/* Base content would be rendered here */}
          <div className={styles.placeholder}>[Resource Panel Content]</div>
        </div>
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'ComparisonPage' in contentType) {
    const comparison = (contentType as any).ComparisonPage;
    return (
      <div className={styles.comparisonPage}>
        <header className={styles.comparisonHeader}>
          <h1 className={styles.comparisonTitle}>{cleanGroupNotation(comparison.title)}</h1>
          <div className={styles.comparisonMeta}>
            <span className={styles.contentType}>Comparison Page</span>
            <span className={styles.criteriaCount}>{comparison.comparison_criteria.length} criteria</span>
            {comparison.highlight_differences && <span className={styles.highlightDifferences}>Highlight Differences</span>}
            {comparison.synchronized_navigation && <span className={styles.syncNavigation}>Sync Navigation</span>}
          </div>
        </header>
        <div className={styles.comparisonContent}>
          {/* Comparison structure would be rendered here */}
          <div className={styles.placeholder}>[Comparison Page Content]</div>
        </div>
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'TransformationMapping' in contentType) {
    const transformation = (contentType as any).TransformationMapping;
    return (
      <div className={styles.transformationMapping}>
        <header className={styles.transformationHeader}>
          <h1 className={styles.transformationTitle}>{cleanGroupNotation(transformation.title)}</h1>
          <div className={styles.transformationMeta}>
            <span className={styles.contentType}>Transformation Mapping</span>
            <span className={styles.transformationType}>{transformation.transformation_type.name}</span>
            <span className={styles.sourceTheory}>From: {transformation.source_theory}</span>
            <span className={styles.targetTheory}>To: {transformation.target_theory}</span>
            {transformation.step_by_step && <span className={styles.stepByStep}>Step-by-step</span>}
            {transformation.bidirectional && <span className={styles.bidirectional}>Bidirectional</span>}
          </div>
        </header>
        <div className={styles.transformationContent}>
          {/* Transformation steps would be rendered here */}
          <div className={styles.placeholder}>[Transformation Mapping Content]</div>
        </div>
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'ConceptAlignment' in contentType) {
    const alignment = (contentType as any).ConceptAlignment;
    return (
      <div className={styles.conceptAlignment}>
        <header className={styles.alignmentHeader}>
          <h1 className={styles.alignmentTitle}>{cleanGroupNotation(alignment.title)}</h1>
          <div className={styles.alignmentMeta}>
            <span className={styles.contentType}>Concept Alignment</span>
            <span className={styles.alignmentType}>{alignment.alignment_type.name}</span>
            <span className={styles.correspondenceCount}>{alignment.correspondence_mappings.length} correspondences</span>
          </div>
        </header>
        <div className={styles.alignmentContent}>
          {/* Alignment visualizations would be rendered here */}
          <div className={styles.placeholder}>[Concept Alignment Content]</div>
        </div>
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'StaticPreview' in contentType) {
    const preview = (contentType as any).StaticPreview;
    return (
      <div className={styles.staticPreview}>
        <header className={styles.previewHeader}>
          <h1 className={styles.previewTitle}>Static Preview</h1>
          <div className={styles.previewMeta}>
            <span className={styles.contentType}>Static Preview</span>
            <span className={styles.sourceDocument}>Source: {preview.source_document_id}</span>
            <span className={styles.lastUpdated}>Updated: {preview.last_updated}</span>
            {preview.auto_refresh && <span className={styles.autoRefresh}>Auto-refresh</span>}
          </div>
        </header>
        <div className={styles.previewContent}>
          {/* Content snapshot would be rendered here */}
          <div className={styles.placeholder}>[Static Preview Content]</div>
        </div>
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'LiveEmbed' in contentType) {
    const embed = (contentType as any).LiveEmbed;
    return (
      <div className={styles.liveEmbed}>
        <header className={styles.embedHeader}>
          <h1 className={styles.embedTitle}>Live Embed</h1>
          <div className={styles.embedMeta}>
            <span className={styles.contentType}>Live Embed</span>
            <span className={styles.sourceDocument}>Source: {embed.source_document_id}</span>
            {embed.sync_with_source && <span className={styles.syncWithSource}>Sync with source</span>}
          </div>
        </header>
        <div className={styles.embedContent}>
          {/* Embedded content would be rendered here */}
          <div className={styles.placeholder}>[Live Embed Content]</div>
        </div>
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'ConceptExtract' in contentType) {
    const extract = (contentType as any).ConceptExtract;
    return (
      <div className={styles.conceptExtract}>
        <header className={styles.extractHeader}>
          <h1 className={styles.extractTitle}>Concept Extract</h1>
          <div className={styles.extractMeta}>
            <span className={styles.contentType}>Concept Extract</span>
            <span className={styles.sourceDocument}>Source: {extract.source_document_id}</span>
            <span className={styles.extractedConcepts}>Concepts: {extract.extracted_concepts.join(', ')}</span>
          </div>
        </header>
        <div className={styles.extractContent}>
          {/* Extracted concepts would be rendered here */}
          <div className={styles.placeholder}>[Concept Extract Content]</div>
        </div>
      </div>
    );
  }

  if (typeof contentType === 'object' && contentType !== null && 'IFrameEmbed' in contentType) {
    const iframe = (contentType as any).IFrameEmbed;
    return (
      <div className={styles.iframeEmbed}>
        <header className={styles.iframeHeader}>
          <h1 className={styles.iframeTitle}>IFrame Embed</h1>
          <div className={styles.iframeMeta}>
            <span className={styles.contentType}>IFrame Embed</span>
            <span className={styles.sourceDocument}>Source: {iframe.source_document_id}</span>
            {iframe.responsive_scaling && <span className={styles.responsiveScaling}>Responsive</span>}
          </div>
        </header>
        <div className={styles.iframeContent}>
          {/* IFrame content would be rendered here */}
          <div className={styles.placeholder}>[IFrame Embed Content]</div>
        </div>
      </div>
    );
  }

  // For any unhandled content type, show debug info
  const contentTypeKey = typeof contentType === 'object' && contentType !== null 
    ? Object.keys(contentType)[0] 
    : String(contentType);
    
  return (
    <div className={styles.unhandledContentType}>
      <h2>Unhandled Content Type</h2>
      <p>Content Type: <code>{contentTypeKey}</code></p>
      <details>
        <summary>Raw Data</summary>
        <pre>{JSON.stringify(contentType, null, 2)}</pre>
      </details>
    </div>
  );
};

export default DocumentRenderer; 