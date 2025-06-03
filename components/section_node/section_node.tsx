import React from 'react';
import { renderMathNode } from '../math_node/math_node.tsx';
import { RichTextRenderer } from '../rich_text/rich_text.tsx';

// Import proper binding types instead of duplicating interfaces
import type { SectionContentNode } from '../../bindings/SectionContentNode.ts';
import type { Section } from '../../bindings/Section.ts';
import type { ParagraphNode } from '../../bindings/ParagraphNode.ts';
import type { MathNode } from '../../bindings/MathNode.ts';
import type { ListNode } from '../../bindings/ListNode.ts';
import type { TableNode } from '../../bindings/TableNode.ts';
import type { StructuredMathNode } from '../../bindings/StructuredMathNode.ts';
import type { CodeBlockNode } from '../../bindings/CodeBlockNode.ts';
import type { ImageNode } from '../../bindings/ImageNode.ts';

import styles from './section_node.module.scss';

interface SectionNodeProps {
  sections: Section[];
  className?: string;
}

export const SectionContentRenderer: React.FC<SectionNodeProps> = ({ 
  sections, 
  className = '' 
}) => {
  return (
    <div className={`${styles.sectionsContainer} ${className}`}>
      {sections.map((section, index) => (
        <SectionRenderer key={section.id || index} section={section} />
      ))}
    </div>
  );
};

const SectionRenderer: React.FC<{ section: Section }> = ({ section }) => {
  const getMetadata = (key: string): string | undefined => {
    return section.metadata?.find(([k]) => k === key)?.[1];
  };

  const sectionLevel = getMetadata('abstraction_level') || '1';
  const sectionType = getMetadata('type') || 'general';

  return (
    <section 
      className={`${styles.section} ${styles[`level${sectionLevel}`]} ${styles[sectionType]}`}
      id={section.id}
    >
      {section.title && (
        <header className={styles.sectionHeader}>
          <h2 className={styles.sectionTitle}>
            <RichTextRenderer segments={section.title.segments} />
          </h2>
          {section.metadata.length > 0 && (
            <div className={styles.sectionMeta}>
              <span className={styles.level}>Level {sectionLevel}</span>
              {sectionType !== 'general' && (
                <span className={styles.type}>{sectionType}</span>
              )}
            </div>
          )}
        </header>
      )}
      
      <div className={styles.sectionContent}>
        {section.content.map((contentNode, index) => (
          <ContentNodeRenderer key={index} node={contentNode} />
        ))}
      </div>
    </section>
  );
};

