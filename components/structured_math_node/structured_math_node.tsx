import React from 'react';
import { renderMathNode } from '../math_node/math_node.tsx';
import { RichTextRenderer } from '../rich_text/rich_text.tsx';

// Import proper binding types
import type { SecondOrderMathNode } from '../../bindings/SecondOrderMathNode';

import type { SectionContentNode } from '../../bindings/SectionContentNode';
import type { MathNode } from '../../bindings/MathNode';

import type { CollapsibleBlockNode } from '../../bindings/CollapsibleBlockNode';

import type { Judgement } from '../../bindings/Judgement';
import type { LogicalNode } from '../../bindings/LogicalNode';
import type { Solution } from '../../bindings/Solution';

import styles from './structured_math_node.module.scss';

interface SecondOrderMathNodeProps {
  secondOrderMath: SecondOrderMathNode;
  className?: string;
}

export const SecondOrderMathNodeRenderer: React.FC<SecondOrderMathNodeProps> = ({ 
  secondOrderMath, 
  className = '' 
}) => {
  // Get the variant key from the union type
  const variantKey = Object.keys(secondOrderMath)[0] as keyof SecondOrderMathNode;
  
  switch (variantKey) {
    case 'Logic': {
      const { Logic } = secondOrderMath as Extract<SecondOrderMathNode, { Logic: LogicalNode }>;
      return <LogicalNodeRenderer logicalNode={Logic} className={className} />;
    }
    
    case 'Judgement': {
      const { Judgement } = secondOrderMath as Extract<SecondOrderMathNode, { Judgement: Judgement }>;
      return <JudgementRenderer judgement={Judgement} className={className} />;
    }
    
    case 'SystemOf': {
      const { SystemOf } = secondOrderMath as Extract<SecondOrderMathNode, { SystemOf: MathNode[] }>;
      return <SystemOfRenderer nodes={SystemOf} className={className} />;
    }
    
    case 'Solution': {
      const { Solution } = secondOrderMath as Extract<SecondOrderMathNode, { Solution: Solution }>;
      return <SolutionRenderer solution={Solution} className={className} />;
    }
    
    default:
      return (
        <div className={`${styles.unknownSecondOrderMath} ${className}`}>
          <span className={styles.unknownType}>
            [Unknown second order math type: {variantKey}]
          </span>
        </div>
      );
  }
};

// Individual renderer components for each second order math type

interface LogicalNodeRendererProps {
  logicalNode: LogicalNode;
  className?: string;
}

const LogicalNodeRenderer: React.FC<LogicalNodeRendererProps> = ({ logicalNode, className }) => {
  const variantKey = Object.keys(logicalNode)[0] as keyof LogicalNode;
  
  switch (variantKey) {
    case 'And': {
      const { And } = logicalNode as Extract<LogicalNode, { And: LogicalNode[] }>;
      return (
        <div className={`${styles.logicalAnd} ${className}`}>
          {And.map((node: LogicalNode, index: number) => (
            <LogicalNodeRenderer key={index} logicalNode={node} />
          ))}
        </div>
      );
    }
    
    case 'Or': {
      const { Or } = logicalNode as Extract<LogicalNode, { Or: LogicalNode[] }>;
      return (
        <div className={`${styles.logicalOr} ${className}`}>
          {Or.map((node: LogicalNode, index: number) => (
            <LogicalNodeRenderer key={index} logicalNode={node} />
          ))}
        </div>
      );
    }
    
    case 'Atomic': {
      const { Atomic } = logicalNode as Extract<LogicalNode, { Atomic: MathNode }>;
      return (
        <div className={`${styles.logicalAtomic} ${className}`}>
          {renderMathNode(Atomic)}
        </div>
      );
    }
    
    case 'True':
      return (
        <div className={`${styles.logicalTrue} ${className}`}>
          ⊤
        </div>
      );
    
    case 'False':
      return (
        <div className={`${styles.logicalFalse} ${className}`}>
          ⊥
        </div>
      );
    
    default:
      return (
        <div className={`${styles.unknownLogicalNode} ${className}`}>
          <span className={styles.unknownType}>
            Unknown logical node type: {variantKey}
          </span>
        </div>
      );
  }
};

