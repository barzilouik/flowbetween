use super::frame_edit::*;

use std::time::Duration;

///
/// Represents a type of layer edit
/// 
/// Layers may have different types, so this can be used to check what
/// types of action a particular layer might support.
/// 

#[derive(Clone, PartialEq, Debug)]
pub enum LayerEditType {
    Vector
}

///
/// Represents an edit to a layer
///
#[derive(Clone, PartialEq, Debug)]
pub enum LayerEdit {
    /// Edit to a frame at a specific time
    Paint(Duration, PaintEdit),

    /// Adds a keyframe at a particular point in time
    /// 
    /// Edits don't have to correspond to a keyframe - instead, keyframes
    /// indicate where the layer is cleared.
    AddKeyFrame(Duration),

    /// Removes a keyframe previously added at a particular duration
    RemoveKeyFrame(Duration)
}

impl LayerEdit {
    ///
    /// If this edit contains an unassigned element ID, calls the specified function to supply a new
    /// element ID. If the edit already has an ID, leaves it unchanged.
    /// 
    pub fn assign_element_id<AssignFn: FnOnce() -> i64>(self, assign_element_id: AssignFn) -> LayerEdit {
        use self::LayerEdit::*;

        match self {
            Paint(when, paint_edit) => Paint(when, paint_edit.assign_element_id(assign_element_id)),
            other                   => other
        }
    }
}