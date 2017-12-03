use std::time::Duration;

///
/// Represents a keuyframe in a vector animation
/// 
pub struct VectorKeyFrame {
    /// When this frame starts
    start_time: Duration
}

impl VectorKeyFrame {
    ///
    /// Creates a new vector key frame
    /// 
    pub fn new(start_time: Duration) -> VectorKeyFrame {
        VectorKeyFrame {
            start_time: start_time
        }
    }
}