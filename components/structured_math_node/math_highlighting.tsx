// Math highlighting system for interactive proof visualization
import React from 'react';

// Highlighting modes
export enum HighlightMode {
  WITHIN_PROOF_NODE = 'within_proof_node',
  ENTIRE_DOCUMENT = 'entire_document',
  CURRENT_AND_PREVIOUS = 'current_and_previous'
}

// Context for highlighting
export interface HighlightContext {
  proofNodeId?: string;
  sectionId?: string;
  mode: HighlightMode;
}

// Global highlighting system
let highlightedId: string | null = null;
let currentHighlightMode: HighlightMode = HighlightMode.WITHIN_PROOF_NODE;

let currentContext: HighlightContext = {
  mode: HighlightMode.WITHIN_PROOF_NODE
};

let setHighlightMode = (mode: HighlightMode) => {
  currentHighlightMode = mode;
  currentContext.mode = mode;
  console.log(`🎯 Highlight mode changed to: ${mode}`);
};

const setHighlightContext = (context: Partial<HighlightContext>) => {
  currentContext = { ...currentContext, ...context };
  console.log('📍 Highlight context updated:', currentContext);
};

export const highlightSameIds = (id: string, context?: Partial<HighlightContext>) => {
  console.log('🎯 highlightSameIds called with id:', id, 'context:', context);
  
  // Update context if provided
  if (context) {
    setHighlightContext(context);
  }
  
  // Remove previous highlights
  const previousHighlights = document.querySelectorAll('.math-highlight');
  console.log('🧹 Removing', previousHighlights.length, 'previous highlights');
  previousHighlights.forEach(el => {
    el.classList.remove('math-highlight');
  });
  
  let elements: NodeListOf<Element>;
  
  console.log('🎯 Current context mode:', currentContext.mode);
  console.log('🎯 Current context:', currentContext);
  
  switch (currentContext.mode) {
    case HighlightMode.WITHIN_PROOF_NODE:
      // Highlight only within the current proof node
      if (currentContext.proofNodeId) {
        const proofNode = document.querySelector(`[data-node-id="${currentContext.proofNodeId}"]`);
        console.log('🔍 Looking for proof node with data-node-id:', currentContext.proofNodeId);
        console.log('🔍 Found proof node:', proofNode);
        if (proofNode) {
          elements = proofNode.querySelectorAll(`[data-id="${id}"]`);
          console.log('🔍 Found', elements.length, 'elements within proof node');
        } else {
          elements = document.querySelectorAll(`[data-id="${id}"]`);
          console.log('🔍 Proof node not found, searching entire document');
        }
      } else {
        elements = document.querySelectorAll(`[data-id="${id}"]`);
        console.log('🔍 No proof node ID, searching entire document');
      }
      break;
      
    case HighlightMode.CURRENT_AND_PREVIOUS:
      // Highlight in current and all previous proof nodes
      const allProofNodes = document.querySelectorAll('[data-node-id]');
      console.log('🔍 Found', allProofNodes.length, 'proof nodes total');
      const currentIndex = Array.from(allProofNodes).findIndex(node => 
        node.getAttribute('data-node-id') === currentContext.proofNodeId
      );
      console.log('🔍 Current proof node index:', currentIndex);
      
      const relevantNodes = Array.from(allProofNodes).slice(0, currentIndex + 1);
      elements = document.querySelectorAll(`[data-id="${id}"]`);
      // Filter to only elements within relevant nodes
      const relevantElements = Array.from(elements).filter(el => 
        relevantNodes.some(node => node.contains(el))
      );
      elements = relevantElements as any;
      console.log('🔍 Found', elements.length, 'elements in current and previous nodes');
      break;
      
    case HighlightMode.ENTIRE_DOCUMENT:
    default:
      // Highlight across entire document
      elements = document.querySelectorAll(`[data-id="${id}"]`);
      console.log('🔍 Searching entire document for data-id:', id);
      break;
  }
  
  console.log('🔍 Total elements found:', elements.length);
  
  // Apply highlights
  elements.forEach((el, index) => {
    el.classList.add('math-highlight');
    console.log(`🎨 Highlighted element ${index + 1}:`, el);
  });
  
  highlightedId = id;
  
  // Show detailed information in console
  console.log(`🔍 Highlighted ${elements.length} elements with data-id: "${id}" in mode: ${currentContext.mode}`);
  console.log('📍 Elements found:', Array.from(elements).map(el => ({
    tagName: el.tagName,
    textContent: el.textContent?.trim().substring(0, 50),
    className: el.className,
    parentId: el.parentElement?.getAttribute('data-id') || 'none',
    proofNodeId: el.closest('[data-node-id]')?.getAttribute('data-node-id') || 'none'
  })));
  
  // Update visual indicator
  const indicator = document.getElementById('math-highlight-indicator') || createHighlightIndicator();
  indicator.textContent = `Highlighted: ${id} (${elements.length} elements) - Mode: ${currentContext.mode}`;
  indicator.style.display = 'block';
};

