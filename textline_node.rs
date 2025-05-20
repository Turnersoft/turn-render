use super::math_node::MathNode;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

// --- Core Building Blocks for Rich Text ---
#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum TurnTextLineNode {
    Math(MathNode, String),
    Phrase(String),
    Empty,
    Comment(String),
    Latex(String),
    PageLink(String),
    Image(String),
}
