use animation::*;

///
/// Action that updates the brush preview
/// 
#[derive(Debug)]
pub enum BrushPreviewAction {
    /// Clears any existing brush preview
    Clear,

    /// Specifies the layer whose brush preview is being edited
    Layer(u64),

    /// Sets the brush definition to use for the brush preview
    BrushDefinition(BrushDefinition, BrushDrawingStyle),

    /// Sets the brush properties to use for the brush preview
    BrushProperties(BrushProperties),

    /// Adds a raw point to the brush preview
    AddPoint(RawPoint),

    /// Commits the brush preview to the current layer
    Commit
}
