use super::frame::*;

use std::sync::*;
use std::time::Duration;

///
/// A layer represents a renderable plane in an animation
///
pub trait Layer {
    ///
    /// The ID associated with this layer
    /// 
    fn id(&self) -> u64;

    ///
    /// Retrieves a frame from this layer with the specified parameters
    ///
    fn get_frame_at_time<'a>(&self, time_index: Duration) -> &'a Frame;

    ///
    /// Retrieves the key frames in this layer
    ///
    fn get_key_frames<'a>(&'a self) -> Box<'a+Iterator<Item = &'a Frame>>;
}