const ContentNodeRenderer: React.FC<{ node: SectionContentNode }> = ({ node }) => {
  // Get the variant key from the union type
  const variantKey = Object.keys(node)[0] as keyof SectionContentNode;
  
  switch (variantKey) {
    case 'Paragraph': {
      const { Paragraph } = node as Extract<SectionContentNode, { Paragraph: ParagraphNode }>;
      return <ParagraphRenderer paragraph={Paragraph} />;
    }
    
    case 'MathNode': {
      const { MathNode } = node as Extract<SectionContentNode, { MathNode: { math: MathNode; label: string | null; caption: ParagraphNode | null } }>;
      return <MathNodeRenderer mathNodeContent={MathNode} />;
    }
    
    case 'StructuredMath': {
      const { StructuredMath } = node as Extract<SectionContentNode, { StructuredMath: StructuredMathNode }>;
      return <StructuredMathRenderer structuredMath={StructuredMath} />;
    }
    
    case 'List': {
      const { List } = node as Extract<SectionContentNode, { List: ListNode }>;
      return <ListRenderer list={List} />;
    }
    
    case 'Table': {
      const { Table } = node as Extract<SectionContentNode, { Table: TableNode }>;
      return <TableRenderer table={Table} />;
    }
    
    case 'CodeBlock': {
      const { CodeBlock } = node as Extract<SectionContentNode, { CodeBlock: CodeBlockNode }>;
      return <CodeBlockRenderer codeBlock={CodeBlock} />;
    }
    
    case 'Image': {
      const { Image } = node as Extract<SectionContentNode, { Image: ImageNode }>;
      return <ImageRenderer image={Image} />;
    }
    
    case 'InteractiveDiagram': {
      const { InteractiveDiagram } = node as Extract<SectionContentNode, { InteractiveDiagram: any }>;
      return <InteractiveDiagramRenderer diagram={InteractiveDiagram} />;
    }
    
    case 'CollapsibleBlock': {
      const { CollapsibleBlock } = node as Extract<SectionContentNode, { CollapsibleBlock: any }>;
      return <CollapsibleBlockRenderer block={CollapsibleBlock} />;
    }
    
    case 'Grid': {
      const { Grid } = node as Extract<SectionContentNode, { Grid: any }>;
      return <GridRenderer grid={Grid} />;
    }
    
    case 'Columns': {
      const { Columns } = node as Extract<SectionContentNode, { Columns: any }>;
      return <ColumnsRenderer columns={Columns} />;
    }
    
    case 'ThematicBreak': {
      const { ThematicBreak } = node as Extract<SectionContentNode, { ThematicBreak: any }>;
      return <ThematicBreakRenderer break={ThematicBreak} />;
    }
    
    case 'QuoteBlock': {
      const { QuoteBlock } = node as Extract<SectionContentNode, { QuoteBlock: { content: ParagraphNode[]; attribution: ParagraphNode | null } }>;
      return <QuoteBlockRenderer quote={QuoteBlock} />;
    }
    
    case 'AlertBox': {
      const { AlertBox } = node as Extract<SectionContentNode, { AlertBox: any }>;
      return <AlertBoxRenderer alert={AlertBox} />;
    }
    
    case 'CustomComponent': {
      const { CustomComponent } = node as Extract<SectionContentNode, { CustomComponent: any }>;
      return <CustomComponentRenderer component={CustomComponent} />;
    }
    
    case 'EmbeddedSectionRef': {
      const { EmbeddedSectionRef } = node as Extract<SectionContentNode, { EmbeddedSectionRef: string }>;
      return <EmbeddedSectionRefRenderer ref={EmbeddedSectionRef} />;
    }
    
    case 'SubSection': {
      const { SubSection } = node as Extract<SectionContentNode, { SubSection: Section }>;
      return <SubSectionRenderer subSection={SubSection} />;
    }
    
    case 'SideBySideLayout': {
      const { SideBySideLayout } = node as Extract<SectionContentNode, { SideBySideLayout: any }>;
      return <SideBySideLayoutRenderer layout={SideBySideLayout} />;
    }
    
    case 'PanelLayout': {
      const { PanelLayout } = node as Extract<SectionContentNode, { PanelLayout: any }>;
      return <PanelLayoutRenderer layout={PanelLayout} />;
    }
    
    case 'AnnotationOverlay': {
      const { AnnotationOverlay } = node as Extract<SectionContentNode, { AnnotationOverlay: any }>;
      return <AnnotationOverlayRenderer overlay={AnnotationOverlay} />;
    }
    
    case 'InteractiveControls': {
      const { InteractiveControls } = node as Extract<SectionContentNode, { InteractiveControls: any }>;
      return <InteractiveControlsRenderer controls={InteractiveControls} />;
    }
    
    case 'EmbeddedDocument': {
      const { EmbeddedDocument } = node as Extract<SectionContentNode, { EmbeddedDocument: any }>;
      return <EmbeddedDocumentRenderer document={EmbeddedDocument} />;
    }
    
    default:
      return <UnknownContentRenderer node={node} />;
  }
};

// Individual renderer components for each variant
const ParagraphRenderer: React.FC<{ paragraph: ParagraphNode }> = ({ paragraph }) => (
  <div className={`${styles.paragraph} ${paragraph.alignment ? styles[paragraph.alignment] : ''}`}>
    <RichTextRenderer segments={paragraph.segments} />
  </div>
);

