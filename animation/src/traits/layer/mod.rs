mod vector;

pub use self::vector::*;

use super::edit::*;
use super::frame::*;

use std::u32;
use std::sync::*;
use std::time::Duration;
use std::ops::{Range, Deref};

///
/// A layer represents a renderable plane in an animation
///
pub trait Layer : 
    Send {
    ///
    /// The ID associated with this layer
    /// 
    fn id(&self) -> u64;

    ///
    /// The types of edit that are supported by this layer
    /// 
    fn supported_edit_types(&self) -> Vec<LayerEditType>;

    ///
    /// Retrieves a frame from this layer with the specified parameters
    ///
    fn get_frame_at_time(&self, time_index: Duration) -> Arc<dyn Frame>;

    ///
    /// Retrieves the times where key frames exist
    ///
    fn get_key_frames(&self) -> Box<dyn Iterator<Item=Duration>> { self.get_key_frames_during_time(Duration::from_millis(0)..Duration::from_secs(u32::MAX as u64)) }

    ///
    /// Retrieves the times where key frames exist during a specified time range
    /// 
    fn get_key_frames_during_time(&self, when: Range<Duration>) -> Box<dyn Iterator<Item=Duration>>;

    ///
    /// Retrieves the definition of this layer as a vector layer
    /// 
    fn as_vector_layer<'a>(&'a self) -> Option<Box<dyn 'a+Deref<Target=dyn 'a+VectorLayer>>>;
}
