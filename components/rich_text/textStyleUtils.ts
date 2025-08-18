import React from 'react';

// Convert TextStyle enum values to CSS properties
// This function handles the TextStyle variants from rich_text.rs
export const convertTextStylesToCSS = (styles: any[]): React.CSSProperties => {
  const style: React.CSSProperties = {};
  
  if (!styles || !Array.isArray(styles)) {
    return style;
  }
  
  styles.forEach(styleType => {
    if (styleType === 'Bold') {
      style.fontWeight = 'bold';
    } else if (styleType === 'Italic') {
      style.fontStyle = 'italic';
    } else if (styleType === 'Underline') {
      style.textDecoration = 'underline';
    } else if (styleType === 'Strikethrough') {
      style.textDecoration = 'line-through';
    } else if (styleType === 'Subscript') {
      style.verticalAlign = 'sub';
      style.fontSize = '0.8em';
    } else if (styleType === 'Superscript') {
      style.verticalAlign = 'super';
      style.fontSize = '0.8em';
    } else if (typeof styleType === 'object') {
      // Handle object-based styles
      if ('Color' in styleType) {
        style.color = styleType.Color;
      } else if ('BackgroundColor' in styleType) {
        style.backgroundColor = styleType.BackgroundColor;
      } else if ('FontSize' in styleType) {
        style.fontSize = styleType.FontSize;
      } else if ('FontFamily' in styleType) {
        style.fontFamily = styleType.FontFamily;
      }
    }
  });
  
  return style;
}; 