const createHighlightIndicator = () => {
  const indicator = document.createElement('div');
  indicator.id = 'math-highlight-indicator';
  indicator.style.cssText = `
    position: fixed;
    top: 10px;
    right: 10px;
    background: #ffeb3b;
    color: #000;
    padding: 8px 12px;
    border-radius: 4px;
    font-family: monospace;
    font-size: 12px;
    z-index: 10000;
    box-shadow: 0 2px 8px rgba(0,0,0,0.2);
    display: none;
    max-width: 300px;
    word-wrap: break-word;
  `;
  document.body.appendChild(indicator);
  return indicator;
};

export const clearHighlights = () => {
  document.querySelectorAll('.math-highlight').forEach(el => {
    el.classList.remove('math-highlight');
  });
  highlightedId = null;
  
  // Hide the indicator
  const indicator = document.getElementById('math-highlight-indicator');
  if (indicator) {
    indicator.style.display = 'none';
  }
};

// Add global event listeners for highlighting
if (typeof window !== 'undefined') {
  // Debug: Check for data-id attributes on page load
  setTimeout(() => {
    const allDataIds = document.querySelectorAll('[data-id]');
    console.log('🔍 Found', allDataIds.length, 'elements with data-id attributes on page load');
    console.log('🔍 Sample data-ids:', Array.from(allDataIds).slice(0, 10).map(el => ({
      id: el.getAttribute('data-id'),
      text: el.textContent?.trim().substring(0, 20),
      tagName: el.tagName
    })));
  }, 2000);
  
  // Clear highlights when clicking outside
  document.addEventListener('click', (e) => {
    const target = e.target as HTMLElement;
    if (!target.closest('[data-id]')) {
      clearHighlights();
    }
  });
  
  // Add keyboard shortcuts for mode switching
  document.addEventListener('keydown', (e) => {
    if (e.ctrlKey || e.metaKey) {
      switch (e.key) {
        case '1':
          e.preventDefault();
          setHighlightMode(HighlightMode.WITHIN_PROOF_NODE);
          break;
        case '2':
          e.preventDefault();
          setHighlightMode(HighlightMode.ENTIRE_DOCUMENT);
          break;
        case '3':
          e.preventDefault();
          setHighlightMode(HighlightMode.CURRENT_AND_PREVIOUS);
          break;
      }
    }
  });
  
  // Create mode switcher UI
  const createModeSwitcher = () => {
    const switcher = document.getElementById('highlight-mode-switcher');
    if (switcher) return switcher;
    
    const div = document.createElement('div');
    div.id = 'highlight-mode-switcher';
    div.style.cssText = `
      position: fixed;
      top: 10px;
      left: 10px;
      background: #fff;
      border: 1px solid #ccc;
      border-radius: 4px;
      padding: 8px;
      font-family: monospace;
      font-size: 12px;
      z-index: 10000;
      box-shadow: 0 2px 8px rgba(0,0,0,0.2);
    `;
    
    div.innerHTML = `
      <div style="margin-bottom: 4px; font-weight: bold;">Highlight Mode:</div>
      <div style="margin-bottom: 2px;">
        <button id="mode-1" style="margin-right: 4px; padding: 2px 6px; font-size: 10px;">1: Within Node</button>
        <button id="mode-2" style="margin-right: 4px; padding: 2px 6px; font-size: 10px;">2: Entire Doc</button>
        <button id="mode-3" style="padding: 2px 6px; font-size: 10px;">3: Current+Prev</button>
      </div>
      <div id="current-mode" style="font-size: 10px; color: #666;">Current: Within Node</div>
    `;
    
    document.body.appendChild(div);
    
    // Add event listeners
    div.querySelector('#mode-1')?.addEventListener('click', () => setHighlightMode(HighlightMode.WITHIN_PROOF_NODE));
    div.querySelector('#mode-2')?.addEventListener('click', () => setHighlightMode(HighlightMode.ENTIRE_DOCUMENT));
    div.querySelector('#mode-3')?.addEventListener('click', () => setHighlightMode(HighlightMode.CURRENT_AND_PREVIOUS));
    
    return div;
  };
  
  // Create the mode switcher when the page loads
  setTimeout(createModeSwitcher, 1000);
  
  // Update the current mode display
  const updateModeDisplay = () => {
    const display = document.getElementById('current-mode');
    if (display) {
      const modeNames = {
        [HighlightMode.WITHIN_PROOF_NODE]: 'Within Node',
        [HighlightMode.ENTIRE_DOCUMENT]: 'Entire Doc',
        [HighlightMode.CURRENT_AND_PREVIOUS]: 'Current+Prev'
      };
      display.textContent = `Current: ${modeNames[currentHighlightMode]}`;
    }
  };
  
  // Override setHighlightMode to update display
  const originalSetHighlightMode = setHighlightMode;
  setHighlightMode = (mode: HighlightMode) => {
    originalSetHighlightMode(mode);
    updateModeDisplay();
  };
}

// Export highlighting functions for use in other components
export { setHighlightMode, setHighlightContext }; 