// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { AlertBoxStyle } from "./AlertBoxStyle";
import type { AnnotationOverlay } from "./AnnotationOverlay";
import type { CodeBlockNode } from "./CodeBlockNode";
import type { CollapsibleBlockNode } from "./CollapsibleBlockNode";
import type { ColumnsNode } from "./ColumnsNode";
import type { GridNode } from "./GridNode";
import type { ImageNode } from "./ImageNode";
import type { InteractiveControls } from "./InteractiveControls";
import type { InteractiveDiagramNode } from "./InteractiveDiagramNode";
import type { ListNode } from "./ListNode";
import type { MathDocument } from "./MathDocument";
import type { MathNode } from "./MathNode";
import type { PanelLayout } from "./PanelLayout";
import type { RichText } from "./RichText";
import type { Section } from "./Section";
import type { SideBySideLayout } from "./SideBySideLayout";
import type { StructuredMathNode } from "./StructuredMathNode";
import type { TableNode } from "./TableNode";
import type { ThematicBreakNode } from "./ThematicBreakNode";

/**
 * Enum representing the different types of content blocks that can appear in a section.
 * This is the primary building block for document content.
 */
export type SectionContentNode =
  | { "SubSection": Section }
  | { "RichText": RichText }
  | {
    "MathNode": {
      math: MathNode;
      label: string | null;
      caption: RichText | null;
    };
  }
  | { "StructuredMath": StructuredMathNode }
  | { "List": ListNode }
  | { "Table": TableNode }
  | { "CodeBlock": CodeBlockNode }
  | { "Image": ImageNode }
  | { "InteractiveDiagram": InteractiveDiagramNode }
  | { "CollapsibleBlock": CollapsibleBlockNode }
  | { "Grid": GridNode }
  | { "Columns": ColumnsNode }
  | { "ThematicBreak": ThematicBreakNode }
  | { "QuoteBlock": { content: Array<RichText>; attribution: RichText | null } }
  | { "AlertBox": { style: AlertBoxStyle; content: Array<SectionContentNode> } }
  | {
    "CustomComponent": {
      component_name: string;
      props: string | null;
      fallback_content: Array<SectionContentNode>;
    };
  }
  | { "EmbeddedSectionRef": string }
  | { "SideBySideLayout": SideBySideLayout }
  | { "PanelLayout": PanelLayout }
  | { "AnnotationOverlay": AnnotationOverlay }
  | { "InteractiveControls": InteractiveControls }
  | { "EmbeddedDocument": MathDocument };
