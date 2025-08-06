// Wrapper component that adds highlighting functionality to MathNode components
import React from 'react';
import { highlightSameIds, HighlightContext } from './math_highlighting';

interface HighlightableComponentProps {
  children: React.ReactNode;
  id?: string | number;
  context?: Partial<HighlightContext>;
  className?: string;
  style?: React.CSSProperties;
  onClick?: (e: React.MouseEvent) => void;
  [key: string]: any; // Allow any other props
}

export const HighlightableComponent: React.FC<HighlightableComponentProps> = ({
  children,
  id,
  context,
  className = '',
  style,
  onClick,
  ...otherProps
}) => {
  const handleClick = (e: React.MouseEvent) => {
    const target = e.currentTarget as HTMLElement;
    const dataId = target.getAttribute('data-id');
    console.log('🔍 Click detected on element:', target);
    console.log('🔍 Element data-id:', dataId);
    console.log('🔍 Element text content:', target.textContent);
    console.log('🔍 Context:', context);
    
    if (dataId) {
      e.stopPropagation();
      console.log('🔍 Calling highlightSameIds with:', dataId);
      highlightSameIds(dataId, context);
    } else {
      console.log('⚠️ No data-id found on clicked element');
    }
    
    // Call original onClick if provided
    if (onClick) {
      onClick(e);
    }
  };
  
  return (
    <span
      data-id={id}
      className={className}
      style={style}
      onClick={handleClick}
      {...otherProps}
    >
      {children}
    </span>
  );
};

// Higher-order component to wrap any component with highlighting
export const withHighlighting = <P extends object>(
  Component: React.ComponentType<P>
) => {
  return React.forwardRef<any, P & { context?: Partial<HighlightContext> }>(
    ({ context, ...props }, ref) => {
      const handleClick = (e: React.MouseEvent) => {
        const target = e.currentTarget as HTMLElement;
        const dataId = target.getAttribute('data-id');
        console.log('🔍 Click detected on wrapped element:', target);
        console.log('🔍 Element data-id:', dataId);
        console.log('🔍 Context:', context);
        
        if (dataId) {
          e.stopPropagation();
          console.log('🔍 Calling highlightSameIds with:', dataId);
          highlightSameIds(dataId, context);
        } else {
          console.log('⚠️ No data-id found on clicked element');
        }
      };
      
      return (
        <Component
          {...(props as P)}
          ref={ref}
          onClick={handleClick}
        />
      );
    }
  );
}; 