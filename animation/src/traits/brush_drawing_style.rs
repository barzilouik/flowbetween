///
/// Represents the drawing style to use with a brush
/// 
#[derive(Clone, Copy)]
pub enum BrushDrawingStyle {
    /// Draw this brush directly on to the current layer
    Draw,

    /// Erase what's underneath this brush on the current layer
    Erase
}
