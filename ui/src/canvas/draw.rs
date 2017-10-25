//!
//! Actions that can be performed to draw on a canvas
//!

use super::transform2d::*;
use super::color::*;

///
/// Possible way to join lines 
///
pub enum LineJoin {
    Miter,
    Round,
    Bevel
}

///
/// How to cap lines
///
pub enum LineCap {
    Butt,
    Round,
    Square
}

///
/// Blend mode to use when drawing
/// 
pub enum BlendMode {
    SourceOver,
    SourceIn,
    SourceOut,
    DestinationOver,
    DestinationIn,
    DestinationOut,
    SourceAtop,
    DestinationAtop,

    Multiply,
    Screen,
    Darken,
    Lighten
}

///
/// Instructions for drawing to a canvas
///
pub enum Draw {
    /// Begins a new path
    NewPath,

    /// Move to a new point
    Move(f32, f32),

    /// Line to point
    Line(f32, f32),

    /// Bezier curve to point
    BezierCurve((f32, f32), (f32, f32), (f32, f32)),

    /// Rectangle path between points
    Rect((f32, f32), (f32, f32)),

    /// Fills the current clipping region with a single colour
    Blit(Color),

    /// Fill the current path
    Fill,

    /// Draw a line around the current path
    Stroke,

    /// Set the line width
    LineWidth(f32),

    /// Line join
    LineJoin(LineJoin),

    /// The cap to use on lines
    LineCap(LineCap),

    /// Sets the dash pattern
    DashLengths(Vec<f32>),

    /// Sets the offset for the dash pattern
    DashOffset(f32),

    /// Set the fill color
    FillColor(Color),

    /// Set the line color
    StrokeColor(Color),

    /// Set how future renderings are blended with one another
    BlendMode(BlendMode),

    /// Reset the transformation to the identity transformation
    IdentityTransform,

    /// Sets a transformation such that:
    /// (0,0) is the center point of the canvas
    /// (0,height/2) is the bottom of the canvas
    /// Pixels are square
    CanvasHeight(f32),

    /// Multiply a 2D transform into the canvas
    MultiplyTransform(Transform2D),

    /// Unset the clipping path
    Unclip,

    /// Clip to the currently set path
    Clip,

    /// Push the current state of the canvas (line settings, stored image, current path - all state)
    PushState,

    /// Restore a state previously pushed
    PopState,

    /// Stores the content of the clipping path in a background buffer
    Store,

    /// Restores what was stored in the background buffer
    /// (If the clipping path has changed since then, the restored image is clipped against the new path)
    Restore,

    /// Clears the canvas entirely
    ClearCanvas
}