// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { MathNode } from "./MathNode";
import type { RichText } from "./RichText";
import type { SectionContentNode } from "./SectionContentNode";

/**
 * Display representation of case analysis
 */
export type CaseDisplayNode = {
  case_name: RichText;
  condition: MathNode;
  values: Array<MathNode>;
  case_explanation: RichText;
  proof_outline: Array<SectionContentNode> | null;
};
