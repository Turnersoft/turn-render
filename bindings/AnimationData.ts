// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { AnimationState } from "./AnimationState";

/**
 * Data for animations to visualize mathematical concepts
 */
export type AnimationData = {
  /**
   * Type of animation (e.g., "2d-graph", "3d-model", "state-transition")
   */
  animation_type: string;
  /**
   * Initial state for the animation (JSON string)
   */
  initial_state: string;
  /**
   * Sequence of animation states
   */
  states: Array<AnimationState>;
  /**
   * Additional settings for the animation renderer (JSON string)
   */
  settings: string | null;
  /**
   * Description of what the animation shows
   */
  description: string | null;
};
