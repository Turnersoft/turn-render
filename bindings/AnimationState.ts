// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.

/**
 * A single state in an animation sequence
 */
export type AnimationState = {
  /**
   * Time position in the animation (e.g., 0.0 to 1.0)
   */
  time_position: number;
  /**
   * Description of what's happening at this state
   */
  description: string;
  /**
   * State data (JSON string, format depends on animation_type)
   */
  state_data: string;
  /**
   * Whether this is a key state that should pause for explanation
   */
  is_key_state: boolean;
};
