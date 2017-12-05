use super::super::traits::*;

use ui::canvas::*;

use std::time::Duration;

///
/// A frame with nothing in it
/// 
pub struct EmptyFrame {
    time_index: Duration
}

impl EmptyFrame {
    ///
    /// Creates a new empty frame at a particular time index
    ///
    pub fn new(time_index: Duration) -> EmptyFrame {
        EmptyFrame {
            time_index: time_index
        }
    }
}

impl Frame for EmptyFrame {
    fn time_index(&self) -> Duration {
        self.time_index
    }

    fn render_to(&self, gc: &mut GraphicsContext) {
    }
}