const MathNodeRenderer: React.FC<{ mathNodeContent: { math: MathNode; label: string | null; caption: ParagraphNode | null } }> = ({ mathNodeContent }) => {
  // Check if mathNodeContent exists
  if (!mathNodeContent) {
    return (
      <div className={styles.mathBlock}>
        <span style={{color: 'red'}}>Error: No math content provided</span>
        <details style={{marginTop: '10px', fontSize: '12px', color: '#666'}}>
          <summary>Debug Info</summary>
          <pre>mathNodeContent: {JSON.stringify(mathNodeContent, null, 2)}</pre>
        </details>
      </div>
    );
  }
  
  // Check if math property exists
  if (!mathNodeContent.math) {
    return (
      <div className={styles.mathBlock}>
        <span style={{color: 'red'}}>Error: No math data provided</span>
        <details style={{marginTop: '10px', fontSize: '12px', color: '#666'}}>
          <summary>Debug Info</summary>
          <pre>mathNodeContent: {JSON.stringify(mathNodeContent, null, 2)}</pre>
        </details>
      </div>
    );
  }
  
  // Check if math has the required structure (id and content)
  if (typeof mathNodeContent.math !== 'object' || !mathNodeContent.math.id || mathNodeContent.math.content === undefined || mathNodeContent.math.content === null) {
    return (
      <div className={styles.mathBlock}>
        <span style={{color: 'red'}}>Error: Math data has invalid structure</span>
        <details style={{marginTop: '10px', fontSize: '12px', color: '#666'}}>
          <summary>Debug Info</summary>
          <pre>mathNodeContent.math: {JSON.stringify(mathNodeContent.math, null, 2)}</pre>
        </details>
      </div>
    );
  }
  
  // Check if content is the Empty string (which is valid)
  if (mathNodeContent.math.content === 'Empty') {
    return <div className={styles.mathBlock}><span style={{color: '#888'}}>Empty math content</span></div>;
  }
  
  // Check if content is an object that can be processed with Object.keys
  if (typeof mathNodeContent.math.content !== 'object' || mathNodeContent.math.content === null) {
    return (
      <div className={styles.mathBlock}>
        <span style={{color: 'red'}}>Error: Math content is not a valid object</span>
        <details style={{marginTop: '10px', fontSize: '12px', color: '#666'}}>
          <summary>Debug Info</summary>
          <pre>content type: {typeof mathNodeContent.math.content}</pre>
          <pre>content value: {JSON.stringify(mathNodeContent.math.content, null, 2)}</pre>
        </details>
      </div>
    );
  }
  
  let mathElement;
  try {
    mathElement = renderMathNode(mathNodeContent.math);
  } catch (error) {
    const errorMessage = error instanceof Error ? error.message : 'Unknown error';
    return (
      <div className={styles.mathBlock}>
        <span style={{color: 'red'}}>Error rendering math: {errorMessage}</span>
        <details style={{marginTop: '10px', fontSize: '12px', color: '#666'}}>
          <summary>Debug Info</summary>
          <pre>Error: {error instanceof Error ? error.stack : String(error)}</pre>
          <pre>Math Node: {JSON.stringify(mathNodeContent.math, null, 2)}</pre>
        </details>
      </div>
    );
  }

  return (
    <div className={styles.mathBlock}>
      <div className={styles.mathContent}>
        {mathElement}
      </div>
      {mathNodeContent.label && (
        <div className={styles.mathLabel}>{mathNodeContent.label}</div>
      )}
      {mathNodeContent.caption && (
        <div className={styles.mathCaption}>
          <RichTextRenderer segments={mathNodeContent.caption.segments} />
        </div>
      )}
    </div>
  );
};

