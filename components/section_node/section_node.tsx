import React from 'react';
import { renderMathNode } from '../math_node/math_node.tsx';
import { RichTextRenderer } from '../rich_text/rich_text.tsx';

// Import proper binding types instead of duplicating interfaces
import type { SectionContentNode } from '../../bindings/SectionContentNode.ts';
import type { Section } from '../../bindings/Section.ts';
import type { RichText } from '../../bindings/RichText.ts';
import type { MathNode } from '../../bindings/MathNode.ts';
import type { ListNode } from '../../bindings/ListNode.ts';
import type { TableNode } from '../../bindings/TableNode.ts';
import type { StructuredMathNode } from '../../bindings/StructuredMathNode.ts';
import type { CodeBlockNode } from '../../bindings/CodeBlockNode.ts';
import type { ImageNode } from '../../bindings/ImageNode.ts';
import type { ProofDisplayNode } from '../../bindings/ProofDisplayNode.ts';
import type { ProofStepNode } from '../../bindings/ProofStepNode.ts';
import type { ProofCaseNode } from '../../bindings/ProofCaseNode.ts';
import type { RichTextSegment } from '../../bindings/RichTextSegment.ts';

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
    case 'RichText': {
      const { RichText } = node as Extract<SectionContentNode, { RichText: RichText }>;
      return <ParagraphRenderer paragraph={RichText} />;
    }
    
    case 'MathNode': {
      const { MathNode } = node as Extract<SectionContentNode, { MathNode: { math: MathNode; label: string | null; caption: RichText | null } }>;
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
      const { QuoteBlock } = node as Extract<SectionContentNode, { QuoteBlock: { content: RichText[]; attribution: RichText | null } }>;
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
const ParagraphRenderer: React.FC<{ paragraph: RichText }> = ({ paragraph }) => (
  <div className={`${styles.paragraph} ${paragraph.alignment ? styles[paragraph.alignment] : ''}`}>
    <RichTextRenderer segments={paragraph.segments} />
  </div>
);

const MathNodeRenderer: React.FC<{ mathNodeContent: { math: MathNode; label: string | null; caption: RichText | null } }> = ({ mathNodeContent }) => {
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
            <RichTextRenderer segments={Definition.term_display.segments} />
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
              <ProofDisplayRenderer proof={TheoremLike.proof} />
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

const QuoteBlockRenderer: React.FC<{ quote: { content: RichText[]; attribution: RichText | null } }> = ({ quote }) => (
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

const ProofDisplayRenderer: React.FC<{ proof: ProofDisplayNode }> = ({ proof }) => {
  try {
    if (!proof) {
      return (
        <div className={styles.proofDisplay}>
          <div className={styles.proofTitle}>
            [Invalid proof data]
          </div>
        </div>
      );
    }

    return (
      <div className={styles.proofDisplay}>
        {proof.title?.segments && (
          <div className={styles.proofTitle}>
            <RichTextRenderer segments={proof.title.segments} />
          </div>
        )}
        
        {proof.strategy && proof.strategy.length > 0 && (
          <div className={styles.proofStrategy}>
            <h4 className={styles.strategyTitle}>Strategy:</h4>
            {proof.strategy.map((contentNode, index) => (
              <ContentNodeRenderer key={index} node={contentNode} />
            ))}
          </div>
        )}
        
        <div className={styles.proofSteps}>
          {proof.steps?.map((step, index) => (
            <ProofStepRenderer key={index} step={step} stepNumber={index + 1} />
          )) || null}
        </div>
        
        {proof.qed_symbol && (
          <div className={styles.qedSymbol}>
            {proof.qed_symbol}
          </div>
        )}
      </div>
    );
  } catch (error) {
    console.error('Error rendering proof display:', error);
    return (
      <div className={styles.proofDisplay}>
        <div className={styles.proofTitle}>
          [Error rendering proof: {error instanceof Error ? error.message : 'Unknown error'}]
        </div>
      </div>
    );
  }
};

const ProofStepRenderer: React.FC<{ step: ProofStepNode; stepNumber: number }> = ({ step, stepNumber }) => {
  // Add error boundary for this component
  try {
    // Get the variant key from the union type
    const variantKey = Object.keys(step)[0] as keyof ProofStepNode;
    
    switch (variantKey) {
      case 'Statement': {
        const { Statement } = step as Extract<ProofStepNode, { Statement: { claim: RichTextSegment[]; justification: RichTextSegment[] } }>;
        return (
          <div className={styles.proofStatement}>
            <div className={styles.stepNumber}>{stepNumber}.</div>
            <div className={styles.statementContent}>
              <div className={styles.claim}>
                <RichTextRenderer segments={Statement?.claim || []} />
              </div>
              {Statement?.justification && Statement.justification.length > 0 && (
                <div className={styles.justification}>
                  <RichTextRenderer segments={Statement.justification} />
                </div>
              )}
            </div>
          </div>
        );
      }
      
      case 'TacticApplication': {
        const { TacticApplication } = step as Extract<ProofStepNode, { TacticApplication: import('../../bindings/TacticDisplayNode.ts').TacticDisplayNode }>;
        return (
          <div className={styles.proofTacticApplication}>
            <div className={styles.stepNumber}>{stepNumber}.</div>
            <div className={styles.tacticContent}>
              <TacticApplicationRenderer tactic={TacticApplication} />
            </div>
          </div>
        );
      }
      
      case 'Elaboration': {
        const { Elaboration } = step as Extract<ProofStepNode, { Elaboration: any[] }>;
        return (
          <div className={styles.proofElaboration}>
            <div className={styles.stepNumber}>{stepNumber}.</div>
            <div className={styles.elaborationContent}>
              {Elaboration?.map((contentNode, index) => (
                <ContentNodeRenderer key={index} node={contentNode} />
              )) || null}
            </div>
          </div>
        );
      }
      
      case 'CaseAnalysis': {
        const { CaseAnalysis } = step as Extract<ProofStepNode, { CaseAnalysis: { introduction: any; cases: ProofCaseNode[] } }>;
        return (
          <div className={styles.proofCaseAnalysis}>
            <div className={styles.stepNumber}>{stepNumber}.</div>
            <div className={styles.caseAnalysisContent}>
              {CaseAnalysis?.introduction?.segments && (
                <div className={styles.caseIntroduction}>
                  <RichTextRenderer segments={CaseAnalysis.introduction.segments} />
                </div>
              )}
              <div className={styles.cases}>
                {CaseAnalysis?.cases?.map((caseNode, index) => (
                  <ProofCaseRenderer key={index} caseNode={caseNode} caseNumber={index + 1} />
                )) || null}
              </div>
            </div>
          </div>
        );
      }
      
      case 'InductiveProof': {
        const { InductiveProof } = step as Extract<ProofStepNode, { InductiveProof: any }>;
        return (
          <div className={styles.proofInductive}>
            <div className={styles.stepNumber}>{stepNumber}.</div>
            <div className={styles.inductiveContent}>
              {InductiveProof?.variable_of_induction && (
                <div className={styles.inductionVariable}>
                  <strong>Induction on: </strong>
                  {renderMathNode(InductiveProof.variable_of_induction)}
                </div>
              )}
              
              {InductiveProof?.base_case && (
                <div className={styles.baseCase}>
                  <h5>Base Case:</h5>
                  <ProofDisplayRenderer proof={InductiveProof.base_case} />
                </div>
              )}
              
              {InductiveProof?.inductive_hypothesis?.segments && (
                <div className={styles.inductiveHypothesis}>
                  <h5>Inductive Hypothesis:</h5>
                  <RichTextRenderer segments={InductiveProof.inductive_hypothesis.segments} />
                </div>
              )}
              
              {InductiveProof?.inductive_step && (
                <div className={styles.inductiveStep}>
                  <h5>Inductive Step:</h5>
                  <ProofDisplayRenderer proof={InductiveProof.inductive_step} />
                </div>
              )}
            </div>
          </div>
        );
      }
      
      case 'Assume': {
        const { Assume } = step as Extract<ProofStepNode, { Assume: any }>;
        return (
          <div className={styles.proofAssumption}>
            <div className={styles.stepNumber}>{stepNumber}.</div>
            <div className={styles.assumptionContent}>
              <strong>Assume: </strong>
              <RichTextRenderer segments={Assume?.segments || []} />
            </div>
          </div>
        );
      }
      
      case 'Goal': {
        const { Goal } = step as Extract<ProofStepNode, { Goal: any }>;
        return (
          <div className={styles.proofGoal}>
            <div className={styles.stepNumber}>{stepNumber}.</div>
            <div className={styles.goalContent}>
              <strong>Goal: </strong>
              <RichTextRenderer segments={Goal?.segments || []} />
            </div>
          </div>
        );
      }
      
      case 'NestedProof': {
        const { NestedProof } = step as Extract<ProofStepNode, { NestedProof: ProofDisplayNode }>;
        return (
          <div className={styles.proofNested}>
            <div className={styles.stepNumber}>{stepNumber}.</div>
            <div className={styles.nestedContent}>
              {NestedProof && <ProofDisplayRenderer proof={NestedProof} />}
            </div>
          </div>
        );
      }
      
      default:
        return (
          <div className={styles.unknownProofStep}>
            <div className={styles.stepNumber}>{stepNumber}.</div>
            <span className={styles.unknownType}>
              [Unknown proof step type: {variantKey}]
            </span>
          </div>
        );
    }
  } catch (error) {
    console.error('Error rendering proof step:', error);
    return (
      <div className={styles.unknownProofStep}>
        <div className={styles.stepNumber}>{stepNumber}.</div>
        <span className={styles.unknownType}>
          [Error rendering proof step: {error instanceof Error ? error.message : 'Unknown error'}]
        </span>
      </div>
    );
  }
};

const TacticApplicationRenderer: React.FC<{ tactic: import('../../bindings/TacticDisplayNode.ts').TacticDisplayNode }> = ({ tactic }) => {
  try {
    // Get the variant key from the union type
    const variantKey = Object.keys(tactic)[0] as keyof import('../../bindings/TacticDisplayNode.ts').TacticDisplayNode;
    
    switch (variantKey) {
      case 'IntroduceQuantifier': {
        const { IntroduceQuantifier } = tactic as Extract<import('../../bindings/TacticDisplayNode.ts').TacticDisplayNode, { IntroduceQuantifier: any }>;
        return (
          <div className={styles.tacticIntroduceQuantifier}>
            <div className={styles.tacticName}>Introduce Quantifier</div>
            <div className={styles.tacticDescription}>
              <RichTextRenderer segments={IntroduceQuantifier.object_description.segments} />
            </div>
            {IntroduceQuantifier.before_state && (
              <div className={styles.beforeState}>
                <strong>Before:</strong> <RichTextRenderer segments={IntroduceQuantifier.before_state.segments} />
              </div>
            )}
            {IntroduceQuantifier.after_state && (
              <div className={styles.afterState}>
                <strong>After:</strong> <RichTextRenderer segments={IntroduceQuantifier.after_state.segments} />
              </div>
            )}
          </div>
        );
      }
      
      case 'IntroduceFreshVariable': {
        const { IntroduceFreshVariable } = tactic as Extract<import('../../bindings/TacticDisplayNode.ts').TacticDisplayNode, { IntroduceFreshVariable: any }>;
        return (
          <div className={styles.tacticIntroduceFreshVariable}>
            <div className={styles.tacticName}>Introduce Fresh Variable</div>
            <div className={styles.tacticDetails}>
              <div><strong>Target:</strong> <RichTextRenderer segments={IntroduceFreshVariable.target_quantifier.segments} /></div>
              <div><strong>New Variable:</strong> <RichTextRenderer segments={IntroduceFreshVariable.fresh_variable_name.segments} /></div>
              <div><strong>Explanation:</strong> <RichTextRenderer segments={IntroduceFreshVariable.explanation.segments} /></div>
            </div>
          </div>
        );
      }
      
      case 'ProvideWitness': {
        const { ProvideWitness } = tactic as Extract<import('../../bindings/TacticDisplayNode.ts').TacticDisplayNode, { ProvideWitness: any }>;
        return (
          <div className={styles.tacticProvideWitness}>
            <div className={styles.tacticName}>Provide Witness</div>
            <div className={styles.tacticDetails}>
              <div><strong>Target:</strong> <RichTextRenderer segments={ProvideWitness.target_quantifier.segments} /></div>
                             <div><strong>Witness:</strong> {renderMathNode(ProvideWitness.witness_expression)}</div>
              <div><strong>Explanation:</strong> <RichTextRenderer segments={ProvideWitness.witness_explanation.segments} /></div>
              {ProvideWitness.verification_steps.length > 0 && (
                <div className={styles.verificationSteps}>
                  <strong>Verification:</strong>
                  {ProvideWitness.verification_steps.map((step, index) => (
                    <ContentNodeRenderer key={index} node={step} />
                  ))}
                </div>
              )}
            </div>
          </div>
        );
      }
      
      case 'ExactWith': {
        const { ExactWith } = tactic as Extract<import('../../bindings/TacticDisplayNode.ts').TacticDisplayNode, { ExactWith: any }>;
        return (
          <div className={styles.tacticExactWith}>
            <div className={styles.tacticName}>Exact (Apply Theorem)</div>
            <div className={styles.tacticDetails}>
              <div><strong>Theorem:</strong> <RichTextRenderer segments={ExactWith.theorem_name.segments} /></div>
                             <div><strong>Statement:</strong> <RichTextRenderer segments={ExactWith.theorem_statement.segments} /></div>
               {ExactWith.instantiation_mapping.length > 0 && (
                 <div className={styles.instantiations}>
                   <strong>Instantiations:</strong>
                   {ExactWith.instantiation_mapping.map((pair, index) => (
                     <div key={index} className={styles.instantiationPair}>
                       <RichTextRenderer segments={pair.variable_name.segments} /> ↦ {renderMathNode(pair.variable_value)}
                     </div>
                   ))}
                 </div>
               )}
            </div>
          </div>
        );
      }
      
      case 'Rewrite': {
        const { Rewrite } = tactic as Extract<import('../../bindings/TacticDisplayNode.ts').TacticDisplayNode, { Rewrite: any }>;
        return (
          <div className={styles.tacticRewrite}>
            <div className={styles.tacticName}>Rewrite</div>
                         <div className={styles.tacticDetails}>
               <div><strong>Target:</strong> {renderMathNode(Rewrite.target_expression)}</div>
               <div><strong>Using:</strong> <RichTextRenderer segments={Rewrite.theorem_name.segments} /></div>
               <div><strong>Rule:</strong> <RichTextRenderer segments={Rewrite.theorem_rule.segments} /></div>
               <div><strong>Direction:</strong> 
                 {'LeftToRight' in Rewrite.direction ? (
                   <span>Left to Right</span>
                 ) : (
                   <span>Right to Left</span>
                 )}
               </div>
               <div><strong>Result:</strong> {' '}
                 {'LeftToRight' in Rewrite.direction ? (
                   renderMathNode(Rewrite.direction.LeftToRight.right_side)
                 ) : (
                   renderMathNode(Rewrite.direction.RightToLeft.right_side)
                 )}
               </div>
               {Rewrite.step_by_step_transformation.length > 0 && (
                 <div className={styles.rewriteSteps}>
                   <strong>Steps:</strong>
                   {Rewrite.step_by_step_transformation.map((step, index) => (
                     <div key={index} className={styles.rewriteStep}>
                       {renderMathNode(step.before)} → {renderMathNode(step.after)}
                       <span className={styles.ruleApplied}> (by <RichTextRenderer segments={step.rule_applied.segments} />)</span>
                     </div>
                   ))}
                 </div>
               )}
             </div>
          </div>
        );
      }
      
      case 'AssumeImplicationAntecedent': {
        const { AssumeImplicationAntecedent } = tactic as Extract<import('../../bindings/TacticDisplayNode.ts').TacticDisplayNode, { AssumeImplicationAntecedent: any }>;
        return (
          <div className={styles.tacticAssumeAntecedent}>
            <div className={styles.tacticName}>Assume Implication Antecedent</div>
            <div className={styles.tacticDetails}>
              <div><strong>Implication:</strong> {renderMathNode(AssumeImplicationAntecedent.implication_statement)}</div>
              <div><strong>Hypothesis Name:</strong> <RichTextRenderer segments={AssumeImplicationAntecedent.hypothesis_name.segments} /></div>
              <div><strong>Antecedent:</strong> {renderMathNode(AssumeImplicationAntecedent.antecedent)}</div>
              <div><strong>Consequent:</strong> {renderMathNode(AssumeImplicationAntecedent.consequent)}</div>
              <div><strong>Context:</strong> <RichTextRenderer segments={AssumeImplicationAntecedent.context_explanation.segments} /></div>
            </div>
          </div>
        );
      }
      
      case 'SplitConjunction': {
        const { SplitConjunction } = tactic as Extract<import('../../bindings/TacticDisplayNode.ts').TacticDisplayNode, { SplitConjunction: any }>;
        return (
          <div className={styles.tacticSplitConjunction}>
            <div className={styles.tacticName}>Split Conjunction</div>
            <div className={styles.tacticDetails}>
              <div><strong>Target:</strong> {renderMathNode(SplitConjunction.target_conjunction)}</div>
              <div><strong>Selected:</strong> conjunct {SplitConjunction.selected_index + 1}</div>
              <div className={styles.conjuncts}>
                <strong>Conjuncts:</strong>
                {SplitConjunction.conjuncts.map((conjunct, index) => (
                  <div key={index} className={`${styles.conjunct} ${index === SplitConjunction.selected_index ? styles.selected : ''}`}>
                    {index + 1}. {renderMathNode(conjunct)}
                  </div>
                ))}
              </div>
            </div>
          </div>
        );
      }
      
      case 'SplitDisjunction': {
        const { SplitDisjunction } = tactic as Extract<import('../../bindings/TacticDisplayNode.ts').TacticDisplayNode, { SplitDisjunction: any }>;
        return (
          <div className={styles.tacticSplitDisjunction}>
            <div className={styles.tacticName}>Split Disjunction</div>
            <div className={styles.tacticDetails}>
              <div><strong>Target:</strong> {renderMathNode(SplitDisjunction.target_disjunction)}</div>
              <div><strong>Chosen:</strong> disjunct {SplitDisjunction.chosen_index + 1}</div>
              <div><strong>Strategy:</strong> <RichTextRenderer segments={SplitDisjunction.strategy_explanation.segments} /></div>
              <div className={styles.disjuncts}>
                <strong>Disjuncts:</strong>
                {SplitDisjunction.disjuncts.map((disjunct, index) => (
                  <div key={index} className={`${styles.disjunct} ${index === SplitDisjunction.chosen_index ? styles.chosen : ''}`}>
                    {index + 1}. {renderMathNode(disjunct)}
                  </div>
                ))}
              </div>
            </div>
          </div>
        );
      }
      
      case 'Simplify': {
        const { Simplify } = tactic as Extract<import('../../bindings/TacticDisplayNode.ts').TacticDisplayNode, { Simplify: any }>;
        return (
          <div className={styles.tacticSimplify}>
            <div className={styles.tacticName}>Simplify</div>
                         <div className={styles.tacticDetails}>
               <div><strong>Original:</strong> {renderMathNode(Simplify.original_expression)}</div>
               <div><strong>Simplified:</strong> {renderMathNode(Simplify.simplified_expression)}</div>
               {Simplify.rules_used.length > 0 && (
                 <div className={styles.rulesUsed}>
                   <strong>Rules Used:</strong>
                   {Simplify.rules_used.map((rule, index) => (
                     <span key={index} className={styles.rule}>
                       <RichTextRenderer segments={rule.segments} />
                     </span>
                   ))}
                 </div>
               )}
               {Simplify.simplification_steps.length > 0 && (
                 <div className={styles.simplificationSteps}>
                   <strong>Steps:</strong>
                   {Simplify.simplification_steps.map((step, index) => (
                     <div key={index} className={styles.simplificationStep}>
                       {renderMathNode(step.before)} → {renderMathNode(step.after)} <span className={styles.stepDescription}>(<RichTextRenderer segments={step.rule_name.segments} />)</span>
                     </div>
                   ))}
                 </div>
               )}
             </div>
          </div>
        );
      }
      
      case 'Auto': {
        const { Auto } = tactic as Extract<import('../../bindings/TacticDisplayNode.ts').TacticDisplayNode, { Auto: any }>;
        return (
          <div className={styles.tacticAuto}>
            <div className={styles.tacticName}>Auto</div>
            <div className={styles.tacticDetails}>
              <div><strong>Execution:</strong> <RichTextRenderer segments={Auto.execution_summary.segments} /></div>
              {Auto.search_depth && (
                <div><strong>Search Depth:</strong> {Auto.search_depth}</div>
              )}
              {Auto.tactics_attempted.length > 0 && (
                <div className={styles.tacticsAttempted}>
                  <strong>Tactics Attempted:</strong>
                  {Auto.tactics_attempted.map((tacticName, index) => (
                    <span key={index} className={styles.tacticAttempted}>
                      <RichTextRenderer segments={tacticName.segments} />
                    </span>
                  ))}
                </div>
              )}
              {Auto.successful_path && (
                <div className={styles.successfulPath}>
                  <strong>Successful Path:</strong>
                  {Auto.successful_path.map((step, index) => (
                    <span key={index} className={styles.pathStep}>
                      <RichTextRenderer segments={step.segments} />
                    </span>
                  ))}
                </div>
              )}
            </div>
          </div>
        );
      }
      
      case 'IntroduceValueVariable': {
        const { IntroduceValueVariable } = tactic as Extract<import('../../bindings/TacticDisplayNode.ts').TacticDisplayNode, { IntroduceValueVariable: any }>;
        return (
          <div className={styles.tacticIntroduceValueVariable}>
            <div className={styles.tacticDetails}>
              <RichTextRenderer segments={IntroduceValueVariable.variable_name.segments} />
            </div>
          </div>
        );
      }

      case 'Induction': {
        const { Induction } = tactic as Extract<import('../../bindings/TacticDisplayNode.ts').TacticDisplayNode, { Induction: any }>;
        return (
          <div className={styles.tacticInduction}>
            <div className={styles.tacticName}>Induction</div>
            <div className={styles.tacticDetails}>
              <div><strong>Variable:</strong> <RichTextRenderer segments={Induction.induction_variable.segments} /></div>
              <div><strong>Base Case Value:</strong> {renderMathNode(Induction.base_case_value)}</div>
              <div><strong>Inductive Hypothesis:</strong> <RichTextRenderer segments={Induction.inductive_hypothesis.segments} /></div>
              <div><strong>Principle:</strong> <RichTextRenderer segments={Induction.induction_principle.segments} /></div>
              
              <div className={styles.inductionCases}>
                <div className={styles.baseCase}>
                  <h5>Base Case:</h5>
                  <ProofDisplayRenderer proof={Induction.base_case_proof} />
                </div>
                <div className={styles.inductiveStep}>
                  <h5>Inductive Step:</h5>
                  <ProofDisplayRenderer proof={Induction.inductive_step_proof} />
                </div>
              </div>
            </div>
          </div>
        );
      }
      
      default:
        return (
          <div className={styles.unknownTactic}>
            <div className={styles.tacticName}>Unknown Tactic: {variantKey}</div>
            <div className={styles.tacticDetails}>
              <pre>{JSON.stringify(tactic, null, 2)}</pre>
            </div>
          </div>
        );
    }
  } catch (error) {
    console.error('Error rendering tactic application:', error);
    return (
      <div className={styles.unknownTactic}>
        <div className={styles.tacticName}>Error Rendering Tactic</div>
        <div className={styles.tacticDetails}>
          {error instanceof Error ? error.message : 'Unknown error'}
        </div>
      </div>
    );
  }
};

const ProofCaseRenderer: React.FC<{ caseNode: ProofCaseNode; caseNumber: number }> = ({ caseNode, caseNumber }) => {
  try {
    if (!caseNode) {
      return (
        <div className={styles.proofCase}>
          <div className={styles.caseHeader}>
            <strong>Case {caseNumber}: </strong>
            [Invalid case data]
          </div>
        </div>
      );
    }

    return (
      <div className={styles.proofCase}>
        <div className={styles.caseHeader}>
          <strong>Case {caseNumber}: </strong>
          <RichTextRenderer segments={caseNode.condition?.segments || []} />
        </div>
        <div className={styles.caseProof}>
          {caseNode.proof_for_case && <ProofDisplayRenderer proof={caseNode.proof_for_case} />}
        </div>
      </div>
    );
  } catch (error) {
    console.error('Error rendering proof case:', error);
    return (
      <div className={styles.proofCase}>
        <div className={styles.caseHeader}>
          <strong>Case {caseNumber}: </strong>
          [Error rendering case: {error instanceof Error ? error.message : 'Unknown error'}]
        </div>
      </div>
    );
  }
};

export default SectionContentRenderer; 