use animation::*;

/// Provides the enum type and name for a database enum value
pub struct DbEnumName(pub &'static str, pub &'static str);

///
/// Type of edit log item
/// 
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum EditLogType {
    SetSize,
    AddNewLayer,
    RemoveLayer,

    LayerAddKeyFrame,
    LayerRemoveKeyFrame,

    LayerPaintSelectBrush,
    LayerPaintBrushProperties,
    LayerPaintBrushStroke
}

///
/// Types of drawing style
/// 
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum DrawingStyleType {
    Draw,
    Erase
}

///
/// Types of brush definition
/// 
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum BrushDefinitionType {
    Simple,
    Ink
}

///
/// Types of colour definition
/// 
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ColorType {
    Rgb,
    Hsluv
}

///
/// Types of player
/// 
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum LayerType {
    Vector
}

///
/// Types of vector element
/// 
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum VectorElementType {
    BrushDefinition,
    BrushProperties,
    BrushStroke
}

///
/// All of the DB enums in one place
/// 
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum DbEnum {
    EditLog(EditLogType),
    DrawingStyle(DrawingStyleType),
    BrushDefinition(BrushDefinitionType),
    Color(ColorType),
    Layer(LayerType),
    VectorElement(VectorElementType)
}

impl<'a> From<&'a AnimationEdit> for EditLogType {
    fn from(t: &AnimationEdit) -> EditLogType {
        use self::AnimationEdit::*;
        use self::LayerEdit::*;
        use self::PaintEdit::*;

        match t {
            &SetSize(_, _)                          => EditLogType::SetSize,
            &AddNewLayer(_)                         => EditLogType::AddNewLayer,
            &RemoveLayer(_)                         => EditLogType::RemoveLayer,
            &Layer(_, AddKeyFrame(_))               => EditLogType::LayerAddKeyFrame,
            &Layer(_, RemoveKeyFrame(_))            => EditLogType::LayerRemoveKeyFrame,
            &Layer(_, Paint(_, SelectBrush(_, _)))  => EditLogType::LayerPaintSelectBrush,
            &Layer(_, Paint(_, BrushProperties(_))) => EditLogType::LayerPaintBrushProperties,
            &Layer(_, Paint(_, BrushStroke(_)))     => EditLogType::LayerPaintBrushStroke
        }
    }
}

impl<'a> From<&'a BrushDrawingStyle> for DrawingStyleType {
    fn from(t: &BrushDrawingStyle) -> DrawingStyleType {
        use self::BrushDrawingStyle::*;

        match t {
            &Draw   => DrawingStyleType::Draw,
            &Erase  => DrawingStyleType::Erase
        }
    }
}

impl<'a> From<&'a Vector> for VectorElementType {
    fn from(t: &Vector) -> VectorElementType {
        use self::Vector::*;

        match t {
            &BrushDefinition(_) => VectorElementType::BrushDefinition,
            &BrushProperties(_) => VectorElementType::BrushProperties,
            &BrushStroke(_)     => VectorElementType::BrushStroke
        }
    }
}
impl<'a> From<&'a BrushDefinition> for BrushDefinitionType {
    fn from(t: &BrushDefinition) -> BrushDefinitionType {
        use self::BrushDefinition::*;

        match t {
            &Simple     => BrushDefinitionType::Simple,
            &Ink(_)     => BrushDefinitionType::Ink
        }
    }
}


impl From<EditLogType> for DbEnumName {
    fn from(t: EditLogType) -> DbEnumName {
        use self::EditLogType::*;

        match t {
            SetSize                     => DbEnumName("Edit", "SetSize"),
            AddNewLayer                 => DbEnumName("Edit", "AddNewLayer"),
            RemoveLayer                 => DbEnumName("Edit", "RemoveLayer"),

            LayerAddKeyFrame            => DbEnumName("Edit", "Layer::AddKeyFrame"),
            LayerRemoveKeyFrame         => DbEnumName("Edit", "Layer::RemoveKeyFrame"),

            LayerPaintSelectBrush       => DbEnumName("Edit", "Layer::Paint::SelectBrush"),
            LayerPaintBrushProperties   => DbEnumName("Edit", "Layer::Paint::BrushProperties"),
            LayerPaintBrushStroke       => DbEnumName("Edit", "Layer::Paint::BrushStroke")
        }
    }
}

impl From<DrawingStyleType> for DbEnumName {
    fn from(t: DrawingStyleType) -> DbEnumName {
        use self::DrawingStyleType::*;

        match t {
            Draw    => DbEnumName("DrawingStyle", "Draw"),
            Erase   => DbEnumName("DrawingStyle", "Erase")
        }
    }
}

impl From<BrushDefinitionType> for DbEnumName {
    fn from(t: BrushDefinitionType) -> DbEnumName {
        use self::BrushDefinitionType::*;

        match t {
            Simple  => DbEnumName("BrushType", "Simple"),
            Ink     => DbEnumName("BrushType", "Ink")
        }
    }
}

impl From<ColorType> for DbEnumName {
    fn from(t: ColorType) -> DbEnumName {
        use self::ColorType::*;

        match t {
            Rgb     => DbEnumName("ColorType", "RGB"),
            Hsluv   => DbEnumName("ColorType", "HSLUV")
        }
    }
}

impl From<LayerType> for DbEnumName {
    fn from(t: LayerType) -> DbEnumName {
        use self::LayerType::*;

        match t {
            Vector  => DbEnumName("LayerType", "Vector")
        }
    }
}

impl From<VectorElementType> for DbEnumName {
    fn from(t: VectorElementType) -> DbEnumName {
        use self::VectorElementType::*;

        match t {
            BrushDefinition => DbEnumName("VectorElementType", "BrushDefinition"),
            BrushProperties => DbEnumName("VectorElementType", "BrushProperties"),
            BrushStroke     => DbEnumName("VectorElementType", "BrushStroke")
        }
    }
}

impl From<DbEnum> for DbEnumName {
    fn from(t: DbEnum) -> DbEnumName {
        use self::DbEnum::*;

        match t {
            EditLog(elt)            => DbEnumName::from(elt),
            DrawingStyle(dst)       => DbEnumName::from(dst),
            BrushDefinition(bdt)    => DbEnumName::from(bdt),
            Color(ct)               => DbEnumName::from(ct),
            Layer(lt)               => DbEnumName::from(lt),
            VectorElement(vet)      => DbEnumName::from(vet)
        }
    }
}