const StructuredMathRenderer: React.FC<{ structuredMath: StructuredMathNode }> = ({ structuredMath }) => {
  // Get the variant key from the union type
  const variantKey = Object.keys(structuredMath)[0] as keyof StructuredMathNode;
  
  switch (variantKey) {
    case 'Definition': {
      const { Definition } = structuredMath as Extract<StructuredMathNode, { Definition: any }>;
      return (
        <div className={styles.structuredMathDefinition}>
          <h3 className={styles.definitionTitle}>
            <RichTextRenderer segments={Definition.term_display} />
          </h3>
          {Definition.label && (
            <div className={styles.definitionLabel}>{Definition.label}</div>
          )}
          <div className={styles.definitionContent}>
            {Definition.body.map((contentNode: SectionContentNode, index: number) => (
              <ContentNodeRenderer key={index} node={contentNode} />
            ))}
          </div>
        </div>
      );
    }
    
    case 'TheoremLike': {
      const { TheoremLike } = structuredMath as Extract<StructuredMathNode, { TheoremLike: any }>;
      return (
        <div className={styles.structuredMathTheorem}>
          <h3 className={styles.theoremTitle}>
            {TheoremLike.kind} {TheoremLike.label && `(${TheoremLike.label})`}
          </h3>
          <div className={styles.theoremStatement}>
            {/* Handle TheoremStatement which can be Content or Mathematical */}
            {'Content' in TheoremLike.statement ? (
              TheoremLike.statement.Content.map((contentNode: SectionContentNode, index: number) => (
                <ContentNodeRenderer key={index} node={contentNode} />
              ))
            ) : (
              <div className={styles.mathematicalStatement}>
                {TheoremLike.statement.Mathematical && TheoremLike.statement.Mathematical.content ? renderMathNode(TheoremLike.statement.Mathematical) : <span>Invalid mathematical statement</span>}
              </div>
            )}
          </div>
          {TheoremLike.proof && (
            <div className={styles.theoremProof}>
              {/* Render proof display node - would need additional logic based on ProofDisplayNode structure */}
              <div className={styles.proofPlaceholder}>
                [Proof rendering not yet implemented]
              </div>
            </div>
          )}
        </div>
      );
    }
    
    case 'Example': {
      const { Example } = structuredMath as Extract<StructuredMathNode, { Example: any }>;
      return (
        <div className={styles.structuredMathExample}>
          <h3 className={styles.exampleTitle}>
            Example {Example.label && `(${Example.label})`}
          </h3>
          {Example.introduction.length > 0 && (
            <div className={styles.exampleIntroduction}>
              {Example.introduction.map((contentNode: SectionContentNode, index: number) => (
                <ContentNodeRenderer key={index} node={contentNode} />
              ))}
            </div>
          )}
          <div className={styles.exampleBody}>
            {Example.body.map((contentNode: SectionContentNode, index: number) => (
              <ContentNodeRenderer key={index} node={contentNode} />
            ))}
          </div>
          {Example.explanation.length > 0 && (
            <div className={styles.exampleExplanation}>
              {Example.explanation.map((contentNode: SectionContentNode, index: number) => (
                <ContentNodeRenderer key={index} node={contentNode} />
              ))}
            </div>
          )}
        </div>
      );
    }
    
    case 'Remark': {
      const { Remark } = structuredMath as Extract<StructuredMathNode, { Remark: any }>;
      return (
        <div className={styles.structuredMathRemark}>
          <h3 className={styles.remarkTitle}>
            Remark {Remark.label && `(${Remark.label})`}
          </h3>
          <div className={styles.remarkContent}>
            {Remark.body.map((contentNode: SectionContentNode, index: number) => (
              <ContentNodeRenderer key={index} node={contentNode} />
            ))}
          </div>
        </div>
      );
    }
    
    case 'Axiom': {
      const { Axiom } = structuredMath as Extract<StructuredMathNode, { Axiom: any }>;
      return (
        <div className={styles.structuredMathAxiom}>
          <h3 className={styles.axiomTitle}>
            Axiom {Axiom.label && `(${Axiom.label})`}
          </h3>
          <div className={styles.axiomStatement}>
            {Axiom.statement.map((contentNode: SectionContentNode, index: number) => (
              <ContentNodeRenderer key={index} node={contentNode} />
            ))}
          </div>
        </div>
      );
    }
    
    case 'Exercise': {
      const { Exercise } = structuredMath as Extract<StructuredMathNode, { Exercise: any }>;
      return (
        <div className={styles.structuredMathExercise}>
          <h3 className={styles.exerciseTitle}>
            Exercise {Exercise.label && `(${Exercise.label})`}
          </h3>
          <div className={styles.exerciseProblem}>
            {Exercise.problem_statement.map((contentNode: SectionContentNode, index: number) => (
              <ContentNodeRenderer key={index} node={contentNode} />
            ))}
          </div>
          {/* Hints and solution would need CollapsibleBlockNode renderer */}
          <div className={styles.exercisePlaceholder}>
            [Exercise hints and solution rendering not yet implemented]
          </div>
        </div>
      );
    }
    
    case 'ConstructorDefinition': {
      const { ConstructorDefinition } = structuredMath as Extract<StructuredMathNode, { ConstructorDefinition: any }>;
      return (
        <div className={styles.structuredMathConstructor}>
          <h3 className={styles.constructorTitle}>
            <RichTextRenderer segments={ConstructorDefinition.title_display} />
            {ConstructorDefinition.label && ` (${ConstructorDefinition.label})`}
          </h3>
          <div className={styles.constructorContent}>
            {ConstructorDefinition.body.map((contentNode: SectionContentNode, index: number) => (
              <ContentNodeRenderer key={index} node={contentNode} />
            ))}
          </div>
        </div>
      );
    }
    
    case 'CollectionView': {
      const { CollectionView } = structuredMath as Extract<StructuredMathNode, { CollectionView: any }>;
      return (
        <div className={styles.structuredMathCollection}>
          <h3 className={styles.collectionTitle}>{CollectionView.collection_type}</h3>
          <div className={styles.collectionDescription}>
            <RichTextRenderer segments={CollectionView.description.segments} />
          </div>
          <div className={styles.collectionVariants}>
            {CollectionView.variants.map(([name, description], index) => (
              <div key={index} className={styles.variant}>
                <strong>{name}:</strong> {description}
              </div>
            ))}
          </div>
        </div>
      );
    }
    
    default:
      return (
        <div className={styles.unknownStructuredMath}>
          <span className={styles.unknownType}>
            [Unknown structured math type: {variantKey}]
          </span>
        </div>
      );
  }
};

