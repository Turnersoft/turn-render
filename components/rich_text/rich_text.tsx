import React from 'react';
import { RichTextSegment } from '../../bindings/RichTextSegment';
import { RichText as RichTextType } from '../../bindings/RichText';
import { convertTextStylesToCSS } from './textStyleUtils';
import { renderMathNode } from '../math_node/math_node';
import styles from './rich_text.module.scss';

interface RichTextProps {
  segments: RichTextSegment[];
  alignment?: string | null;
}

export const RichText: React.FC<RichTextProps> = ({ segments, alignment }) => {
  const renderSegment = (segment: RichTextSegment, index: number) => {
    if ('Text' in segment) {
      return <span key={index}>{segment.Text}</span>;
    }
    
    if ('StyledText' in segment) {
      // Apply text styles using shared utility
      const style = convertTextStylesToCSS(segment.StyledText.styles);
      
      return (
        <span key={index} style={style}>
          {segment.StyledText.text}
        </span>
      );
    }
    
    if ('Math' in segment) {
      // Render math content using the proper math renderer
      return (
        <span key={index} className="math-inline">
          {renderMathNode(segment.Math)}
        </span>
      );
    }
    
    if ('Link' in segment) {
      return (
        <a key={index} href="#" title={segment.Link.tooltip || undefined}>
          {segment.Link.content.map((contentSegment, i) => renderSegment(contentSegment, i))}
        </a>
      );
    }
    
    if ('FootnoteReference' in segment) {
      return (
        <span key={index} className="footnote-ref">
          [{segment.FootnoteReference}]
        </span>
      );
    }
    
    if ('CodeInline' in segment) {
      return (
        <code key={index} style={{
          backgroundColor: '#f5f5f5',
          padding: '2px 4px',
          borderRadius: '3px',
          fontFamily: 'monospace',
          fontSize: '0.9em'
        }}>
          {segment.CodeInline}
        </code>
      );
    }
    
    if ('InteractiveVariable' in segment) {
      return (
        <span key={index} className="interactive-variable" title={segment.InteractiveVariable.tooltip_content ? JSON.stringify(segment.InteractiveVariable.tooltip_content) : undefined}>
          {segment.InteractiveVariable.display_name}
        </span>
      );
    }
    
    // Fallback for unknown segment types
    return <span key={index} className="unknown-segment">[Unknown segment type]</span>;
  };

  return (
    <div className={`rich-text ${alignment ? `align-${alignment}` : ''}`}>
      {segments.map((segment, index) => renderSegment(segment, index))}
    </div>
  );
};

// RichTextRenderer - handles RichText segments with className
export const RichTextRenderer: React.FC<{ 
  segments: RichTextSegment[]; 
  className?: string;
}> = ({ segments, className }) => (
  <span className={className}>
    {segments.map((segment, index) => {
      if ('Text' in segment) {
        return <span key={index}>{segment.Text}</span>;
      }
      
      if ('StyledText' in segment) {
        // Apply text styles using shared utility
        const style = convertTextStylesToCSS(segment.StyledText.styles);
        
        return (
          <span key={index} style={style}>
            {segment.StyledText.text}
          </span>
        );
      }
      
      if ('Math' in segment) {
        // Render math content using the proper math renderer
        return (
          <span key={index} className="math-inline">
            {renderMathNode(segment.Math)}
          </span>
        );
      }
      
      if ('Link' in segment) {
        return (
          <a key={index} href="#" title={segment.Link.tooltip || undefined}>
            {segment.Link.content.map((contentSegment) => {
              if ('Text' in contentSegment) return contentSegment.Text;
              if ('StyledText' in contentSegment) return contentSegment.StyledText.text;
              return '[Link Content]';
            })}
          </a>
        );
      }
      
      if ('FootnoteReference' in segment) {
        return (
          <sup key={index} className="footnote-ref">
            <a href={`#footnote-${segment.FootnoteReference}`}>
              [{segment.FootnoteReference}]
            </a>
          </sup>
        );
      }
      
      if ('CodeInline' in segment) {
        return (
          <code key={index} style={{
            backgroundColor: '#f5f5f5',
            padding: '2px 4px',
            borderRadius: '3px',
            fontFamily: 'monospace',
            fontSize: '0.9em'
          }}>
            {segment.CodeInline}
          </code>
        );
      }
      
      if ('InteractiveVariable' in segment) {
        return (
          <span key={index} className="interactive-variable" title={segment.InteractiveVariable.tooltip_content ? JSON.stringify(segment.InteractiveVariable.tooltip_content) : undefined}>
            {segment.InteractiveVariable.display_name}
          </span>
        );
      }
      
      // Fallback for unknown segment types
      return <span key={index}>[Unknown segment type]</span>;
    })}
  </span>
);

// ParagraphRenderer - handles RichText paragraphs with alignment
export const ParagraphRenderer: React.FC<{ paragraph: RichTextType }> = ({ paragraph }) => (
  <div className={`${styles.paragraph} ${paragraph.alignment ? styles[paragraph.alignment] : ''}`}>
    <RichTextRenderer segments={paragraph.segments} />
  </div>
);

export default RichTextRenderer; 