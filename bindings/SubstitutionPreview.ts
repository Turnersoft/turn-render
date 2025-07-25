// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { MathNode } from "./MathNode";
import type { SubstitutionHighlight } from "./SubstitutionHighlight";

/**
 * Display representation of substitution preview
 */
export type SubstitutionPreview = {
  before: MathNode;
  after: MathNode;
  highlighted_changes: Array<SubstitutionHighlight>;
};