const ListRenderer: React.FC<{ list: ListNode }> = ({ list }) => {
  // Handle ListStyle union type properly
  const isOrdered = 'Ordered' in list.style;
  
  return (
    <div className={styles.list}>
      {isOrdered ? (
        <ol className={styles.listItems} start={list.start_index || 1}>
          {list.items.map((item, index) => (
            <li key={index} className={styles.listItem}>
              {item.content.map((itemNode, itemIndex) => (
                <ContentNodeRenderer key={itemIndex} node={itemNode} />
              ))}
            </li>
          ))}
        </ol>
      ) : (
        <ul className={styles.listItems}>
          {list.items.map((item, index) => (
            <li key={index} className={styles.listItem}>
              {item.content.map((itemNode, itemIndex) => (
                <ContentNodeRenderer key={itemIndex} node={itemNode} />
              ))}
            </li>
          ))}
        </ul>
      )}
    </div>
  );
};

const TableRenderer: React.FC<{ table: TableNode }> = ({ table }) => (
  <div className={styles.table}>
    {table.caption && (
      <div className={styles.tableCaption}>
        <RichTextRenderer segments={table.caption.segments} />
      </div>
    )}
    <table className={styles.tableElement}>
      {table.header_rows.length > 0 && (
        <thead>
          {table.header_rows.map((row, rowIndex) => (
            <tr key={rowIndex}>
              {row.cells.map((cell, cellIndex) => (
                <th 
                  key={cellIndex} 
                  className={styles.tableHeader}
                  colSpan={cell.col_span || 1}
                  rowSpan={cell.row_span || 1}
                >
                  {cell.content.map((cellNode, cellNodeIndex) => (
                    <ContentNodeRenderer key={cellNodeIndex} node={cellNode} />
                  ))}
                </th>
              ))}
            </tr>
          ))}
        </thead>
      )}
      <tbody>
        {table.body_rows.map((row, rowIndex) => (
          <tr key={rowIndex}>
            {row.cells.map((cell, cellIndex) => (
              <td 
                key={cellIndex} 
                className={styles.tableCell}
                colSpan={cell.col_span || 1}
                rowSpan={cell.row_span || 1}
              >
                {cell.content.map((cellNode, cellNodeIndex) => (
                  <ContentNodeRenderer key={cellNodeIndex} node={cellNode} />
                ))}
              </td>
            ))}
          </tr>
        ))}
      </tbody>
      {table.footer_rows.length > 0 && (
        <tfoot>
          {table.footer_rows.map((row, rowIndex) => (
            <tr key={rowIndex}>
              {row.cells.map((cell, cellIndex) => (
                <td 
                  key={cellIndex} 
                  className={styles.tableFooter}
                  colSpan={cell.col_span || 1}
                  rowSpan={cell.row_span || 1}
                >
                  {cell.content.map((cellNode, cellNodeIndex) => (
                    <ContentNodeRenderer key={cellNodeIndex} node={cellNode} />
                  ))}
                </td>
              ))}
            </tr>
          ))}
        </tfoot>
      )}
    </table>
  </div>
);

const SubSectionRenderer: React.FC<{ subSection: Section }> = ({ subSection }) => (
  <div className={styles.subsection}>
    {subSection.title && (
      <h3 className={styles.subsectionTitle}>
        <RichTextRenderer segments={subSection.title.segments} />
      </h3>
    )}
    <div className={styles.subsectionContent}>
      {subSection.content.map((subNode, index) => (
        <ContentNodeRenderer key={index} node={subNode} />
      ))}
    </div>
  </div>
);

