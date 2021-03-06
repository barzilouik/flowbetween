use super::empty_frame::*;
use super::vector_frame::*;
use super::vector_layer_core::*;
use super::super::traits::*;

use std::sync::*;
use std::time::Duration;
use std::ops::{Range, Deref};

///
/// Represents a vector layer. Vector layers support brush and vector objects.
/// 
pub struct InMemoryVectorLayer {
    /// The core data for this layer
    core: Mutex<VectorLayerCore>
}

impl InMemoryVectorLayer {
    ///
    /// Cretes a new vector layer
    /// 
    pub fn new(id: u64) -> InMemoryVectorLayer {
        let core = VectorLayerCore::new(id);

        InMemoryVectorLayer { 
            core:       Mutex::new(core)
        }
    }

    ///
    /// Edits this layer
    /// 
    #[inline]
    pub fn edit(&self, edit: &LayerEdit) {
        self.core.lock().unwrap().edit(edit);
    }

    ///
    /// Performs an edit on an element contained within this animation
    /// 
    pub fn edit_element(&self, element_id: ElementId, when: Duration, edit: &ElementEdit) {
        // TODO
    }
}

impl Layer for InMemoryVectorLayer {
    fn id(&self) -> u64 {
        self.core.lock().unwrap().id()
    }

    fn get_frame_at_time(&self, time_index: Duration) -> Arc<dyn Frame> {
        let core = self.core.lock().unwrap();

        // TODO: apply any motions attached to the elements

        // Look up the keyframe in the core
        let keyframe = core.find_nearest_keyframe(time_index);
        if let Some(keyframe) = keyframe {
            // Found a keyframe: return a vector frame from it
            Arc::new(VectorFrame::new(keyframe.clone(), time_index - keyframe.start_time()))
        } else {
            // No keyframe at this point in time
            Arc::new(EmptyFrame::new(time_index))
        }
    }

    fn get_key_frames_during_time(&self, when: Range<Duration>) -> Box<dyn Iterator<Item=Duration>> {
        let core = self.core.lock().unwrap();

        let result: Vec<_> = core.keyframes()
            .map(|frame| frame.start_time())
            .filter(|time| &when.start <= time && &when.end >= time)
            .collect();

        Box::new(result.into_iter())
    }

    fn supported_edit_types(&self) -> Vec<LayerEditType> {
        return vec![
            LayerEditType::Vector
        ];
    }

    fn as_vector_layer<'a>(&'a self) -> Option<Box<dyn 'a+Deref<Target=dyn 'a+VectorLayer>>> {
        let core: &Mutex<dyn VectorLayer> = &self.core;

        Some(Box::new(core.lock().unwrap()))
    }
}
