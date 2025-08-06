import React from 'react';
import { renderMathNode } from '../math_node/math_node.tsx';
import { RichTextRenderer, ParagraphRenderer } from '../rich_text/rich_text.tsx';
import { SecondOrderMathNodeRenderer, CollapsibleBlockRenderer } from '../structured_math_node/structured_math_node.tsx';
import { HighlightableComponent } from '../structured_math_node/math_highlighting_wrapper.tsx';

// Import proper binding types instead of duplicating interfaces
import type { SectionContentNode } from '../../bindings/SectionContentNode';
import type { Section } from '../../bindings/Section.ts';
import type { RichText } from '../../bindings/RichText.ts';
import type { MathNode } from '../../bindings/MathNode.ts';
import type { ListNode } from '../../bindings/ListNode.ts';
import type { TableNode } from '../../bindings/TableNode.ts';
import type { SecondOrderMathNode } from '../../bindings/SecondOrderMathNode.ts';
import type { CodeBlockNode } from '../../bindings/CodeBlockNode.ts';
import type { ImageNode } from '../../bindings/ImageNode.ts';
import type { BranchingContainer } from '../../bindings/BranchingContainer.ts';


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
        <ContentNodeRenderer node={section.content} />
      </div>
    </section>
  );
};

// ContentNodeRenderer - handles all SectionContentNode variants
const ContentNodeRenderer: React.FC<{ node: SectionContentNode; context?: any }> = ({ node, context }) => {
  // Get the variant key from the union type
  const variantKey = Object.keys(node)[0] as keyof SectionContentNode;
  
  switch (variantKey) {
    case 'RichText': {
      const { RichText } = node as Extract<SectionContentNode, { RichText: RichText }>;
      return <ParagraphRenderer paragraph={RichText} />;
    }
    
    case 'Math': {
      const { Math } = node as Extract<SectionContentNode, { Math: MathNode }>;
      return (
        <div className={styles.mathBlock}>
          <div className={styles.mathContent}>
            <HighlightableComponent 
              id={Math.id} 
              context={context}
              className={styles.mathHighlightable}
            >
              {renderMathNode(Math)}
            </HighlightableComponent>
          </div>
        </div>
      );
    }
    
    case 'SecondOrderMath': {
      const { SecondOrderMath } = node as Extract<SectionContentNode, { SecondOrderMath: SecondOrderMathNode }>;
      return <SecondOrderMathNodeRenderer secondOrderMath={SecondOrderMath} />;
    }
    
    case 'BranchingContainer': {
      const { BranchingContainer } = node as Extract<SectionContentNode, { BranchingContainer: BranchingContainer }>;
      return <BranchingContainerRenderer container={BranchingContainer} />;
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
      const { SubSection } = node as Extract<SectionContentNode, { SubSection: Array<Section> }>;
      return <SubSectionRenderer subSections={SubSection} />;
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
// ParagraphRenderer is now imported from rich_text.tsx

// MathNodeRenderer is now imported from math_node.tsx

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

const SubSectionRenderer: React.FC<{ subSections: Array<Section> }> = ({ subSections }) => (
  <div className={styles.subsection}>
    {subSections.map((subSection, index) => (
      <div key={index} className={styles.subsection}>
    {subSection.title && (
      <h3 className={styles.subsectionTitle}>
        <RichTextRenderer segments={subSection.title.segments} />
      </h3>
    )}
    <div className={styles.subsectionContent}>
          <ContentNodeRenderer node={subSection.content} />
    </div>
      </div>
    ))}
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

// CollapsibleBlockRenderer is implemented in structured_math_node.tsx

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
    <pre className={styles.debugInfo}>
      {JSON.stringify(node, null, 2)}
    </pre>
  </div>
);





// BranchingContainerRenderer - handles BranchingContainer
const BranchingContainerRenderer: React.FC<{ container: BranchingContainer }> = ({ container }) => (
  <div className={styles.branchingContainer} data-container-id={container.container_id}>
    <div className={styles.containerHeader}>
      <span className={styles.containerType}>
        {typeof container.container_type === 'string' 
          ? container.container_type 
          : 'Custom'}
            </span>
          </div>
    
    <div className={styles.containerNodes}>
      {container.nodes.map((node, index) => (
        <BranchingNodeRenderer key={index} node={node} stepNumber={index + 1} />
            ))}
          </div>
    
    {container.container_metadata.length > 0 && (
      <div className={styles.containerMetadata}>
        {container.container_metadata.map(([key, value], index) => (
          <span key={index} className={styles.metadataItem}>
            {key}: {value}
          </span>
                   ))}
          </div>
        )}
      </div>
    );

// Interactive Tactic Renderer
const InteractiveTacticRenderer: React.FC<{ 
  tacticName: string; 
  transformationData: any; 
  onHighlight: (sourceElements: string[], targetElements: string[]) => void;
}> = ({ tacticName, transformationData, onHighlight }) => {
  const handleTacticClick = () => {
    if (transformationData) {
      // Highlight both source elements (from previous node) and target elements (injected into context)
      const sourceElements = transformationData.source_elements || [];
      const targetElements = transformationData.target_elements || [];
      onHighlight(sourceElements, targetElements);
    }
  };

        return (
    <span 
      className={styles.interactiveTactic}
      onClick={handleTacticClick}
      title="Click to highlight source and target elements"
    >
      {tacticName}
    </span>
  );
};

// BranchingNodeRenderer - handles BranchingNode
const BranchingNodeRenderer: React.FC<{ node: any; stepNumber?: number }> = ({ node, stepNumber }) => {
  const isProofGoal = node.node_type === 'ProofGoal';
  const isProofStep = node.node_type === 'ProofStep';
  const isCompleted = node.node_state === 'Completed';
  
  // Extract tactic name from metadata
  const tacticName = node.node_metadata
    .find(([key]: [string, string]) => key === 'tactic')?.[1] || '';
  
  // Extract transformation data from metadata
  const transformationDataStr = node.node_metadata
    .find(([key]: [string, string]) => key === 'transformation_flow')?.[1] || '';
  
  let transformationData = null;
  try {
    transformationData = transformationDataStr ? JSON.parse(transformationDataStr) : null;
    if (transformationData) {
      console.log('Transformation data for node:', node.node_id, transformationData);
    }
  } catch (e) {
    console.warn('Failed to parse transformation data:', e);
  }
  
  // Extract context size from metadata
  const contextSize = node.node_metadata
    .find(([key]: [string, string]) => key === 'context_size')?.[1] || '0';
  
  // Handle highlighting of source and target elements
  const handleHighlight = (sourceElements: string[], targetElements: string[]) => {
    // Remove previous highlights
    document.querySelectorAll('.highlighted-source, .highlighted-target').forEach(el => {
      el.classList.remove('highlighted-source', 'highlighted-target');
    });
    
    console.log('Highlighting source elements:', sourceElements);
    console.log('Highlighting target elements:', targetElements);
    
    // Debug: Log all data-id attributes in the current proof node
    const currentNode = document.querySelector(`[data-node-id="${node.node_id}"]`);
    if (currentNode) {
      const allDataIds = currentNode.querySelectorAll('[data-id]');
      console.log('Available data-id elements in current node:', 
        Array.from(allDataIds).map(el => ({
          id: el.getAttribute('data-id'),
          text: el.textContent?.trim()
        }))
      );
    }
    
    // Also check if we have transformation data with actual element IDs
    if (transformationData && transformationData.source_expressions) {
      console.log('Transformation data source expressions:', transformationData.source_expressions);
      // Use actual element IDs from transformation data if available
      const actualSourceIds = transformationData.source_expressions.map((expr: any) => expr.id);
      console.log('Actual source element IDs from transformation data:', actualSourceIds);
      
      // Highlight using actual element IDs
      actualSourceIds.forEach((elementId: string) => {
        const elements = document.querySelectorAll(`[data-id="${elementId}"]`);
        elements.forEach(el => {
          el.classList.add('highlighted-source');
          console.log(`Highlighted source element with actual ID ${elementId}:`, el);
        });
      });
    }
    
    // Highlight source elements (from previous proof node) - red/orange
    sourceElements.forEach(id => {
      // Try multiple selectors to find source elements
      const selectors = [
        `[data-id*="${id}"]`,
        `[data-id="${id}"]`,
        `[data-id*="context-name-${id}"]`, // Handle context-name-prefix
        `[data-id*="-context-name-${id}"]`, // Handle full context-name pattern
        `[data-id*="-${id}"]`, // Handle any suffix pattern
        `[id*="${id}"]`,
        `[class*="${id}"]`
      ];
      
      let found = false;
      selectors.forEach(selector => {
        try {
          const elements = document.querySelectorAll(selector);
          if (elements.length > 0) {
            elements.forEach(el => {
              el.classList.add('highlighted-source');
              found = true;
              console.log(`Found source element with selector ${selector}:`, el);
            });
          }
        } catch (e) {
          // Ignore invalid selectors
        }
      });
      
      // If not found with selectors, try text content search
      if (!found) {
        const allElements = document.querySelectorAll('*');
        allElements.forEach(el => {
          if (el.textContent && el.textContent.includes(id)) {
            el.classList.add('highlighted-source');
            found = true;
            console.log(`Found source element by text content:`, el);
          }
        });
      }
      
      if (!found) {
        console.log(`No elements found for source ID: ${id}`);
      }
    });
    
    // Highlight target elements (injected into current context) - green
    targetElements.forEach(id => {
      // Try multiple selectors to find target elements
      const selectors = [
        `[data-id*="${id}"]`,
        `[data-id="${id}"]`,
        `[id*="${id}"]`,
        `[class*="${id}"]`,
        `span:contains("${id}")`,
        `div:contains("${id}")`
      ];
      
      let found = false;
      selectors.forEach(selector => {
        try {
          const elements = document.querySelectorAll(selector);
          if (elements.length > 0) {
            elements.forEach(el => {
              el.classList.add('highlighted-target');
              found = true;
            });
          }
        } catch (e) {
          // Ignore invalid selectors
        }
      });
      
      if (!found) {
        console.log(`No elements found for target ID: ${id}`);
      }
    });
  };
  
        return (
    <div 
      className={`${styles.branchingNode} ${isProofGoal ? styles.proofGoal : ''} ${isProofStep ? styles.proofStep : ''} ${isCompleted ? styles.completed : ''}`}
      data-node-id={node.node_id}
    >
      {/* Step Number */}
      {stepNumber && (
        <div className={styles.stepNumber}>
          #{stepNumber}
                 </div>
               )}
      
      {/* Node Header */}
      <div className={styles.nodeHeader}>
        {transformationData ? (
          <InteractiveTacticRenderer 
            tacticName={tacticName}
            transformationData={transformationData}
            onHighlight={handleHighlight}
          />
        ) : (
          <span className={styles.tacticName}>{tacticName}</span>
        )}
        {isCompleted && (
          <span className={`${styles.nodeState} ${styles.completedState}`}>
            ✓
                    </span>
        )}
                </div>
      
      {/* Context Variables Summary */}
      {isProofGoal && parseInt(contextSize) > 0 && (
        <div className={styles.contextSummary}>
          <span className={styles.contextLabel}>Context: {contextSize} variables</span>
            </div>
      )}
      
      {/* Main Content */}
      <div className={styles.nodeContent}>
        {node.content.map((contentNode: any, index: number) => (
          <ContentNodeRenderer 
            key={index} 
            node={contentNode} 
            context={{ proofNodeId: node.node_id, mode: 'within_proof_node' }}
          />
        ))}
                </div>
      
      {/* Children (for nested proof structure) */}
      {node.children.length > 0 && (
        <div className={styles.nodeChildren}>
          <span className={styles.childrenLabel}>→ {node.children.length} subgoals</span>
            </div>
      )}
          </div>
        );
};




    export { ContentNodeRenderer };
export default SectionContentRenderer; 