// Placeholder components for other variants (can be implemented as needed)
const CodeBlockRenderer: React.FC<{ codeBlock: CodeBlockNode }> = ({ codeBlock }) => (
  <div className={styles.codeBlock}>
    {codeBlock.language && (
      <div className={styles.codeLanguage}>{codeBlock.language}</div>
    )}
    <pre><code>{codeBlock.code}</code></pre>
    {codeBlock.caption && (
      <div className={styles.codeCaption}>
        <RichTextRenderer segments={codeBlock.caption.segments} />
      </div>
    )}
  </div>
);

const ImageRenderer: React.FC<{ image: ImageNode }> = ({ image }) => (
  <div className={styles.image}>
    <img src={image.src} alt={image.alt_text || ''} />
    {image.caption && (
      <div className={styles.imageCaption}>
        <RichTextRenderer segments={image.caption.segments} />
      </div>
    )}
  </div>
);

const ThematicBreakRenderer: React.FC<{ break: any }> = () => (
  <hr className={styles.thematicBreak} />
);

const QuoteBlockRenderer: React.FC<{ quote: { content: ParagraphNode[]; attribution: ParagraphNode | null } }> = ({ quote }) => (
  <blockquote className={styles.quoteBlock}>
    {quote.content.map((paragraph, index) => (
      <ParagraphRenderer key={index} paragraph={paragraph} />
    ))}
    {quote.attribution && (
      <cite className={styles.quoteAttribution}>
        <RichTextRenderer segments={quote.attribution.segments} />
      </cite>
    )}
  </blockquote>
);

const AlertBoxRenderer: React.FC<{ alert: any }> = ({ alert }) => (
  <div className={`${styles.alertBox} ${styles[alert.style]}`}>
    {alert.content.map((contentNode: SectionContentNode, index: number) => (
      <ContentNodeRenderer key={index} node={contentNode} />
    ))}
  </div>
);

// Placeholder components for unimplemented variants
const InteractiveDiagramRenderer: React.FC<{ diagram: any }> = () => (
  <div className={styles.placeholder}>[Interactive Diagram - Not yet implemented]</div>
);

const CollapsibleBlockRenderer: React.FC<{ block: any }> = () => (
  <div className={styles.placeholder}>[Collapsible Block - Not yet implemented]</div>
);

const GridRenderer: React.FC<{ grid: any }> = () => (
  <div className={styles.placeholder}>[Grid - Not yet implemented]</div>
);

const ColumnsRenderer: React.FC<{ columns: any }> = () => (
  <div className={styles.placeholder}>[Columns - Not yet implemented]</div>
);

const CustomComponentRenderer: React.FC<{ component: any }> = ({ component }) => (
  <div className={styles.customComponent}>
    <div className={styles.componentName}>Custom Component: {component.component_name}</div>
    {component.fallback_content.map((contentNode: SectionContentNode, index: number) => (
      <ContentNodeRenderer key={index} node={contentNode} />
    ))}
  </div>
);

const EmbeddedSectionRefRenderer: React.FC<{ ref: string }> = ({ ref }) => (
  <div className={styles.embeddedSectionRef}>
    [Section Reference: {ref}]
  </div>
);

const SideBySideLayoutRenderer: React.FC<{ layout: any }> = () => (
  <div className={styles.placeholder}>[Side-by-Side Layout - Not yet implemented]</div>
);

const PanelLayoutRenderer: React.FC<{ layout: any }> = () => (
  <div className={styles.placeholder}>[Panel Layout - Not yet implemented]</div>
);

const AnnotationOverlayRenderer: React.FC<{ overlay: any }> = () => (
  <div className={styles.placeholder}>[Annotation Overlay - Not yet implemented]</div>
);

const InteractiveControlsRenderer: React.FC<{ controls: any }> = () => (
  <div className={styles.placeholder}>[Interactive Controls - Not yet implemented]</div>
);

const EmbeddedDocumentRenderer: React.FC<{ document: any }> = () => (
  <div className={styles.placeholder}>[Embedded Document - Not yet implemented]</div>
);

const UnknownContentRenderer: React.FC<{ node: SectionContentNode }> = ({ node }) => (
  <div className={styles.unknownContent}>
    <span className={styles.unknownType}>
      [Unknown content type: {Object.keys(node)[0]}]
    </span>
  </div>
);

export default SectionContentRenderer; 