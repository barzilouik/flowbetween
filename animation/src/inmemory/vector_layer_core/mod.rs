mod keyframes;
mod paint;

use super::super::traits::*;
use super::vector_keyframe::*;

use ui::canvas::*;

use std::mem;
use std::time::Duration;

///
/// The core of the vector layer
/// 
pub struct VectorLayerCore {
    // The ID assigned to this layer
    id: u64,

    /// The key frames for this vector, in order
    keyframes: Vec<VectorKeyFrame>,

    /// The brush stroke that is currently being drawn
    active_brush_stroke: Option<BrushElement>
}

impl VectorLayerCore {
    ///
    /// Creates a new vector layer core
    /// 
    pub fn new(id: u64) -> VectorLayerCore {
        VectorLayerCore {
            id:                     id,
            keyframes:              vec![],
            active_brush_stroke:    None
        }
    }

    ///
    /// Returns the ID for this layer
    /// 
    pub fn id(&self) -> u64 {
        self.id
    }

    ///
    /// Sorts the keyframes in order
    /// 
    fn sort_key_frames(&mut self) {
        self.keyframes.sort_by(|a, b| a.start_time().cmp(&b.start_time()));
    }

    fn find_nearest_keyframe<'a>(&'a mut self, time: Duration) -> Option<&'a mut VectorKeyFrame> {
        // Binary search for the key frame
        let search_result = self.keyframes.binary_search_by(|a| a.start_time().cmp(&time));

        match search_result {
            Ok(exact_frame)         => Some(&mut self.keyframes[exact_frame]),
            Err(following_frame)    => if following_frame == 0 {
                None
            } else {
                Some(&mut self.keyframes[following_frame-1])
            }
        }
    }
}