interface JudgementRendererProps {
  judgement: Judgement;
  className?: string;
}

const JudgementRenderer: React.FC<JudgementRendererProps> = ({ judgement, className }) => (
  <div className={`${styles.judgement} ${className}`}>
    <div className={styles.judgementLayout}>
      {/* Left side: Variables and Quantifiers */}
      <div className={styles.judgementLeft}>
        {/* Non-quantifier variables */}
        {judgement.non_quantifiers.length > 0 && (
          <div className={styles.nonQuantifiers}>
            {judgement.non_quantifiers.map((variable, index) => (
              <span key={index} className={styles.contextVariable}>
                {renderMathNode(variable.name)} : <RichTextRenderer segments={variable.type_info.segments} />
              </span>
            ))}
          </div>
        )}
        
        {/* Quantifiers */}
        {judgement.quantifiers.map((group, index) => (
          <div key={index} className={styles.quantifierGroup}>
            {'ForAll' in group ? (
              // Universal quantifiers: vertical layout
              <div className={styles.universalGroup}>
                {group.ForAll.map((variable, varIndex) => (
                  <div key={varIndex} className={styles.universalVariable}>
                    ∀ {renderMathNode(variable.name)} : {renderMathNode(variable.type_info)}
                  </div>
                ))}
              </div>
            ) : 'Exists' in group ? (
              // Existential quantifiers: single horizontal
              <div className={styles.existentialGroup}>
                <span className={styles.existentialVariable}>
                  ∃ {renderMathNode(group.Exists.name)} : {renderMathNode(group.Exists.type_info)}
                </span>
              </div>
            ) : 'UniqueExists' in group ? (
              // Unique existential quantifiers: single horizontal
              <div className={styles.existentialGroup}>
                <span className={styles.existentialVariable}>
                  ∃! {renderMathNode(group.UniqueExists.name)} : {renderMathNode(group.UniqueExists.type_info)}
                </span>
              </div>
            ) : null}
          </div>
        ))}
      </div>
      
      {/* Middle: Turnstile */}
      <div className={styles.turnstile}>
        <span className={styles.turnstileSymbol}></span>
      </div>
      
      {/* Right side: Statement */}
      <div className={styles.judgementRight}>
        <LogicalNodeRenderer logicalNode={judgement.statement} />
      </div>
    </div>
  </div>
);




interface SystemOfRendererProps {
  nodes: MathNode[];
  className?: string;
}

const SystemOfRenderer: React.FC<SystemOfRendererProps> = ({ nodes, className }) => (
  <div className={`${styles.systemOf} ${className}`}>
    {nodes.map((node, index) => (
      <div key={index} className={styles.systemNode}>
        {renderMathNode(node)}
      </div>
    ))}
  </div>
);

interface SolutionRendererProps {
  solution: Solution;
  className?: string;
}

const SolutionRenderer: React.FC<SolutionRendererProps> = ({ solution, className }) => (
  <div className={`${styles.solution} ${className}`}>
    <span className={styles.label}>Solution:</span>
    {solution.solution_space.map((section, index) => (
      <div key={index} className={styles.solutionSection}>
        {/* Render solution sections */}
        <span className={styles.sectionTitle}>
          {section.title?.segments?.[0] && 'Text' in section.title.segments[0] 
            ? section.title.segments[0].Text 
            : 'Section'}
        </span>
      </div>
    ))}
  </div>
);

// CollapsibleBlockRenderer - handles CollapsibleBlockNode
interface CollapsibleBlockRendererProps {
  block: CollapsibleBlockNode;
}

export const CollapsibleBlockRenderer: React.FC<CollapsibleBlockRendererProps> = ({ block }) => (
  <div className={styles.collapsibleBlock}>
    <details className={styles.details}>
      <summary className={styles.summary}>
        <RichTextRenderer segments={block.summary} />
      </summary>
      <div className={styles.content}>
        {block.details.map((_contentNode: SectionContentNode, index: number) => (
          <div key={index} className={styles.contentNode}>
            {/* Render content nodes */}
            <span>Content Node {index + 1}</span>
          </div>
        ))}
      </div>
    </details>
  </div>
);
